#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFacetGrouptype {
    GuessFromGroup = 0,
    Points = 1,
    Primitives = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFacetCons {
    NoConsolidate = 0,
    ConsolidatePointsSlow = 1,
    ConsolidatePointsFast = 2,
    ConsolidateNormalsSlow = 3,
    ConsolidateNormalsFast = 4,
}

#[derive(Debug, Clone)]
pub struct SopFacet {
    pub base: crate::core::types::NodeBase,
}

impl SopFacet {
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
    pub fn with_inlinedist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "inlinedist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inlinedist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inlinedist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopFacetGrouptype) -> Self {
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
    pub fn with_cons(mut self, val: SopFacetCons) -> Self {
        self.base.params.insert(
            "cons".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cons_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cons".to_string(),
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
    pub fn with_prenml(mut self, val: bool) -> Self {
        self.base.params.insert(
            "prenml".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prenml_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prenml".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unit(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unique(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unique".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unique_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unique".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_accurate(mut self, val: bool) -> Self {
        self.base.params.insert(
            "accurate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_accurate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "accurate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inline(mut self, val: bool) -> Self {
        self.base.params.insert(
            "inline".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inline_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inline".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientpolys(mut self, val: bool) -> Self {
        self.base.params.insert(
            "orientPolys".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_orientpolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orientPolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cusp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "cusp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cusp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cusp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_mkplanar(mut self, val: bool) -> Self {
        self.base.params.insert(
            "mkplanar".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkplanar_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mkplanar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postnml(mut self, val: bool) -> Self {
        self.base.params.insert(
            "postnml".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_postnml_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "postnml".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reversenml(mut self, val: bool) -> Self {
        self.base.params.insert(
            "reversenml".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reversenml_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reversenml".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFacet {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "facet"
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
pub enum SopFalloffOutputtype {
    Distance = 0,
    NormalizedDistance = 1,
    UnboundedDistance = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFalloffDistmetric {
    Edge = 0,
    Radius = 1,
    Surface = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFalloffRamppreset {
    Constant = 0,
    Custom = 1,
    Hill = 2,
    Linear = 3,
    Round = 4,
    Sharp = 5,
    Smooth = 6,
    Steps = 7,
    Valley = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFalloffMix {
    Maximum = 0,
    Minimum = 1,
    None = 2,
    Overwrite = 3,
}

#[derive(Debug, Clone)]
pub struct SopFalloff {
    pub base: crate::core::types::NodeBase,
}

impl SopFalloff {
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

    /// Connects to input 0: "Geometry to Measure"
    pub fn set_input_geometry_to_measure(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Measure" and specifies the output index of the target node.
    pub fn set_input_geometry_to_measure_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_rad(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rad".to_string(),
            crate::core::types::ParamValue::Float(val),
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

    // --- Menu parameters ---
    pub fn with_outputtype(mut self, val: SopFalloffOutputtype) -> Self {
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
    pub fn with_distmetric(mut self, val: SopFalloffDistmetric) -> Self {
        self.base.params.insert(
            "distmetric".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_distmetric_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "distmetric".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ramppreset(mut self, val: SopFalloffRamppreset) -> Self {
        self.base.params.insert(
            "ramppreset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_ramppreset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ramppreset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mix(mut self, val: SopFalloffMix) -> Self {
        self.base.params.insert(
            "mix".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mix_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "ramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_ramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ramp".to_string(),
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
    pub fn with_distattr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "distattr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_distattr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "distattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_leadptattr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "leadptattr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_leadptattr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "leadptattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "falloffgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_falloffgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_data_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("vis_data{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vis_data_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("vis_data{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_reverse(mut self, val: bool) -> Self {
        self.base.params.insert(
            "reverse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reverse_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reverse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputdist(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputdist".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputdist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputleadpt(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputleadpt".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputleadpt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputleadpt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_node_vis_enabled(mut self, val: bool) -> Self {
        self.base.params.insert(
            "node_vis_enabled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_node_vis_enabled_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "node_vis_enabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_active_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("vis_active{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vis_active_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("vis_active{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFalloff {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "falloff"
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
pub enum SopFasciaShowviz {
    CavityMask = 0,
    Surface = 1,
    TetrahedralMesh = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFasciaSizing {
    Uniform = 0,
    Adaptive = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFasciaTposerefswitch {
    UseInitializationFrame = 0,
    FromAttribute = 1,
}

#[derive(Debug, Clone)]
pub struct SopFascia {
    pub base: crate::core::types::NodeBase,
}

impl SopFascia {
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

    /// Connects to input 0: "Collision Geometry"
    pub fn set_input_collision_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Collision Geometry" and specifies the output index of the target node.
    pub fn set_input_collision_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Outer Skin"
    pub fn set_input_outer_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Outer Skin" and specifies the output index of the target node.
    pub fn set_input_outer_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_initialoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "initialoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_initialoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initialoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "targetsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetsize".to_string(),
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
    pub fn with_cavityoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "cavityoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cavityoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cavityoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cavitybias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "cavitybias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cavitybias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cavitybias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cavityraydist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "cavityraydist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cavityraydist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cavityraydist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsurfacedepth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "subsurfacedepth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_subsurfacedepth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "subsurfacedepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxtetsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxtetsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxtetsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxtetsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

    // --- Int parameters ---
    pub fn with_showviz(mut self, val: SopFasciaShowviz) -> Self {
        self.base.params.insert(
            "showviz".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_showviz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showviz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsurfacelayers(mut self, val: i32) -> Self {
        self.base.params.insert(
            "subsurfacelayers".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_subsurfacelayers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "subsurfacelayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tposerefswitch(mut self, val: SopFasciaTposerefswitch) -> Self {
        self.base.params.insert(
            "tposerefswitch".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_tposerefswitch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tposerefswitch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sizing(mut self, val: SopFasciaSizing) -> Self {
        self.base.params.insert(
            "sizing".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sizing_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sizing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_tposerefattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "tposerefattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tposerefattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tposerefattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enableskinclip(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableskinclip".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableskinclip_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableskinclip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removecavities(mut self, val: bool) -> Self {
        self.base.params.insert(
            "removecavities".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removecavities_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "removecavities".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expandcavities(mut self, val: bool) -> Self {
        self.base.params.insert(
            "expandcavities".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_expandcavities_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "expandcavities".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFascia {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "fascia"
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
pub enum SopFasciasolidifyLocalscaling {
    None = 0,
    UseConstant = 1,
    UseLocalFeatureSize = 2,
    UsePointAttribute = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFasciasolidifyCompresssims {
    NoCompression = 0,
    Blosc = 1,
    Gzip = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFasciasolidifyTposeswitch {
    UseInitializationFrame = 0,
    FromAttribute = 1,
    FromFile = 2,
    FromOtherGeometry = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFasciasolidifyBonetposeswitch {
    UseInitializationFrame = 0,
    FromAttribute = 1,
    FromFile = 2,
    FromOtherGeometry = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFasciasolidifyFasciatrange {
    SaveCurrentFrame = 0,
    SaveFrameRange = 1,
    /// Save Frame Range Only (Strict)
    SaveFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone)]
pub struct SopFasciasolidify {
    pub base: crate::core::types::NodeBase,
}

impl SopFasciasolidify {
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

    /// Connects to input 0: "Fascia Source"
    pub fn set_input_fascia_source(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Fascia Source" and specifies the output index of the target node.
    pub fn set_input_fascia_source_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Bones"
    pub fn set_input_bones(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Bones" and specifies the output index of the target node.
    pub fn set_input_bones_from(
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
    pub fn trigger_tposereload(mut self) -> Self {
        self.base.params.insert(
            "tposereload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_bonetposereload(mut self) -> Self {
        self.base.params.insert(
            "bonetposereload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_fasciareload(mut self) -> Self {
        self.base.params.insert(
            "fasciareload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_fasciaexecute(mut self) -> Self {
        self.base.params.insert(
            "fasciaexecute".to_string(),
            crate::core::types::ParamValue::Button,
        );
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
    pub fn with_fasciasurftriscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fasciasurftriscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fasciasurftriscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fasciasurftriscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fasciamaxtetscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fasciamaxtetscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fasciamaxtetscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fasciamaxtetscale".to_string(),
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
    pub fn with_fasciaenlargeoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fasciaenlargeoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fasciaenlargeoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fasciaenlargeoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_effectivestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "effectivestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effectivestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "effectivestiffness".to_string(),
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
    pub fn with_attachmultiplier(mut self, val: f32) -> Self {
        self.base.params.insert(
            "attachmultiplier".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attachmultiplier_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attachmultiplier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_fasciaf(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "fasciaf".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_fasciaf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fasciaf".to_string(),
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
    pub fn with_tposeswitch(mut self, val: SopFasciasolidifyTposeswitch) -> Self {
        self.base.params.insert(
            "tposeswitch".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_tposeswitch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tposeswitch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tposeframe(mut self, val: i32) -> Self {
        self.base.params.insert(
            "tposeframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tposeframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tposeframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonetposeswitch(mut self, val: SopFasciasolidifyBonetposeswitch) -> Self {
        self.base.params.insert(
            "bonetposeswitch".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_bonetposeswitch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonetposeswitch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonetposeframe(mut self, val: i32) -> Self {
        self.base.params.insert(
            "bonetposeframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bonetposeframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonetposeframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_localscaling(mut self, val: SopFasciasolidifyLocalscaling) -> Self {
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
    pub fn with_compresssims(mut self, val: SopFasciasolidifyCompresssims) -> Self {
        self.base.params.insert(
            "compresssims".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_compresssims_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "compresssims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fasciatrange(mut self, val: SopFasciasolidifyFasciatrange) -> Self {
        self.base.params.insert(
            "fasciatrange".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fasciatrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fasciatrange".to_string(),
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
    pub fn with_fasciaboneattachattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fasciaboneattachattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fasciaboneattachattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fasciaboneattachattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonefasciaattachattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bonefasciaattachattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bonefasciaattachattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonefasciaattachattribute".to_string(),
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
    pub fn with_tposegeofile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "tposegeofile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tposegeofile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tposegeofile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tposepath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "tposepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tposepath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tposepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tposegroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "tposegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tposegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tposegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonetposeattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bonetposeattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bonetposeattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonetposeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonetposegeofile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bonetposegeofile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bonetposegeofile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonetposegeofile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonetposepath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bonetposepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bonetposepath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonetposepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonetposegroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bonetposegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bonetposegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonetposegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fasciafile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fasciafile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fasciafile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fasciafile".to_string(),
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
    pub fn with_fasciaenlarge(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fasciaenlarge".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fasciaenlarge_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fasciaenlarge".to_string(),
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
    pub fn with_fascialoadfromdisk(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fascialoadfromdisk".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fascialoadfromdisk_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fascialoadfromdisk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFasciasolidify {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "fasciasolidify"
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
pub enum SopFasciasolverSolvertype {
    Fem = 0,
    Vellum = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFasciasolverFemtristiffnessexp {
    /// 1e+10
    N1ePlus10 = 0,
    /// 1e+9
    N1ePlus9 = 1,
    /// 1e+8
    N1ePlus8 = 2,
    /// 1e+7
    N1ePlus7 = 3,
    /// 1 000 000
    N1000000 = 4,
    /// 100 000
    N100000 = 5,
    /// 10 000
    N10000 = 6,
    /// 1 000
    N1000 = 7,
    /// 100
    N100 = 8,
    /// 10
    N10 = 9,
    /// 1
    N1 = 10,
    /// 0.1
    N01 = 11,
    /// 0.01
    N001 = 12,
    /// 0.001
    N0001 = 13,
    /// 0.000 1
    N00001 = 14,
    /// 0.000 01
    N000001 = 15,
    /// 0.000 001
    N0000001 = 16,
    /// 1e-7
    N1eMinus7 = 17,
    /// 1e-8
    N1eMinus8 = 18,
    /// 1e-9
    N1eMinus9 = 19,
    /// 1e-10
    N1eMinus10 = 20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFasciasolverFemtetstiffnessexp {
    /// 1e+10
    N1ePlus10 = 0,
    /// 1e+9
    N1ePlus9 = 1,
    /// 1e+8
    N1ePlus8 = 2,
    /// 1e+7
    N1ePlus7 = 3,
    /// 1 000 000
    N1000000 = 4,
    /// 100 000
    N100000 = 5,
    /// 10 000
    N10000 = 6,
    /// 1 000
    N1000 = 7,
    /// 100
    N100 = 8,
    /// 10
    N10 = 9,
    /// 1
    N1 = 10,
    /// 0.1
    N01 = 11,
    /// 0.01
    N001 = 12,
    /// 0.001
    N0001 = 13,
    /// 0.000 1
    N00001 = 14,
    /// 0.000 01
    N000001 = 15,
    /// 0.000 001
    N0000001 = 16,
    /// 1e-7
    N1eMinus7 = 17,
    /// 1e-8
    N1eMinus8 = 18,
    /// 1e-9
    N1eMinus9 = 19,
    /// 1e-10
    N1eMinus10 = 20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFasciasolverTristiffnessexp {
    /// 1e+10
    N1ePlus10 = 0,
    /// 1e+9
    N1ePlus9 = 1,
    /// 1e+8
    N1ePlus8 = 2,
    /// 1e+7
    N1ePlus7 = 3,
    /// 1 000 000
    N1000000 = 4,
    /// 100 000
    N100000 = 5,
    /// 10 000
    N10000 = 6,
    /// 1 000
    N1000 = 7,
    /// 100
    N100 = 8,
    /// 10
    N10 = 9,
    /// 1
    N1 = 10,
    /// 0.1
    N01 = 11,
    /// 0.01
    N001 = 12,
    /// 0.001
    N0001 = 13,
    /// 0.000 1
    N00001 = 14,
    /// 0.000 01
    N000001 = 15,
    /// 0.000 001
    N0000001 = 16,
    /// 1e-7
    N1eMinus7 = 17,
    /// 1e-8
    N1eMinus8 = 18,
    /// 1e-9
    N1eMinus9 = 19,
    /// 1e-10
    N1eMinus10 = 20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFasciasolverTetstiffnessexp {
    /// 1e+10
    N1ePlus10 = 0,
    /// 1e+9
    N1ePlus9 = 1,
    /// 1e+8
    N1ePlus8 = 2,
    /// 1e+7
    N1ePlus7 = 3,
    /// 1 000 000
    N1000000 = 4,
    /// 100 000
    N100000 = 5,
    /// 10 000
    N10000 = 6,
    /// 1 000
    N1000 = 7,
    /// 100
    N100 = 8,
    /// 10
    N10 = 9,
    /// 1
    N1 = 10,
    /// 0.1
    N01 = 11,
    /// 0.01
    N001 = 12,
    /// 0.001
    N0001 = 13,
    /// 0.000 1
    N00001 = 14,
    /// 0.000 01
    N000001 = 15,
    /// 0.000 001
    N0000001 = 16,
    /// 1e-7
    N1eMinus7 = 17,
    /// 1e-8
    N1eMinus8 = 18,
    /// 1e-9
    N1eMinus9 = 19,
    /// 1e-10
    N1eMinus10 = 20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFasciasolverVellumattachexp {
    /// 1e+10
    N1ePlus10 = 0,
    /// 1e+9
    N1ePlus9 = 1,
    /// 1e+8
    N1ePlus8 = 2,
    /// 1e+7
    N1ePlus7 = 3,
    /// 1 000 000
    N1000000 = 4,
    /// 100 000
    N100000 = 5,
    /// 10 000
    N10000 = 6,
    /// 1 000
    N1000 = 7,
    /// 100
    N100 = 8,
    /// 10
    N10 = 9,
    /// 1
    N1 = 10,
    /// 0.1
    N01 = 11,
    /// 0.01
    N001 = 12,
    /// 0.001
    N0001 = 13,
    /// 0.000 1
    N00001 = 14,
    /// 0.000 01
    N000001 = 15,
    /// 0.000 001
    N0000001 = 16,
    /// 1e-7
    N1eMinus7 = 17,
    /// 1e-8
    N1eMinus8 = 18,
    /// 1e-9
    N1eMinus9 = 19,
    /// 1e-10
    N1eMinus10 = 20,
}

#[derive(Debug, Clone)]
pub struct SopFasciasolver {
    pub base: crate::core::types::NodeBase,
}

impl SopFasciasolver {
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

    /// Connects to input 0: "Fascia"
    pub fn set_input_fascia(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Fascia" and specifies the output index of the target node.
    pub fn set_input_fascia_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Collision Geometry"
    pub fn set_input_collision_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Collision Geometry" and specifies the output index of the target node.
    pub fn set_input_collision_geometry_from(
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
    pub fn trigger_femresimulate(mut self) -> Self {
        self.base.params.insert(
            "femresimulate".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_femrestscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "femrestscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_femrestscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "femrestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_femtrianglestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "femtrianglestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_femtrianglestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "femtrianglestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_femtetstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "femtetstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_femtetstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "femtetstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_femdampingratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "femdampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_femdampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "femdampingratio".to_string(),
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
    pub fn with_trianglestiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "trianglestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_trianglestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trianglestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tetstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tetstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tetstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tetstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampingratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumattachstrength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vellumattachstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vellumattachstrength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumattachstrength".to_string(),
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

    // --- Int parameters ---
    pub fn with_solvertype(mut self, val: SopFasciasolverSolvertype) -> Self {
        self.base.params.insert(
            "solvertype".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
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
    pub fn with_endframe(mut self, val: i32) -> Self {
        self.base.params.insert(
            "endframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_endframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "endframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_femsubsteps(mut self, val: i32) -> Self {
        self.base.params.insert(
            "femsubsteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_femsubsteps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "femsubsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_femcollisionpasses(mut self, val: i32) -> Self {
        self.base.params.insert(
            "femcollisionpasses".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_femcollisionpasses_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "femcollisionpasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

    // --- Menu parameters ---
    pub fn with_femtristiffnessexp(mut self, val: SopFasciasolverFemtristiffnessexp) -> Self {
        self.base.params.insert(
            "femtristiffnessexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_femtristiffnessexp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "femtristiffnessexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_femtetstiffnessexp(mut self, val: SopFasciasolverFemtetstiffnessexp) -> Self {
        self.base.params.insert(
            "femtetstiffnessexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_femtetstiffnessexp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "femtetstiffnessexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tristiffnessexp(mut self, val: SopFasciasolverTristiffnessexp) -> Self {
        self.base.params.insert(
            "tristiffnessexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tristiffnessexp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tristiffnessexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tetstiffnessexp(mut self, val: SopFasciasolverTetstiffnessexp) -> Self {
        self.base.params.insert(
            "tetstiffnessexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tetstiffnessexp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tetstiffnessexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumattachexp(mut self, val: SopFasciasolverVellumattachexp) -> Self {
        self.base.params.insert(
            "vellumattachexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vellumattachexp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vellumattachexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_femrestscaleattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "femrestscaleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_femrestscaleattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "femrestscaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restscaleattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "restscaleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restscaleattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restscaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fasciastiffnessattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fasciastiffnessattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fasciastiffnessattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fasciastiffnessattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usefemrestscaleattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usefemrestscaleattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usefemrestscaleattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usefemrestscaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_userestscaleattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "userestscaleattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userestscaleattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "userestscaleattrib".to_string(),
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
}

impl crate::core::types::HoudiniNode for SopFasciasolver {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "fasciasolver"
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
pub enum SopFeatherattribinterpolateBarbsegmode {
    Constant = 0,
    MatchSource = 1,
}

#[derive(Debug, Clone)]
pub struct SopFeatherattribinterpolate {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherattribinterpolate {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Source Feathers"
    pub fn set_input_source_feathers(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Source Feathers" and specifies the output index of the target node.
    pub fn set_input_source_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_shaftbasesegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftbasesegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftbasesegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbasesegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_barbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_barbsegmode(mut self, val: SopFeatherattribinterpolateBarbsegmode) -> Self {
        self.base.params.insert(
            "barbsegmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_barbsegmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegmode".to_string(),
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
    pub fn with_barbattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_identifierattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "identifierattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_identifierattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "identifierattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weightsattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "weightsattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_weightsattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "weightsattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "blendattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blendattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftbasesegsattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftbasesegsattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftbasesegsattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbasesegsattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_shaftsubd(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shaftsubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shaftsubd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftsubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useshaftbasesegsattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useshaftbasesegsattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useshaftbasesegsattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useshaftbasesegsattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbmirror(mut self, val: bool) -> Self {
        self.base.params.insert(
            "barbmirror".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_barbmirror_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbmirror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherattribinterpolate {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherattribinterpolate"
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
pub struct SopFeatherbarbtangents {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherbarbtangents {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherbarbtangents {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherbarbtangents"
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
pub enum SopFeatherbarbxformXform {
    FeatherToObjectSpace = 0,
    ObjectToFeatherSpace = 1,
}

#[derive(Debug, Clone)]
pub struct SopFeatherbarbxform {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherbarbxform {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_xform(mut self, val: SopFeatherbarbxformXform) -> Self {
        self.base.params.insert(
            "xform".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xform_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherbarbxform {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherbarbxform"
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
pub enum SopFeatherclumpSplitlocmode {
    Parametric = 0,
    FromAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpSplitfreqoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpSplitfreqoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpSplitfrequvmode {
    SkinUv = 0,
    CurveUv = 1,
    FeatherUv = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpAmountoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpAmountoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpAmountuvmode {
    SkinUv = 0,
    CurveUv = 1,
    FeatherUv = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpFalloffoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpFalloffoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpFalloffuvmode {
    SkinUv = 0,
    CurveUv = 1,
    FeatherUv = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpSplitdepthoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpSplitdepthoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpSplitdepthuvmode {
    SkinUv = 0,
    CurveUv = 1,
    FeatherUv = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpShiftoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpShiftoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherclumpShiftuvmode {
    SkinUv = 0,
    CurveUv = 1,
    FeatherUv = 2,
}

#[derive(Debug, Clone)]
pub struct SopFeatherclump {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherclump {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_splitfreq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "splitfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_splitfreq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitjitter(mut self, val: f32) -> Self {
        self.base.params.insert(
            "splitjitter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_splitjitter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitjitter".to_string(),
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
    pub fn with_amount(mut self, val: f32) -> Self {
        self.base.params.insert(
            "amount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitdepth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "splitdepth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_splitdepth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitdepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shift(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shift".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shift_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shift".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_splitfreqinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "splitfreqinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_splitfreqinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitfreqinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitfreqoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "splitfreqoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_splitfreqoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitfreqoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amountinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "amountinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_amountinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amountinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amountoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "amountoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_amountoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amountoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "falloffinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_falloffinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "falloffoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_falloffoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitdepthinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "splitdepthinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_splitdepthinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitdepthinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitdepthoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "splitdepthoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_splitdepthoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitdepthoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shiftinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "shiftinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_shiftinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shiftinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shiftoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "shiftoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_shiftoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shiftoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_splitlocmode(mut self, val: SopFeatherclumpSplitlocmode) -> Self {
        self.base.params.insert(
            "splitlocmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_splitlocmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitlocmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitfreqoverride(mut self, val: SopFeatherclumpSplitfreqoverride) -> Self {
        self.base.params.insert(
            "splitfreqoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_splitfreqoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitfreqoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitfreqoptions(mut self, val: SopFeatherclumpSplitfreqoptions) -> Self {
        self.base.params.insert(
            "splitfreqoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_splitfreqoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitfreqoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitfrequvmode(mut self, val: SopFeatherclumpSplitfrequvmode) -> Self {
        self.base.params.insert(
            "splitfrequvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_splitfrequvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitfrequvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amountoverride(mut self, val: SopFeatherclumpAmountoverride) -> Self {
        self.base.params.insert(
            "amountoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amountoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amountoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amountoptions(mut self, val: SopFeatherclumpAmountoptions) -> Self {
        self.base.params.insert(
            "amountoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amountoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amountoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amountuvmode(mut self, val: SopFeatherclumpAmountuvmode) -> Self {
        self.base.params.insert(
            "amountuvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amountuvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amountuvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffoverride(mut self, val: SopFeatherclumpFalloffoverride) -> Self {
        self.base.params.insert(
            "falloffoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_falloffoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffoptions(mut self, val: SopFeatherclumpFalloffoptions) -> Self {
        self.base.params.insert(
            "falloffoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_falloffoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffuvmode(mut self, val: SopFeatherclumpFalloffuvmode) -> Self {
        self.base.params.insert(
            "falloffuvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_falloffuvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffuvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitdepthoverride(mut self, val: SopFeatherclumpSplitdepthoverride) -> Self {
        self.base.params.insert(
            "splitdepthoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_splitdepthoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitdepthoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitdepthoptions(mut self, val: SopFeatherclumpSplitdepthoptions) -> Self {
        self.base.params.insert(
            "splitdepthoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_splitdepthoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitdepthoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitdepthuvmode(mut self, val: SopFeatherclumpSplitdepthuvmode) -> Self {
        self.base.params.insert(
            "splitdepthuvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_splitdepthuvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitdepthuvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shiftoverride(mut self, val: SopFeatherclumpShiftoverride) -> Self {
        self.base.params.insert(
            "shiftoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shiftoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shiftoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shiftoptions(mut self, val: SopFeatherclumpShiftoptions) -> Self {
        self.base.params.insert(
            "shiftoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shiftoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shiftoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shiftuvmode(mut self, val: SopFeatherclumpShiftuvmode) -> Self {
        self.base.params.insert(
            "shiftuvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shiftuvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shiftuvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_splitfreqremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "splitfreqremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_splitfreqremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitfreqremapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amountremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "amountremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_amountremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amountremapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "falloffremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_falloffremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffremapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitdepthremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "splitdepthremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_splitdepthremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitdepthremapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shiftremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "shiftremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_shiftremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shiftremapramp".to_string(),
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
    pub fn with_splitattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "splitattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_splitattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitfreqcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "splitfreqcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_splitfreqcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitfreqcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitfreqattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "splitfreqattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_splitfreqattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitfreqattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitfreqtexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "splitfreqtexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_splitfreqtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitfreqtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitfreqtextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "splitfreqtextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_splitfreqtextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitfreqtextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outsplitattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outsplitattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outsplitattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outsplitattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outclumpidattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outclumpidattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outclumpidattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outclumpidattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amountcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amountcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amountcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amountcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amountattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amountattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amountattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amountattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amounttexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amounttexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amounttexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amounttexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amounttextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amounttextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amounttextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amounttextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "falloffcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_falloffcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "falloffattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_falloffattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fallofftexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fallofftexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fallofftexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fallofftexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fallofftextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fallofftextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fallofftextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fallofftextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitdepthcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "splitdepthcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_splitdepthcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitdepthcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitdepthattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "splitdepthattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_splitdepthattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitdepthattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitdepthtexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "splitdepthtexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_splitdepthtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitdepthtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitdepthtextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "splitdepthtextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_splitdepthtextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "splitdepthtextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shiftcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shiftcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shiftcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shiftcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shiftattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shiftattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shiftattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shiftattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shifttexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shifttexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shifttexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shifttexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shifttextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shifttextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shifttextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shifttextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinuvattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "skinuvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skinuvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinuvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_featheruvattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "featheruvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_featheruvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "featheruvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_createsplitattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createsplitattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createsplitattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createsplitattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createclumpidattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createclumpidattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createclumpidattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createclumpidattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doclump(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doclump".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doclump_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doclump".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherclump {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherclump"
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
pub enum SopFeatherconvertOutputtype {
    Curves = 0,
    Surface = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherconvertShaftoutput {
    None = 0,
    Curve = 1,
    Surface = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherconvertShaftwidthoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherconvertShaftwidthoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherconvertShaftwidthuvmode {
    SkinUv = 0,
    FeatherUv = 1,
}

#[derive(Debug, Clone)]
pub struct SopFeatherconvert {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherconvert {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_shaftwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shaftwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shaftwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_shaftwidthinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "shaftwidthinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_shaftwidthinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "shaftwidthoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_shaftwidthoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_skipfirstnbarbpoints(mut self, val: i32) -> Self {
        self.base.params.insert(
            "skipfirstnbarbpoints".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_skipfirstnbarbpoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skipfirstnbarbpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftmaxwidthsegments(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftmaxwidthsegments".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftmaxwidthsegments_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftmaxwidthsegments".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_outputtype(mut self, val: SopFeatherconvertOutputtype) -> Self {
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
    pub fn with_shaftoutput(mut self, val: SopFeatherconvertShaftoutput) -> Self {
        self.base.params.insert(
            "shaftoutput".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftoutput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthoverride(mut self, val: SopFeatherconvertShaftwidthoverride) -> Self {
        self.base.params.insert(
            "shaftwidthoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftwidthoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthoptions(mut self, val: SopFeatherconvertShaftwidthoptions) -> Self {
        self.base.params.insert(
            "shaftwidthoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftwidthoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthuvmode(mut self, val: SopFeatherconvertShaftwidthuvmode) -> Self {
        self.base.params.insert(
            "shaftwidthuvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftwidthuvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthuvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_shaftwidthremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "shaftwidthremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_shaftwidthremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthremapramp".to_string(),
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
    pub fn with_normuvattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "normuvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normuvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "normuvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftwidthcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftwidthcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftwidthattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftwidthattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthtexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftwidthtexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftwidthtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthtextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftwidthtextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftwidthtextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthtextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barblgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barblgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barblgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barblgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbrgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbrgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbrgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbrgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primgroups(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primgroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primgroups_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointgroups(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pointgroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointgroups_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbattribsets(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbattribsets".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbattribsets_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbattribsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_columnattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "columnattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_columnattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "columnattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_createnormuv(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createnormuv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createnormuv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createnormuv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidth_multfirstbarb(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shaftwidth_multfirstbarb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shaftwidth_multfirstbarb_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidth_multfirstbarb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createbarbgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createbarbgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createbarbgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createbarbgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createshaftgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createshaftgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createshaftgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createshaftgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createshaftptattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createshaftptattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createshaftptattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createshaftptattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createid(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcolumnattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputcolumnattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputcolumnattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputcolumnattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherconvert {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherconvert"
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
pub enum SopFeatherdeformDeformertype {
    Curves = 0,
    Feathers = 1,
    Surfaces = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherdeformMode {
    CaptureAndDeform = 0,
    Capture = 1,
    Deform = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherdeformDeformbarbsoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherdeformDeformbarbsoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherdeformDeformbarbsuvmode {
    SkinUv = 0,
    CurveUv = 1,
    FeatherUv = 2,
}

#[derive(Debug, Clone)]
pub struct SopFeatherdeform {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherdeform {
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

    /// Connects to input 0: "Geometry to Deform"
    pub fn set_input_geometry_to_deform(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Deform" and specifies the output index of the target node.
    pub fn set_input_geometry_to_deform_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Rest Deformers"
    pub fn set_input_rest_deformers(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Rest Deformers" and specifies the output index of the target node.
    pub fn set_input_rest_deformers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: "Animated Deformers"
    pub fn set_input_animated_deformers(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "Animated Deformers" and specifies the output index of the target node.
    pub fn set_input_animated_deformers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_deformbarbs(mut self, val: f32) -> Self {
        self.base.params.insert(
            "deformbarbs".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_deformbarbs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deformbarbs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_deformbarbsinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "deformbarbsinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_deformbarbsinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deformbarbsinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deformbarbsoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "deformbarbsoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_deformbarbsoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deformbarbsoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_deformertype(mut self, val: SopFeatherdeformDeformertype) -> Self {
        self.base.params.insert(
            "deformertype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_deformertype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deformertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: SopFeatherdeformMode) -> Self {
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
    pub fn with_deformbarbsoverride(mut self, val: SopFeatherdeformDeformbarbsoverride) -> Self {
        self.base.params.insert(
            "deformbarbsoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_deformbarbsoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deformbarbsoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deformbarbsoptions(mut self, val: SopFeatherdeformDeformbarbsoptions) -> Self {
        self.base.params.insert(
            "deformbarbsoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_deformbarbsoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deformbarbsoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deformbarbsuvmode(mut self, val: SopFeatherdeformDeformbarbsuvmode) -> Self {
        self.base.params.insert(
            "deformbarbsuvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_deformbarbsuvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deformbarbsuvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_deformbarbsremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "deformbarbsremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_deformbarbsremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deformbarbsremapramp".to_string(),
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
    pub fn with_deformbarbscurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "deformbarbscurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deformbarbscurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deformbarbscurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deformbarbsattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "deformbarbsattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deformbarbsattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deformbarbsattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deformbarbstexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "deformbarbstexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deformbarbstexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deformbarbstexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deformbarbstextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "deformbarbstextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deformbarbstextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deformbarbstextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_skinuvattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "skinuvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skinuvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinuvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_featheruvattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "featheruvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_featheruvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "featheruvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_treatdeformerassubd(mut self, val: bool) -> Self {
        self.base.params.insert(
            "treatdeformerassubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_treatdeformerassubd_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "treatdeformerassubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transfervel(mut self, val: bool) -> Self {
        self.base.params.insert(
            "transfervel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_transfervel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "transfervel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rigidxform(mut self, val: bool) -> Self {
        self.base.params.insert(
            "rigidxform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rigidxform_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rigidxform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_delcaptureattribs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "delcaptureattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_delcaptureattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "delcaptureattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherdeform {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherdeform"
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
pub enum SopFeatherdeintersectOrdermode {
    /// Automatic (Find Neighbors)
    AutomaticFindNeighbors = 0,
    LayerAttribute = 1,
    NeighborArrays = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherdeintersectSidelayeringmode {
    PositionAttribute = 0,
    DirectionAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherdeintersectSidelayeringposattribcomp {
    X = 0,
    Y = 1,
    Z = 2,
}

#[derive(Debug, Clone)]
pub struct SopFeatherdeintersect {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherdeintersect {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_neighborradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "neighborradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_neighborradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "neighborradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sideconeangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sideconeangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sideconeangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sideconeangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_barbsegsreduceamount(mut self, val: f32) -> Self {
        self.base.params.insert(
            "barbsegsreduceamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barbsegsreduceamount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegsreduceamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegsreducebias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "barbsegsreducebias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barbsegsreducebias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegsreducebias".to_string(),
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
    pub fn with_smoothiters(mut self, val: i32) -> Self {
        self.base.params.insert(
            "smoothiters".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_smoothiters_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "smoothiters".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relaxiters(mut self, val: i32) -> Self {
        self.base.params.insert(
            "relaxiters".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_relaxiters_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "relaxiters".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftbarbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftbarbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftbarbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbarbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_barbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_ordermode(mut self, val: SopFeatherdeintersectOrdermode) -> Self {
        self.base.params.insert(
            "ordermode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_ordermode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ordermode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sidelayeringmode(mut self, val: SopFeatherdeintersectSidelayeringmode) -> Self {
        self.base.params.insert(
            "sidelayeringmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sidelayeringmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sidelayeringmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sidelayeringposattribcomp(
        mut self,
        val: SopFeatherdeintersectSidelayeringposattribcomp,
    ) -> Self {
        self.base.params.insert(
            "sidelayeringposattribcomp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sidelayeringposattribcomp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sidelayeringposattribcomp".to_string(),
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
    pub fn with_layerattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "layerattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_layerattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "layerattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frontneighborsattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "frontneighborsattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_frontneighborsattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "frontneighborsattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rearneighborsattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rearneighborsattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rearneighborsattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rearneighborsattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sidelayeringposattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sidelayeringposattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sidelayeringposattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sidelayeringposattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sidelayeringdirattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sidelayeringdirattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sidelayeringdirattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sidelayeringdirattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_smoothdeform(mut self, val: bool) -> Self {
        self.base.params.insert(
            "smoothdeform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_smoothdeform_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "smoothdeform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relax(mut self, val: bool) -> Self {
        self.base.params.insert(
            "relax".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_relax_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "relax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sidelayeringposattribabs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "sidelayeringposattribabs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sidelayeringposattribabs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sidelayeringposattribabs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sidelayeringreverse(mut self, val: bool) -> Self {
        self.base.params.insert(
            "sidelayeringreverse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sidelayeringreverse_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sidelayeringreverse".to_string(),
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
    pub fn with_resampleshaft(mut self, val: bool) -> Self {
        self.base.params.insert(
            "resampleshaft".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resampleshaft_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resampleshaft".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resamplebarbs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "resamplebarbs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resamplebarbs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resamplebarbs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegsreduce(mut self, val: bool) -> Self {
        self.base.params.insert(
            "barbsegsreduce".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_barbsegsreduce_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegsreduce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherdeintersect {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherdeintersect"
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
pub struct SopFeatherdeintersectExpandneighbors {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherdeintersectExpandneighbors {
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

    /// Connects to input 1: "Sub-Network Input #2"
    pub fn set_input_sub_network_input_2(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
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
    pub fn with_neighboriterstep(mut self, val: i32) -> Self {
        self.base.params.insert(
            "neighboriterstep".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_neighboriterstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "neighboriterstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_neighborsattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "neighborsattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_neighborsattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "neighborsattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherdeintersectExpandneighbors {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherdeintersect_expandneighbors"
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
pub enum SopFeatherinstancepoolOutput {
    Curves = 0,
    Agents = 1,
    PackedGeo = 2,
}

#[derive(Debug, Clone)]
pub struct SopFeatherinstancepool {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherinstancepool {
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

    /// Connects to input 0: "Feather Templates"
    pub fn set_input_feather_templates(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feather Templates" and specifies the output index of the target node.
    pub fn set_input_feather_templates_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Feather Guides"
    pub fn set_input_feather_guides(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Feather Guides" and specifies the output index of the target node.
    pub fn set_input_feather_guides_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_weightsteps(mut self, val: i32) -> Self {
        self.base.params.insert(
            "weightsteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_weightsteps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "weightsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_agent_shaftsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "agent_shaftsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_agent_shaftsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "agent_shaftsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_agent_shaftbarbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "agent_shaftbarbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_agent_shaftbarbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "agent_shaftbarbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_barbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_output(mut self, val: SopFeatherinstancepoolOutput) -> Self {
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
    pub fn with_primattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_templateblending(mut self, val: bool) -> Self {
        self.base.params.insert(
            "templateblending".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_templateblending_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "templateblending".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expand(mut self, val: bool) -> Self {
        self.base.params.insert(
            "expand".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_expand_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "expand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cache(mut self, val: bool) -> Self {
        self.base.params.insert(
            "cache".to_string(),
            crate::core::types::ParamValue::Toggle(val),
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
    pub fn with_agent_createbarbjoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "agent_createbarbjoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_agent_createbarbjoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "agent_createbarbjoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherinstancepool {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherinstancepool"
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
pub struct SopFeathermatchuncondensed {
    pub base: crate::core::types::NodeBase,
}

impl SopFeathermatchuncondensed {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Uncondensed Feathers"
    pub fn set_input_uncondensed_feathers(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Uncondensed Feathers" and specifies the output index of the target node.
    pub fn set_input_uncondensed_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
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
    pub fn with_barbattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_transferpos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "transferpos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_transferpos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "transferpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transferuv(mut self, val: bool) -> Self {
        self.base.params.insert(
            "transferuv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_transferuv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "transferuv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeathermatchuncondensed {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "feathermatchuncondensed"
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
pub struct SopFeathermindist {
    pub base: crate::core::types::NodeBase,
}

impl SopFeathermindist {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Target Feathers"
    pub fn set_input_target_feathers(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Target Feathers" and specifies the output index of the target node.
    pub fn set_input_target_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_idattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "idattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_idattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "idattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeathermindist {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "feathermindist"
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
pub enum SopFeathernoiseAmplitudeoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseAmplitudeoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseAmplitudeuvmode {
    SkinUv = 0,
    FeatherUv = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseAmplitudenormaloverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseAmplitudenormaloptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseAmplitudenormaluvmode {
    SkinUv = 0,
    FeatherUv = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseAmplitudetangentoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseAmplitudetangentoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseAmplitudetangentuvmode {
    SkinUv = 0,
    FeatherUv = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseAmplitudebitangentoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseAmplitudebitangentoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseAmplitudebitangentuvmode {
    SkinUv = 0,
    FeatherUv = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseShaftoffsetoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseShaftoffsetoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseShaftoffsetuvmode {
    SkinUv = 0,
    FeatherUv = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseFalloffoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseFalloffoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseFalloffuvmode {
    SkinUv = 0,
    FeatherUv = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseShaftfreqoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseShaftfreqoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseShaftfrequvmode {
    SkinUv = 0,
    FeatherUv = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseBarbfreqoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseBarbfreqoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathernoiseBarbfrequvmode {
    SkinUv = 0,
    FeatherUv = 1,
}

#[derive(Debug, Clone)]
pub struct SopFeathernoise {
    pub base: crate::core::types::NodeBase,
}

impl SopFeathernoise {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_amplitude(mut self, val: f32) -> Self {
        self.base.params.insert(
            "amplitude".to_string(),
            crate::core::types::ParamValue::Float(val),
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
    pub fn with_amplitudenormal(mut self, val: f32) -> Self {
        self.base.params.insert(
            "amplitudenormal".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amplitudenormal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudenormal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudetangent(mut self, val: f32) -> Self {
        self.base.params.insert(
            "amplitudetangent".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amplitudetangent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudetangent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangent(mut self, val: f32) -> Self {
        self.base.params.insert(
            "amplitudebitangent".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amplitudebitangent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shaftoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shaftoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftfreq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shaftfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shaftfreq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbfreq(mut self, val: f32) -> Self {
        self.base.params.insert(
            "barbfreq".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barbfreq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbfreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_amplitudeinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "amplitudeinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_amplitudeinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudeinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudeoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "amplitudeoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_amplitudeoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudeoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudenormalinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "amplitudenormalinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_amplitudenormalinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudenormalinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudenormaloutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "amplitudenormaloutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_amplitudenormaloutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudenormaloutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudetangentinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "amplitudetangentinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_amplitudetangentinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudetangentinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudetangentoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "amplitudetangentoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_amplitudetangentoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudetangentoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangentinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "amplitudebitangentinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_amplitudebitangentinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangentinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangentoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "amplitudebitangentoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_amplitudebitangentoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangentoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftoffsetinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "shaftoffsetinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_shaftoffsetinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftoffsetinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftoffsetoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "shaftoffsetoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_shaftoffsetoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftoffsetoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "falloffinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_falloffinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "falloffoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_falloffoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftfreqinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "shaftfreqinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_shaftfreqinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftfreqinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftfreqoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "shaftfreqoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_shaftfreqoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftfreqoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbfreqinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "barbfreqinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_barbfreqinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbfreqinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbfreqoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "barbfreqoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_barbfreqoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbfreqoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_amplitudeoverride(mut self, val: SopFeathernoiseAmplitudeoverride) -> Self {
        self.base.params.insert(
            "amplitudeoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amplitudeoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudeoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudeoptions(mut self, val: SopFeathernoiseAmplitudeoptions) -> Self {
        self.base.params.insert(
            "amplitudeoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amplitudeoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudeoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudeuvmode(mut self, val: SopFeathernoiseAmplitudeuvmode) -> Self {
        self.base.params.insert(
            "amplitudeuvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amplitudeuvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudeuvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudenormaloverride(
        mut self,
        val: SopFeathernoiseAmplitudenormaloverride,
    ) -> Self {
        self.base.params.insert(
            "amplitudenormaloverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amplitudenormaloverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudenormaloverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudenormaloptions(
        mut self,
        val: SopFeathernoiseAmplitudenormaloptions,
    ) -> Self {
        self.base.params.insert(
            "amplitudenormaloptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amplitudenormaloptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudenormaloptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudenormaluvmode(mut self, val: SopFeathernoiseAmplitudenormaluvmode) -> Self {
        self.base.params.insert(
            "amplitudenormaluvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amplitudenormaluvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudenormaluvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudetangentoverride(
        mut self,
        val: SopFeathernoiseAmplitudetangentoverride,
    ) -> Self {
        self.base.params.insert(
            "amplitudetangentoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amplitudetangentoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudetangentoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudetangentoptions(
        mut self,
        val: SopFeathernoiseAmplitudetangentoptions,
    ) -> Self {
        self.base.params.insert(
            "amplitudetangentoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amplitudetangentoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudetangentoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudetangentuvmode(
        mut self,
        val: SopFeathernoiseAmplitudetangentuvmode,
    ) -> Self {
        self.base.params.insert(
            "amplitudetangentuvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amplitudetangentuvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudetangentuvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangentoverride(
        mut self,
        val: SopFeathernoiseAmplitudebitangentoverride,
    ) -> Self {
        self.base.params.insert(
            "amplitudebitangentoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amplitudebitangentoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangentoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangentoptions(
        mut self,
        val: SopFeathernoiseAmplitudebitangentoptions,
    ) -> Self {
        self.base.params.insert(
            "amplitudebitangentoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amplitudebitangentoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangentoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangentuvmode(
        mut self,
        val: SopFeathernoiseAmplitudebitangentuvmode,
    ) -> Self {
        self.base.params.insert(
            "amplitudebitangentuvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_amplitudebitangentuvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangentuvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftoffsetoverride(mut self, val: SopFeathernoiseShaftoffsetoverride) -> Self {
        self.base.params.insert(
            "shaftoffsetoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftoffsetoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftoffsetoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftoffsetoptions(mut self, val: SopFeathernoiseShaftoffsetoptions) -> Self {
        self.base.params.insert(
            "shaftoffsetoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftoffsetoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftoffsetoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftoffsetuvmode(mut self, val: SopFeathernoiseShaftoffsetuvmode) -> Self {
        self.base.params.insert(
            "shaftoffsetuvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftoffsetuvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftoffsetuvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffoverride(mut self, val: SopFeathernoiseFalloffoverride) -> Self {
        self.base.params.insert(
            "falloffoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_falloffoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffoptions(mut self, val: SopFeathernoiseFalloffoptions) -> Self {
        self.base.params.insert(
            "falloffoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_falloffoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffuvmode(mut self, val: SopFeathernoiseFalloffuvmode) -> Self {
        self.base.params.insert(
            "falloffuvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_falloffuvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffuvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftfreqoverride(mut self, val: SopFeathernoiseShaftfreqoverride) -> Self {
        self.base.params.insert(
            "shaftfreqoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftfreqoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftfreqoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftfreqoptions(mut self, val: SopFeathernoiseShaftfreqoptions) -> Self {
        self.base.params.insert(
            "shaftfreqoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftfreqoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftfreqoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftfrequvmode(mut self, val: SopFeathernoiseShaftfrequvmode) -> Self {
        self.base.params.insert(
            "shaftfrequvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftfrequvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftfrequvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbfreqoverride(mut self, val: SopFeathernoiseBarbfreqoverride) -> Self {
        self.base.params.insert(
            "barbfreqoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_barbfreqoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbfreqoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbfreqoptions(mut self, val: SopFeathernoiseBarbfreqoptions) -> Self {
        self.base.params.insert(
            "barbfreqoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_barbfreqoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbfreqoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbfrequvmode(mut self, val: SopFeathernoiseBarbfrequvmode) -> Self {
        self.base.params.insert(
            "barbfrequvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_barbfrequvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbfrequvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_amplituderemapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "amplituderemapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_amplituderemapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplituderemapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudenormalremapramp(
        mut self,
        val: Vec<crate::core::types::RampPoint>,
    ) -> Self {
        self.base.params.insert(
            "amplitudenormalremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_amplitudenormalremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudenormalremapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudetangentremapramp(
        mut self,
        val: Vec<crate::core::types::RampPoint>,
    ) -> Self {
        self.base.params.insert(
            "amplitudetangentremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_amplitudetangentremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudetangentremapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangentremapramp(
        mut self,
        val: Vec<crate::core::types::RampPoint>,
    ) -> Self {
        self.base.params.insert(
            "amplitudebitangentremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_amplitudebitangentremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangentremapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftoffsetremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "shaftoffsetremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_shaftoffsetremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftoffsetremapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "falloffremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_falloffremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffremapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftfreqremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "shaftfreqremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_shaftfreqremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftfreqremapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbfreqremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "barbfreqremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_barbfreqremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbfreqremapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_amplitudecurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudecurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudecurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudecurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudeattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudeattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudeattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudetexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudetexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudetexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudetexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudetextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudetextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudetextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudetextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudenormalcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudenormalcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudenormalcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudenormalcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudenormalattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudenormalattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudenormalattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudenormalattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudenormaltexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudenormaltexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudenormaltexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudenormaltexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudenormaltextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudenormaltextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudenormaltextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudenormaltextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudetangentcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudetangentcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudetangentcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudetangentcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudetangentattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudetangentattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudetangentattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudetangentattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudetangenttexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudetangenttexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudetangenttexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudetangenttexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudetangenttextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudetangenttextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudetangenttextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudetangenttextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangentcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangentcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangentcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangentcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangentattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangentattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangentattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangentattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangenttexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangenttexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangenttexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangenttexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangenttextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangenttextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_amplitudebitangenttextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "amplitudebitangenttextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftoffsetcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftoffsetcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftoffsetcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftoffsetcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftoffsetattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftoffsetattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftoffsetattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftoffsetattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftoffsettexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftoffsettexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftoffsettexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftoffsettexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftoffsettextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftoffsettextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftoffsettextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftoffsettextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "falloffcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_falloffcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "falloffattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_falloffattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloffattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fallofftexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fallofftexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fallofftexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fallofftexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fallofftextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fallofftextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fallofftextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fallofftextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftfreqcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftfreqcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftfreqcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftfreqcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftfreqattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftfreqattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftfreqattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftfreqattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftfreqtexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftfreqtexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftfreqtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftfreqtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftfreqtextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftfreqtextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftfreqtextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftfreqtextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbfreqcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbfreqcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbfreqcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbfreqcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbfreqattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbfreqattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbfreqattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbfreqattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbfreqtexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbfreqtexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbfreqtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbfreqtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbfreqtextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbfreqtextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbfreqtextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbfreqtextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinuvattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "skinuvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skinuvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinuvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_featheruvattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "featheruvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_featheruvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "featheruvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeathernoise {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "feathernoise"
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
pub struct SopFeathernormalize {
    pub base: crate::core::types::NodeBase,
}

impl SopFeathernormalize {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Toggle parameters ---
    pub fn with_movetoorigin(mut self, val: bool) -> Self {
        self.base.params.insert(
            "movetoorigin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_movetoorigin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "movetoorigin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normalizelength(mut self, val: bool) -> Self {
        self.base.params.insert(
            "normalizelength".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normalizelength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "normalizelength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flatten(mut self, val: bool) -> Self {
        self.base.params.insert(
            "flatten".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flatten_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "flatten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeathernormalize {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "feathernormalize"
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
pub struct SopFeatherprimitive {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherprimitive {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_shaftsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftbarbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftbarbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftbarbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbarbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_barbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_fallbackname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fallbackname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fallbackname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fallbackname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_createbarbjoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createbarbjoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createbarbjoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createbarbjoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usebasename(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usebasename".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebasename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usebasename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_basenamewithid(mut self, val: bool) -> Self {
        self.base.params.insert(
            "basenamewithid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_basenamewithid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "basenamewithid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherprimitive {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherprimitive"
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
pub enum SopFeatherrayTextype {
    TextureImage = 0,
    TexturePrimitive = 1,
}

#[derive(Debug, Clone)]
pub struct SopFeatherray {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherray {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Volumes"
    pub fn set_input_volumes(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Volumes" and specifies the output index of the target node.
    pub fn set_input_volumes_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Ray Geometry"
    pub fn set_input_ray_geometry(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Ray Geometry" and specifies the output index of the target node.
    pub fn set_input_ray_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_textype(mut self, val: SopFeatherrayTextype) -> Self {
        self.base.params.insert(
            "textype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_textype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "textype".to_string(),
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
    pub fn with_vertexattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vertexattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vertexattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vertexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primnumattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primnumattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primnumattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primnumattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primuvattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primuvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primuvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primuvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texuv(mut self, val: &str) -> Self {
        self.base.params.insert(
            "texuv".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_texuv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texuv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "texpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_texpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "texprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_texprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "texattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_texattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texattribtype(mut self, val: &str) -> Self {
        self.base.params.insert(
            "texattribtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_texattribtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "texattribtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_createprimnumattribs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createprimnumattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createprimnumattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createprimnumattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createprimuvattribs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createprimuvattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createprimuvattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createprimuvattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sampletex(mut self, val: bool) -> Self {
        self.base.params.insert(
            "sampletex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sampletex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sampletex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherray {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherray"
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
pub struct SopFeatherraycore {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherraycore {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Source Feathers"
    pub fn set_input_source_feathers(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Source Feathers" and specifies the output index of the target node.
    pub fn set_input_source_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
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
    pub fn with_vertexattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vertexattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vertexattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vertexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primnumattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primnumattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primnumattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primnumattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primuvattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primuvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primuvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primuvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_createprimnumattribs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createprimnumattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createprimnumattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createprimnumattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createprimuvattribs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createprimuvattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createprimuvattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createprimuvattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherraycore {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherraycore"
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
pub enum SopFeatherresampleShaftbaseMode {
    Count = 0,
    Length = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherresampleShaftMode {
    Count = 0,
    Length = 1,
}

#[derive(Debug, Clone)]
pub struct SopFeatherresample {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherresample {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_shaftbase_length(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shaftbase_length".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shaftbase_length_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbase_length".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaft_length(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shaft_length".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shaft_length_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaft_length".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_shaftbase_maxsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftbase_maxsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftbase_maxsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbase_maxsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaft_maxsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaft_maxsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaft_maxsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaft_maxsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_barbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftbarbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftbarbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftbarbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbarbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_shaftbase_mode(mut self, val: SopFeatherresampleShaftbaseMode) -> Self {
        self.base.params.insert(
            "shaftbase_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftbase_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbase_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaft_mode(mut self, val: SopFeatherresampleShaftMode) -> Self {
        self.base.params.insert(
            "shaft_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaft_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaft_mode".to_string(),
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
    pub fn with_shaftbase_maxsegsattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftbase_maxsegsattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftbase_maxsegsattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbase_maxsegsattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftbase_lengthattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftbase_lengthattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftbase_lengthattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbase_lengthattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaft_maxsegsattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaft_maxsegsattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaft_maxsegsattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaft_maxsegsattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaft_lengthattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaft_lengthattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaft_lengthattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaft_lengthattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_shaftbase_resample(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shaftbase_resample".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shaftbase_resample_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbase_resample".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftbase_maxsegsuseattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shaftbase_maxsegsuseattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shaftbase_maxsegsuseattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbase_maxsegsuseattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftbase_lengthuseattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shaftbase_lengthuseattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shaftbase_lengthuseattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbase_lengthuseattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaft_resample(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shaft_resample".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shaft_resample_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaft_resample".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaft_maxsegsuseattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shaft_maxsegsuseattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shaft_maxsegsuseattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaft_maxsegsuseattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaft_lengthuseattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shaft_lengthuseattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shaft_lengthuseattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaft_lengthuseattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resamplebarbs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "resamplebarbs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resamplebarbs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resamplebarbs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resampleshaft(mut self, val: bool) -> Self {
        self.base.params.insert(
            "resampleshaft".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resampleshaft_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resampleshaft".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherresample {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherresample"
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
pub struct SopFeathershapeorg {
    pub base: crate::core::types::NodeBase,
}

impl SopFeathershapeorg {
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

    /// Connects to input 0: "Shape Curves"
    pub fn set_input_shape_curves(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Shape Curves" and specifies the output index of the target node.
    pub fn set_input_shape_curves_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Profile Curves"
    pub fn set_input_profile_curves(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Profile Curves" and specifies the output index of the target node.
    pub fn set_input_profile_curves_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Label Points"
    pub fn set_input_label_points(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Label Points" and specifies the output index of the target node.
    pub fn set_input_label_points_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeathershapeorg {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "feathershapeorg"
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
pub enum SopFeathersurfaceShaftoutput {
    None = 0,
    Curve = 1,
    Surface = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathersurfaceShaftwidthoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathersurfaceShaftwidthoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathersurfaceShaftwidthuvmode {
    SkinUv = 0,
    FeatherUv = 1,
}

#[derive(Debug, Clone)]
pub struct SopFeathersurface {
    pub base: crate::core::types::NodeBase,
}

impl SopFeathersurface {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_shaftwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shaftwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shaftwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegsreduceamount(mut self, val: f32) -> Self {
        self.base.params.insert(
            "barbsegsreduceamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barbsegsreduceamount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegsreduceamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegsreducebias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "barbsegsreducebias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barbsegsreducebias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegsreducebias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_shaftwidthinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "shaftwidthinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_shaftwidthinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "shaftwidthoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_shaftwidthoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_shaftmaxwidthsegments(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftmaxwidthsegments".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftmaxwidthsegments_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftmaxwidthsegments".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_shaftoutput(mut self, val: SopFeathersurfaceShaftoutput) -> Self {
        self.base.params.insert(
            "shaftoutput".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftoutput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthoverride(mut self, val: SopFeathersurfaceShaftwidthoverride) -> Self {
        self.base.params.insert(
            "shaftwidthoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftwidthoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthoptions(mut self, val: SopFeathersurfaceShaftwidthoptions) -> Self {
        self.base.params.insert(
            "shaftwidthoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftwidthoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthuvmode(mut self, val: SopFeathersurfaceShaftwidthuvmode) -> Self {
        self.base.params.insert(
            "shaftwidthuvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftwidthuvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthuvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_shaftwidthremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "shaftwidthremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_shaftwidthremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthremapramp".to_string(),
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
    pub fn with_normuvattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "normuvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normuvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "normuvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftwidthcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftwidthcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftwidthattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftwidthattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthtexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftwidthtexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftwidthtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthtextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftwidthtextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftwidthtextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthtextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barblgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barblgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barblgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barblgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbrgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbrgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbrgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbrgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primgroups(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primgroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primgroups_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointgroups(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pointgroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointgroups_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbattribsets(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbattribsets".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbattribsets_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbattribsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_createnormuv(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createnormuv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createnormuv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createnormuv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidth_multfirstbarb(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shaftwidth_multfirstbarb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shaftwidth_multfirstbarb_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidth_multfirstbarb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegsreduce(mut self, val: bool) -> Self {
        self.base.params.insert(
            "barbsegsreduce".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_barbsegsreduce_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegsreduce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createbarbgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createbarbgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createbarbgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createbarbgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createshaftgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createshaftgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createshaftgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createshaftgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createid(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeathersurface {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "feathersurface"
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
pub enum SopFeathersurfaceblendBlendoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathersurfaceblendBlendoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathersurfaceblendBlenduvmode {
    SkinUv = 0,
    CurveUv = 1,
    FeatherUv = 2,
}

#[derive(Debug, Clone)]
pub struct SopFeathersurfaceblend {
    pub base: crate::core::types::NodeBase,
}

impl SopFeathersurfaceblend {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Target Surface"
    pub fn set_input_target_surface(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Target Surface" and specifies the output index of the target node.
    pub fn set_input_target_surface_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_skinamount(mut self, val: f32) -> Self {
        self.base.params.insert(
            "skinamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_skinamount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegsreduceamount(mut self, val: f32) -> Self {
        self.base.params.insert(
            "barbsegsreduceamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barbsegsreduceamount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegsreduceamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegsreducebias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "barbsegsreducebias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barbsegsreducebias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegsreducebias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_blendinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "blendinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_blendinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "blendoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_blendoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendoutrange".to_string(),
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
    pub fn with_shaftsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftbarbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftbarbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftbarbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbarbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_barbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iterations2(mut self, val: i32) -> Self {
        self.base.params.insert(
            "iterations2".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iterations2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relaxiterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "relaxiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_relaxiterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "relaxiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deltamushiterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "deltamushiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deltamushiterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deltamushiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_blendoverride(mut self, val: SopFeathersurfaceblendBlendoverride) -> Self {
        self.base.params.insert(
            "blendoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendoptions(mut self, val: SopFeathersurfaceblendBlendoptions) -> Self {
        self.base.params.insert(
            "blendoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blenduvmode(mut self, val: SopFeathersurfaceblendBlenduvmode) -> Self {
        self.base.params.insert(
            "blenduvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blenduvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blenduvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_blendremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "blendremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_blendremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendremapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "matchramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_matchramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "matchramp".to_string(),
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
    pub fn with_blendcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "blendcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blendcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "blendattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blendattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendtexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "blendtexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blendtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendtextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "blendtextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blendtextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendtextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
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
    pub fn with_resampleshaft(mut self, val: bool) -> Self {
        self.base.params.insert(
            "resampleshaft".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resampleshaft_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resampleshaft".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resamplebarbs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "resamplebarbs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resamplebarbs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resamplebarbs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegsreduce(mut self, val: bool) -> Self {
        self.base.params.insert(
            "barbsegsreduce".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_barbsegsreduce_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegsreduce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remesh(mut self, val: bool) -> Self {
        self.base.params.insert(
            "remesh".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remesh_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remesh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relaxshape(mut self, val: bool) -> Self {
        self.base.params.insert(
            "relaxshape".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_relaxshape_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "relaxshape".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeathersurfaceblend {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "feathersurfaceblend"
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
pub enum SopFeathertemplateassignMaskoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathertemplateassignMaskoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone)]
pub struct SopFeathertemplateassign {
    pub base: crate::core::types::NodeBase,
}

impl SopFeathertemplateassign {
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

    /// Connects to input 0: "Curves or Feathers"
    pub fn set_input_curves_or_feathers(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Curves or Feathers" and specifies the output index of the target node.
    pub fn set_input_curves_or_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Skin VDB and Texture Primitives"
    pub fn set_input_skin_vdb_and_texture_primitives(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Skin VDB and Texture Primitives" and specifies the output index of the target node.
    pub fn set_input_skin_vdb_and_texture_primitives_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Feather Templates"
    pub fn set_input_feather_templates(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Feather Templates" and specifies the output index of the target node.
    pub fn set_input_feather_templates_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_mask_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("mask{}", index1),
            crate::core::types::ParamValue::Float(val),
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

    // --- Float2 parameters ---
    pub fn with_maskinrange_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.base.params.insert(
            format!("maskinrange{}", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_maskinrange_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("maskinrange{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskoutrange_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.base.params.insert(
            format!("maskoutrange{}", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_maskoutrange_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("maskoutrange{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_maskoverride_inst(
        mut self,
        index1: usize,
        val: SopFeathertemplateassignMaskoverride,
    ) -> Self {
        self.base.params.insert(
            format!("maskoverride{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskoverride_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("maskoverride{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskoptions_inst(
        mut self,
        index1: usize,
        val: SopFeathertemplateassignMaskoptions,
    ) -> Self {
        self.base.params.insert(
            format!("maskoptions{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskoptions_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("maskoptions{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_maskremapramp_inst(
        mut self,
        index1: usize,
        val: Vec<crate::core::types::RampPoint>,
    ) -> Self {
        self.base.params.insert(
            format!("maskremapramp{}", index1),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_maskremapramp_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("maskremapramp{}", index1),
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
    pub fn with_seedattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "seedattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_seedattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seedattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_template_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("template{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_template_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("template{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskcurveattrib_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("maskcurveattrib{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maskcurveattrib_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("maskcurveattrib{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskattrib_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("maskattrib{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maskattrib_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("maskattrib{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_masktexture_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("masktexture{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_masktexture_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("masktexture{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_masktextureprim_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("masktextureprim{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_masktextureprim_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("masktextureprim{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_samplefromweights(mut self, val: bool) -> Self {
        self.base.params.insert(
            "samplefromweights".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_samplefromweights_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "samplefromweights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useseedattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useseedattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useseedattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useseedattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeathertemplateassign {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "feathertemplateassign"
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
pub struct SopFeathertemplatefromshape {
    pub base: crate::core::types::NodeBase,
}

impl SopFeathertemplatefromshape {
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

    /// Connects to input 0: "Shape Curves"
    pub fn set_input_shape_curves(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Shape Curves" and specifies the output index of the target node.
    pub fn set_input_shape_curves_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_rachiswidthroot(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rachiswidthroot".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rachiswidthroot_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rachiswidthroot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rachiswidthtip(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rachiswidthtip".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rachiswidthtip_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rachiswidthtip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shapebarbstart(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shapebarbstart".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shapebarbstart_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shapebarbstart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftdensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shaftdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shaftdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbdensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "barbdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barbdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_barbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_barbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_side(mut self, val: &str) -> Self {
        self.base.params.insert(
            "side".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_side_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "side".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_normalize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "normalize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normalize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "normalize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_movetoorigin(mut self, val: bool) -> Self {
        self.base.params.insert(
            "movetoorigin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_movetoorigin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "movetoorigin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_firstprofilefromshape(mut self, val: bool) -> Self {
        self.base.params.insert(
            "firstprofilefromshape".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_firstprofilefromshape_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "firstprofilefromshape".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addbarbuv(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addbarbuv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addbarbuv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addbarbuv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "setname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "setname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setside(mut self, val: bool) -> Self {
        self.base.params.insert(
            "setside".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setside_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "setside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeathertemplatefromshape {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "feathertemplatefromshape"
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
pub enum SopFeathertemplateinterpolateBlendoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathertemplateinterpolateBlendoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathertemplateinterpolateBlenduvmode {
    SkinUv = 0,
    CurveUv = 1,
    FeatherUv = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathertemplateinterpolateLookupmethod {
    Group = 0,
    MatchByNameAttribute = 1,
    WeightArrays = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathertemplateinterpolateResmode {
    Constant = 0,
    Adaptive = 1,
    MatchTemplate = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeathertemplateinterpolateBarbsegmode {
    Constant = 0,
    MatchTemplate = 1,
}

#[derive(Debug, Clone)]
pub struct SopFeathertemplateinterpolate {
    pub base: crate::core::types::NodeBase,
}

impl SopFeathertemplateinterpolate {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Feather Templates"
    pub fn set_input_feather_templates(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Feather Templates" and specifies the output index of the target node.
    pub fn set_input_feather_templates_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_seglength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "seglength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seglength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seglength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resmult(mut self, val: f32) -> Self {
        self.base.params.insert(
            "resmult".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_resmult_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resmult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_blendinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "blendinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_blendinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "blendoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_blendoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_shaftbasesegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftbasesegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftbasesegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbasesegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftbarbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftbarbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftbarbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbarbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftminsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftminsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftminsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftminsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftminbarbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "shaftminbarbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shaftminbarbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftminbarbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_barbsegs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_blendoverride(mut self, val: SopFeathertemplateinterpolateBlendoverride) -> Self {
        self.base.params.insert(
            "blendoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendoptions(mut self, val: SopFeathertemplateinterpolateBlendoptions) -> Self {
        self.base.params.insert(
            "blendoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blenduvmode(mut self, val: SopFeathertemplateinterpolateBlenduvmode) -> Self {
        self.base.params.insert(
            "blenduvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blenduvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blenduvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupmethod(mut self, val: SopFeathertemplateinterpolateLookupmethod) -> Self {
        self.base.params.insert(
            "lookupmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resmode(mut self, val: SopFeathertemplateinterpolateResmode) -> Self {
        self.base.params.insert(
            "resmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbsegmode(mut self, val: SopFeathertemplateinterpolateBarbsegmode) -> Self {
        self.base.params.insert(
            "barbsegmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_barbsegmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbsegmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_blendremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "blendremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_blendremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendremapramp".to_string(),
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
    pub fn with_blendcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "blendcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blendcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "blendattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blendattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendtexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "blendtexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blendtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendtextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "blendtextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blendtextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendtextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "specgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_specgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "specgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "matchattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matchattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "matchattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_namesattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "namesattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_namesattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "namesattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weightsattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "weightsattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_weightsattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "weightsattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftbasesegsattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftbasesegsattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftbasesegsattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbasesegsattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbattribs".to_string(),
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
    pub fn with_redistribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "redistribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_redistribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "redistribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useshaftbasesegsattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useshaftbasesegsattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useshaftbasesegsattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useshaftbasesegsattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbmirror(mut self, val: bool) -> Self {
        self.base.params.insert(
            "barbmirror".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_barbmirror_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbmirror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interpolateuv(mut self, val: bool) -> Self {
        self.base.params.insert(
            "interpolateuv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_interpolateuv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "interpolateuv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeathertemplateinterpolate {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "feathertemplateinterpolate"
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
pub struct SopFeathertemplateinterpolatecore {
    pub base: crate::core::types::NodeBase,
}

impl SopFeathertemplateinterpolatecore {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Feather Templates"
    pub fn set_input_feather_templates(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Feather Templates" and specifies the output index of the target node.
    pub fn set_input_feather_templates_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Reference Feathers"
    pub fn set_input_reference_feathers(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Reference Feathers" and specifies the output index of the target node.
    pub fn set_input_reference_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_indicesattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "indicesattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indicesattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "indicesattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weightsattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "weightsattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_weightsattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "weightsattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftbasesegsattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftbasesegsattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftbasesegsattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftbasesegsattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourceuattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sourceuattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourceuattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourceuattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeathertemplateinterpolatecore {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "feathertemplateinterpolatecore"
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
pub struct SopFeatheruncondense {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatheruncondense {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
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
    pub fn with_normuvattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "normuvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normuvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "normuvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barblgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barblgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barblgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barblgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbrgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbrgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbrgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbrgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primgroups(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primgroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primgroups_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointgroups(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pointgroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointgroups_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbattribsets(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbattribsets".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbattribsets_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbattribsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_createnormuv(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createnormuv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createnormuv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createnormuv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createbarbgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createbarbgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createbarbgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createbarbgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createshaftgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createshaftgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createshaftgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createshaftgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createshaftptattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createshaftptattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createshaftptattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createshaftptattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createid(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatheruncondense {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featheruncondense"
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
pub struct SopFeatherutility {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherutility {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
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
    pub fn with_firstbarbptattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "firstbarbptattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_firstbarbptattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "firstbarbptattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_widthattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "widthattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_widthattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "widthattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "orientattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_orientattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orientattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_info(mut self, val: &str) -> Self {
        self.base.params.insert(
            "info".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_info_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "info".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_firstbarbpt(mut self, val: bool) -> Self {
        self.base.params.insert(
            "firstbarbpt".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_firstbarbpt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "firstbarbpt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computewidth(mut self, val: bool) -> Self {
        self.base.params.insert(
            "computewidth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computewidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "computewidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_widthlengthnorm(mut self, val: bool) -> Self {
        self.base.params.insert(
            "widthlengthnorm".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_widthlengthnorm_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "widthlengthnorm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alignorienttotangent(mut self, val: bool) -> Self {
        self.base.params.insert(
            "alignorienttotangent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alignorienttotangent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "alignorienttotangent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pinbarbs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pinbarbs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pinbarbs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pinbarbs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherutility {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherutility"
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
pub enum SopFeathervisualizeBarbmode {
    Hide = 0,
    Curve = 1,
    Surface = 2,
}

#[derive(Debug, Clone)]
pub struct SopFeathervisualize {
    pub base: crate::core::types::NodeBase,
}

impl SopFeathervisualize {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "VDB"
    pub fn set_input_vdb(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "VDB" and specifies the output index of the target node.
    pub fn set_input_vdb_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_barbmode(mut self, val: SopFeathervisualizeBarbmode) -> Self {
        self.base.params.insert(
            "barbmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_barbmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeathervisualize {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "feathervisualize"
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
pub enum SopFeatherwidthShaftwidthoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherwidthShaftwidthoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherwidthShaftwidthuvmode {
    SkinUv = 0,
    CurveUv = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherwidthBarbwidthoverride {
    NoOverride = 0,
    GuideAttribute = 1,
    SkinAttribute = 2,
    Texture = 3,
    TexturePrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherwidthBarbwidthoptions {
    Fit = 0,
    Ramp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFeatherwidthBarbwidthuvmode {
    SkinUv = 0,
    CurveUv = 1,
    FeatherUv = 2,
}

#[derive(Debug, Clone)]
pub struct SopFeatherwidth {
    pub base: crate::core::types::NodeBase,
}

impl SopFeatherwidth {
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

    /// Connects to input 0: "Feathers"
    pub fn set_input_feathers(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feathers" and specifies the output index of the target node.
    pub fn set_input_feathers_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Skin"
    pub fn set_input_skin(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Skin" and specifies the output index of the target node.
    pub fn set_input_skin_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Skin VDB and Texture Primitives"
    pub fn set_input_skin_vdb_and_texture_primitives(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Skin VDB and Texture Primitives" and specifies the output index of the target node.
    pub fn set_input_skin_vdb_and_texture_primitives_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_shaftwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "shaftwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shaftwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "barbwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_barbwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_shaftwidthinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "shaftwidthinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_shaftwidthinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "shaftwidthoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_shaftwidthoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbwidthinrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "barbwidthinrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_barbwidthinrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbwidthinrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbwidthoutrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "barbwidthoutrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_barbwidthoutrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbwidthoutrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_shaftwidthoverride(mut self, val: SopFeatherwidthShaftwidthoverride) -> Self {
        self.base.params.insert(
            "shaftwidthoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftwidthoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthoptions(mut self, val: SopFeatherwidthShaftwidthoptions) -> Self {
        self.base.params.insert(
            "shaftwidthoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftwidthoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthuvmode(mut self, val: SopFeatherwidthShaftwidthuvmode) -> Self {
        self.base.params.insert(
            "shaftwidthuvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shaftwidthuvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthuvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbwidthoverride(mut self, val: SopFeatherwidthBarbwidthoverride) -> Self {
        self.base.params.insert(
            "barbwidthoverride".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_barbwidthoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbwidthoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbwidthoptions(mut self, val: SopFeatherwidthBarbwidthoptions) -> Self {
        self.base.params.insert(
            "barbwidthoptions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_barbwidthoptions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbwidthoptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbwidthuvmode(mut self, val: SopFeatherwidthBarbwidthuvmode) -> Self {
        self.base.params.insert(
            "barbwidthuvmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_barbwidthuvmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbwidthuvmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_shaftwidthremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "shaftwidthremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_shaftwidthremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthremapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbwidthremapramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "barbwidthremapramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_barbwidthremapramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbwidthremapramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_shaftwidthcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftwidthcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftwidthcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftwidthattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftwidthattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthtexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftwidthtexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftwidthtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shaftwidthtextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "shaftwidthtextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shaftwidthtextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shaftwidthtextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbwidthcurveattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbwidthcurveattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbwidthcurveattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbwidthcurveattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbwidthattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbwidthattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbwidthattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbwidthattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbwidthtexture(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbwidthtexture".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbwidthtexture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbwidthtexture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbwidthtextureprim(mut self, val: &str) -> Self {
        self.base.params.insert(
            "barbwidthtextureprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_barbwidthtextureprim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbwidthtextureprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skinuvattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "skinuvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skinuvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinuvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_featheruvattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "featheruvattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_featheruvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "featheruvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_createshaftwidth(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createshaftwidth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createshaftwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createshaftwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createbarbwidth(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createbarbwidth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createbarbwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createbarbwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_barbwidthmapramptobarbs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "barbwidthmapramptobarbs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_barbwidthmapramptobarbs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "barbwidthmapramptobarbs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFeatherwidth {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "featherwidth"
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
pub enum SopFemdeformType {
    MeshQuality = 0,
    Inversion = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFemdeformInversionpositiontype {
    Material = 0,
    Current = 1,
}

#[derive(Debug, Clone)]
pub struct SopFemdeform {
    pub base: crate::core::types::NodeBase,
}

impl SopFemdeform {
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

    /// Connects to input 0: "Solid Geometry"
    pub fn set_input_solid_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Solid Geometry" and specifies the output index of the target node.
    pub fn set_input_solid_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Deformed Target Surface"
    pub fn set_input_deformed_target_surface(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Deformed Target Surface" and specifies the output index of the target node.
    pub fn set_input_deformed_target_surface_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Rest Surface"
    pub fn set_input_rest_surface(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Rest Surface" and specifies the output index of the target node.
    pub fn set_input_rest_surface_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
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
    pub fn with_evaluationframe(mut self, val: f32) -> Self {
        self.base.params.insert(
            "evaluationframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_evaluationframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "evaluationframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "solidstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "solidstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distancethreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "distancethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distancethreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "distancethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attachstrength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "attachstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attachstrength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attachstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attachdamping(mut self, val: f32) -> Self {
        self.base.params.insert(
            "attachdamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attachdamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attachdamping".to_string(),
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
    pub fn with_exteriorsurfacealpha(mut self, val: f32) -> Self {
        self.base.params.insert(
            "exteriorsurfacealpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exteriorsurfacealpha_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exteriorsurfacealpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_exteriorsurfacecolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "exteriorsurfacecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_exteriorsurfacecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exteriorsurfacecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: SopFemdeformType) -> Self {
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
    pub fn with_inversionpositiontype(mut self, val: SopFemdeformInversionpositiontype) -> Self {
        self.base.params.insert(
            "inversionpositiontype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inversionpositiontype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inversionpositiontype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_restattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "restattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restattrib".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_ignorestiffness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "ignorestiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ignorestiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ignorestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attachtolocalspace(mut self, val: bool) -> Self {
        self.base.params.insert(
            "attachtolocalspace".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attachtolocalspace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attachtolocalspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedistancethreshold(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usedistancethreshold".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedistancethreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usedistancethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableviz(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableviz".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableviz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableviz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_qualityisolate(mut self, val: bool) -> Self {
        self.base.params.insert(
            "qualityisolate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_qualityisolate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "qualityisolate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inversionisolate(mut self, val: bool) -> Self {
        self.base.params.insert(
            "inversionisolate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inversionisolate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inversionisolate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayexteriorsurface(mut self, val: bool) -> Self {
        self.base.params.insert(
            "displayexteriorsurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayexteriorsurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displayexteriorsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFemdeform {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "femdeform"
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
pub enum SopFemvalidateType {
    MeshQuality = 0,
    Inversion = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFemvalidateInversionpositiontype {
    Material = 0,
    Current = 1,
}

#[derive(Debug, Clone)]
pub struct SopFemvalidate {
    pub base: crate::core::types::NodeBase,
}

impl SopFemvalidate {
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
    pub fn with_clipdist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "clipdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clipdist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clipdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_cliporigin(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "cliporigin".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cliporigin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cliporigin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipdir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "clipdir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_clipdir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clipdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: SopFemvalidateType) -> Self {
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
    pub fn with_inversionpositiontype(mut self, val: SopFemvalidateInversionpositiontype) -> Self {
        self.base.params.insert(
            "inversionpositiontype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inversionpositiontype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inversionpositiontype".to_string(),
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
    pub fn with_qualityisolate(mut self, val: bool) -> Self {
        self.base.params.insert(
            "qualityisolate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_qualityisolate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "qualityisolate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inversionisolate(mut self, val: bool) -> Self {
        self.base.params.insert(
            "inversionisolate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inversionisolate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inversionisolate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFemvalidate {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "femvalidate"
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
pub enum SopFibergroomStrokeTool {
    FiberStrokes = 0,
    Blur = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFibergroomStrokeProjtype {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
    ScreenPlane = 3,
    Geometry = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFibergroomExternalcurveid {
    GroomFibersByProximity = 0,
    GroomByMatchingMuscleid = 1,
}

#[derive(Debug, Clone)]
pub struct SopFibergroom {
    pub base: crate::core::types::NodeBase,
}

impl SopFibergroom {
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

    /// Connects to input 0: "Muscles"
    pub fn set_input_muscles(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Muscles" and specifies the output index of the target node.
    pub fn set_input_muscles_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Curves (Optional)"
    pub fn set_input_curves_optional(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Curves (Optional)" and specifies the output index of the target node.
    pub fn set_input_curves_optional_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_recacheinput(mut self) -> Self {
        self.base.params.insert(
            "recacheinput".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_clearallstrokes(mut self) -> Self {
        self.base.params.insert(
            "clearallstrokes".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Data parameters ---
    pub fn with_strokedata(mut self, val: &str) -> Self {
        self.base.params.insert(
            "strokedata".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_strokedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "strokedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_groomeddata(mut self, val: &str) -> Self {
        self.base.params.insert(
            "groomeddata".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_groomeddata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "groomeddata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_fiberguidescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fiberguidescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fiberguidescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fiberguidescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_radius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stroke_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_radius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_opacity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stroke_opacity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_opacity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_opacity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_radius_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("stroke{}_radius", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_radius_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_radius", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_opacity_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("stroke{}_opacity", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_opacity_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_opacity", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

    // --- Float3 parameters ---
    pub fn with_stroke_color(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "stroke_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_stroke_color_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_projcenter(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "stroke_projcenter".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_stroke_projcenter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_projcenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_color_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.base.params.insert(
            format!("stroke{}_color", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_stroke_color_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_color", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_projcenter_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.base.params.insert(
            format!("stroke{}_projcenter", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_stroke_projcenter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_projcenter", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_projdir_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.base.params.insert(
            format!("stroke{}_projdir", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_stroke_projdir_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_projdir", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_stroke_tool(mut self, val: SopFibergroomStrokeTool) -> Self {
        self.base.params.insert(
            "stroke_tool".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_stroke_tool_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_tool".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_projtype_inst(mut self, index1: usize, val: i32) -> Self {
        self.base.params.insert(
            format!("stroke{}_projtype", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stroke_projtype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_projtype", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_tool_inst(mut self, index1: usize, val: i32) -> Self {
        self.base.params.insert(
            format!("stroke{}_tool", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stroke_tool_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_tool", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalcurveid(mut self, val: SopFibergroomExternalcurveid) -> Self {
        self.base.params.insert(
            "externalcurveid".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_externalcurveid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "externalcurveid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_stroke_projtype(mut self, val: SopFibergroomStrokeProjtype) -> Self {
        self.base.params.insert(
            "stroke_projtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stroke_projtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_projtype".to_string(),
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
    pub fn with_pieceid(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pieceid".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pieceid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pieceid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_data_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_data", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stroke_data_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_data", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_metadata_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_metadata", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stroke_metadata_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_metadata", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axialrampattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "axialrampattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_axialrampattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "axialrampattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_fiberflow(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fiberflow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fiberflow_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fiberflow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_livestroke(mut self, val: bool) -> Self {
        self.base.params.insert(
            "livestroke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_livestroke_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "livestroke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("stroke{}_enable", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stroke_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_enable", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ignoreid(mut self, val: bool) -> Self {
        self.base.params.insert(
            "ignoreid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ignoreid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ignoreid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useaxialramp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useaxialramp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useaxialramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useaxialramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableaxialcorrection(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableaxialcorrection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableaxialcorrection_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableaxialcorrection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_legacyinitialization(mut self, val: bool) -> Self {
        self.base.params.insert(
            "legacyinitialization".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_legacyinitialization_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "legacyinitialization".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overridefibdir(mut self, val: bool) -> Self {
        self.base.params.insert(
            "overridefibdir".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overridefibdir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "overridefibdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesymmetry(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usesymmetry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesymmetry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usesymmetry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFibergroom {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "fibergroom"
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
pub struct SopFilamentAdvectPos {
    pub base: crate::core::types::NodeBase,
}

impl SopFilamentAdvectPos {
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

    /// Connects to input 0: "Vortex Filaments"
    pub fn set_input_vortex_filaments(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Vortex Filaments" and specifies the output index of the target node.
    pub fn set_input_vortex_filaments_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_timestep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "timestep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timestep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timestep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filstrength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "filstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_filstrength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filthickness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "filthickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_filthickness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reconnectdist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "reconnectdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reconnectdist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reconnectdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minedgelen(mut self, val: f32) -> Self {
        self.base.params.insert(
            "minedgelen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minedgelen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minedgelen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxedgelen(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxedgelen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxedgelen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxedgelen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_speedcap(mut self, val: f32) -> Self {
        self.base.params.insert(
            "speedcap".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_speedcap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "speedcap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dospeedcap(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dospeedcap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dospeedcap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dospeedcap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFilamentAdvectPos {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "filament_advect_pos"
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
pub enum SopFileFilemode {
    Automatic = 0,
    ReadFiles = 1,
    WriteFiles = 2,
    NoOperation = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFileMissingframe {
    ReportError = 0,
    NoGeometry = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFileLoadtype {
    AllGeometry = 0,
    InfoBoundingBox = 1,
    Info = 2,
    PointCloud = 3,
    PackedDiskPrimitive = 4,
    PackedDiskSequence = 5,
    PackedGeometry = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilePackedviewedit {
    UseFileSetting = 0,
    FullGeometry = 1,
    PointCloud = 2,
    BoundingBox = 3,
    Centroid = 4,
    Hidden = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFileViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFileWrap {
    Cycle = 0,
    Clamp = 1,
    Strict = 2,
    Mirror = 3,
}

#[derive(Debug, Clone)]
pub struct SopFile {
    pub base: crate::core::types::NodeBase,
}

impl SopFile {
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
    pub fn with_index(mut self, val: f32) -> Self {
        self.base.params.insert(
            "index".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_index_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "index".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_cachesize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_retry(mut self, val: i32) -> Self {
        self.base.params.insert(
            "retry".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_retry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "retry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_f(mut self, val: [i32; 2]) -> Self {
        self.base
            .params
            .insert("f".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_filemode(mut self, val: SopFileFilemode) -> Self {
        self.base.params.insert(
            "filemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_missingframe(mut self, val: SopFileMissingframe) -> Self {
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
    pub fn with_loadtype(mut self, val: SopFileLoadtype) -> Self {
        self.base.params.insert(
            "loadtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_loadtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loadtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedviewedit(mut self, val: SopFilePackedviewedit) -> Self {
        self.base.params.insert(
            "packedviewedit".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_packedviewedit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "packedviewedit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viewportlod(mut self, val: SopFileViewportlod) -> Self {
        self.base.params.insert(
            "viewportlod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viewportlod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "viewportlod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wrap(mut self, val: SopFileWrap) -> Self {
        self.base.params.insert(
            "wrap".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_wrap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "wrap".to_string(),
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
    pub fn with_objpattern(mut self, val: &str) -> Self {
        self.base.params.insert(
            "objpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objpattern_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "objpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geodatapath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "geodatapath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geodatapath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geodatapath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_packexpanded(mut self, val: bool) -> Self {
        self.base.params.insert(
            "packexpanded".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_packexpanded_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "packexpanded".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_delayload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "delayload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_delayload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "delayload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.base.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefetch(mut self, val: bool) -> Self {
        self.base.params.insert(
            "prefetch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefetch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prefetch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFile {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "file"
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
pub enum SopFilecacheFilemethod {
    Constructed = 0,
    Explicit = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilecacheFiletype {
    /// bgeo.sc
    BgeoSc = 0,
    Vdb = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilecacheTrange {
    SingleFrame = 0,
    FrameRange = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilecacheFmenu {
    AnimationRange = 0,
    PlaybarRange = 1,
    NoSubsteps = 2,
    /// 2 Substeps
    N2Substeps = 3,
    /// 4 Substeps
    N4Substeps = 4,
    /// 5 Substeps
    N5Substeps = 5,
    Every2ndFrame = 6,
    Every5thFrame = 7,
    Every10thFrame = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilecacheLoadtype {
    AllGeometry = 0,
    InfoBoundingBox = 1,
    Info = 2,
    PointCloud = 3,
    PackedDiskPrimitive = 4,
    PackedDiskSequence = 5,
    PackedGeometry = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilecachePackedviewedit {
    UseFileSetting = 0,
    FullGeometry = 1,
    PointCloud = 2,
    BoundingBox = 3,
    Centroid = 4,
    Hidden = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilecacheViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilecacheWrap {
    Cycle = 0,
    Clamp = 1,
    Strict = 2,
    Mirror = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilecacheMissingframe {
    ReportError = 0,
    NoGeometry = 1,
}

#[derive(Debug, Clone)]
pub struct SopFilecache {
    pub base: crate::core::types::NodeBase,
}

impl SopFilecache {
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

    /// Connects to input 0: "Geometry to Cache to Disk"
    pub fn set_input_geometry_to_cache_to_disk(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Cache to Disk" and specifies the output index of the target node.
    pub fn set_input_geometry_to_cache_to_disk_from(
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
    pub fn trigger_openfiledir(mut self) -> Self {
        self.base.params.insert(
            "openfiledir".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_openbasedir(mut self) -> Self {
        self.base.params.insert(
            "openbasedir".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_execute(mut self) -> Self {
        self.base.params.insert(
            "execute".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cookoutputnode(mut self) -> Self {
        self.base.params.insert(
            "cookoutputnode".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_dirtyall(mut self) -> Self {
        self.base.params.insert(
            "dirtyall".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_frameoverride(mut self, val: f32) -> Self {
        self.base.params.insert(
            "frameoverride".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frameoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "frameoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clampfirst(mut self, val: f32) -> Self {
        self.base.params.insert(
            "clampfirst".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clampfirst_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clampfirst".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clamplast(mut self, val: f32) -> Self {
        self.base.params.insert(
            "clamplast".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clamplast_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clamplast".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_index(mut self, val: f32) -> Self {
        self.base.params.insert(
            "index".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_index_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "index".to_string(),
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
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("f".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_version(mut self, val: i32) -> Self {
        self.base.params.insert(
            "version".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_version_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "version".to_string(),
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
    pub fn with_saveretry(mut self, val: i32) -> Self {
        self.base.params.insert(
            "saveretry".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_saveretry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "saveretry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loadretry(mut self, val: i32) -> Self {
        self.base.params.insert(
            "loadretry".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_loadretry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loadretry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachesize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_filemethod(mut self, val: SopFilecacheFilemethod) -> Self {
        self.base.params.insert(
            "filemethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filemethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filetype(mut self, val: SopFilecacheFiletype) -> Self {
        self.base.params.insert(
            "filetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filetype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trange(mut self, val: SopFilecacheTrange) -> Self {
        self.base.params.insert(
            "trange".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_fmenu(mut self, val: SopFilecacheFmenu) -> Self {
        self.base.params.insert(
            "fmenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fmenu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loadtype(mut self, val: SopFilecacheLoadtype) -> Self {
        self.base.params.insert(
            "loadtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_loadtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loadtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedviewedit(mut self, val: SopFilecachePackedviewedit) -> Self {
        self.base.params.insert(
            "packedviewedit".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_packedviewedit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "packedviewedit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viewportlod(mut self, val: SopFilecacheViewportlod) -> Self {
        self.base.params.insert(
            "viewportlod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viewportlod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "viewportlod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wrap(mut self, val: SopFilecacheWrap) -> Self {
        self.base.params.insert(
            "wrap".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_wrap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "wrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_missingframe(mut self, val: SopFilecacheMissingframe) -> Self {
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
    pub fn with_basename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "basename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_basename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "basename".to_string(),
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
    pub fn with_basedir(mut self, val: &str) -> Self {
        self.base.params.insert(
            "basedir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_basedir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "basedir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targettopnetwork(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targettopnetwork".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targettopnetwork_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targettopnetwork".to_string(),
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
    pub fn with_deleteattributes(mut self, val: &str) -> Self {
        self.base.params.insert(
            "deleteattributes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deleteattributes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deleteattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deletegroups(mut self, val: &str) -> Self {
        self.base.params.insert(
            "deletegroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deletegroups_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deletegroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_class_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("class{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_class_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("class{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribs_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("attribs{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribs_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("attribs{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_precision_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("precision{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_precision_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("precision{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_take(mut self, val: &str) -> Self {
        self.base.params.insert(
            "take".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "take".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "prerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lprerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "preframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "preframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lpreframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "postframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "postframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lpostframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postwrite(mut self, val: &str) -> Self {
        self.base.params.insert(
            "postwrite".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postwrite_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "postwrite".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostwrite(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lpostwrite".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostwrite_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lpostwrite".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "postrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "postrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lpostrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_framestr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "framestr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_framestr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "framestr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_versionstr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "versionstr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_versionstr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "versionstr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachedir(mut self, val: &str) -> Self {
        self.base.params.insert(
            "cachedir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cachedir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cachedir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "cachename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cachename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cachename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_descriptivelabel(mut self, val: &str) -> Self {
        self.base.params.insert(
            "descriptivelabel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_descriptivelabel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "descriptivelabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sopoutput(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sopoutput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sopoutput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sopoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
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
    pub fn with_timedependent(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timedependent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timedependent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timedependent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableversion(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableversion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableversion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableversion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachesim(mut self, val: bool) -> Self {
        self.base.params.insert(
            "cachesim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cachesim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cachesim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useframeoverride(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useframeoverride".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useframeoverride_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useframeoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doclampfirst(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doclampfirst".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doclampfirst_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doclampfirst".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doclamplast(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doclamplast".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doclamplast_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doclamplast".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packexpanded(mut self, val: bool) -> Self {
        self.base.params.insert(
            "packexpanded".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_packexpanded_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "packexpanded".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_delayload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "delayload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_delayload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "delayload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loadfromdiskonsave(mut self, val: bool) -> Self {
        self.base.params.insert(
            "loadfromdiskonsave".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadfromdiskonsave_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loadfromdiskonsave".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hardenbasename(mut self, val: bool) -> Self {
        self.base.params.insert(
            "hardenbasename".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hardenbasename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hardenbasename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.base.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initsim(mut self, val: bool) -> Self {
        self.base.params.insert(
            "initsim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_initsim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initsim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alfprogress(mut self, val: bool) -> Self {
        self.base.params.insert(
            "alfprogress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alfprogress_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "alfprogress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savebackground(mut self, val: bool) -> Self {
        self.base.params.insert(
            "savebackground".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savebackground_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "savebackground".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefetch(mut self, val: bool) -> Self {
        self.base.params.insert(
            "prefetch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefetch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prefetch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tprerender(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tprerender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tprerender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpreframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tpreframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpreframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostframe(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tpostframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostwrite(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tpostwrite".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostwrite_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tpostwrite".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostrender(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tpostrender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostrender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFilecache {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "filecache"
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
pub enum SopFilemergeMergemode {
    CustomRange = 0,
    ListOfIndices = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilemergeMissingframe {
    ReportError = 0,
    NoGeometry = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilemergeViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilemergePackedviewedit {
    UseFileSetting = 0,
    FullGeometry = 1,
    PointCloud = 2,
    BoundingBox = 3,
    Centroid = 4,
    Hidden = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilemergeWrap {
    Cycle = 0,
    Clamp = 1,
    Strict = 2,
    Mirror = 3,
}

#[derive(Debug, Clone)]
pub struct SopFilemerge {
    pub base: crate::core::types::NodeBase,
}

impl SopFilemerge {
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
    pub fn trigger_reload(mut self) -> Self {
        self.base
            .params
            .insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_selectfiles(mut self) -> Self {
        self.base.params.insert(
            "selectfiles".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_index(mut self, val: f32) -> Self {
        self.base.params.insert(
            "index".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_index_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "index".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_mergerange(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "mergerange".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_mergerange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mergerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_singleindex(mut self, val: i32) -> Self {
        self.base.params.insert(
            "singleindex".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_singleindex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "singleindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachesize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cachesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_retry(mut self, val: i32) -> Self {
        self.base.params.insert(
            "retry".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_retry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "retry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_f(mut self, val: [i32; 2]) -> Self {
        self.base
            .params
            .insert("f".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_mergemode(mut self, val: SopFilemergeMergemode) -> Self {
        self.base.params.insert(
            "mergemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mergemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mergemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_missingframe(mut self, val: SopFilemergeMissingframe) -> Self {
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
    pub fn with_loadtype(mut self, val: i32) -> Self {
        self.base.params.insert(
            "loadtype".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_loadtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loadtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viewportlod(mut self, val: SopFilemergeViewportlod) -> Self {
        self.base.params.insert(
            "viewportlod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viewportlod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "viewportlod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedviewedit(mut self, val: SopFilemergePackedviewedit) -> Self {
        self.base.params.insert(
            "packedviewedit".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_packedviewedit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "packedviewedit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wrap(mut self, val: SopFilemergeWrap) -> Self {
        self.base.params.insert(
            "wrap".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_wrap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "wrap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_filelist_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("filelist{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filelist_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("filelist{}", index1),
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
    pub fn with_merge_var(mut self, val: &str) -> Self {
        self.base.params.insert(
            "merge_var".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_merge_var_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "merge_var".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specificindices(mut self, val: &str) -> Self {
        self.base.params.insert(
            "specificindices".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_specificindices_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "specificindices".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filenameattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "filenameattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filenameattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filenameattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filepathattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "filepathattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filepathattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filepathattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fileindexattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fileindexattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fileindexattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fileindexattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablelist(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablelist".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablelist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablelist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_singlemode(mut self, val: bool) -> Self {
        self.base.params.insert(
            "singlemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_singlemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "singlemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loadfile_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("loadfile{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("loadfile{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerange(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablerange".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deletepoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "deletepoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deletepoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deletepoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packexpanded(mut self, val: bool) -> Self {
        self.base.params.insert(
            "packexpanded".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_packexpanded_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "packexpanded".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_delayload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "delayload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_delayload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "delayload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefetch(mut self, val: bool) -> Self {
        self.base.params.insert(
            "prefetch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefetch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "prefetch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setfilename(mut self, val: bool) -> Self {
        self.base.params.insert(
            "setfilename".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setfilename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "setfilename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setfilepath(mut self, val: bool) -> Self {
        self.base.params.insert(
            "setfilepath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setfilepath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "setfilepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expandpath(mut self, val: bool) -> Self {
        self.base.params.insert(
            "expandpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_expandpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "expandpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setfileindex(mut self, val: bool) -> Self {
        self.base.params.insert(
            "setfileindex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setfileindex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "setfileindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFilemerge {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "filemerge"
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
pub enum SopFilletFillet {
    AllPrimitives = 0,
    GroupsOfNPrimitives = 1,
    SkipEveryNthPrimitive = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilletDir {
    InU = 0,
    InV = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilletFillettype {
    Freeform = 0,
    Convex = 1,
    Circular = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFilletPrimtype {
    InputGeometryType = 0,
    Polygon = 1,
    Nurbs = 2,
    Bezier = 3,
}

#[derive(Debug, Clone)]
pub struct SopFillet {
    pub base: crate::core::types::NodeBase,
}

impl SopFillet {
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

    /// Connects to input 0: "Fillet source"
    pub fn set_input_fillet_source(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Fillet source" and specifies the output index of the target node.
    pub fn set_input_fillet_source_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Auxiliary source"
    pub fn set_input_auxiliary_source(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Auxiliary source" and specifies the output index of the target node.
    pub fn set_input_auxiliary_source_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float2 parameters ---
    pub fn with_leftuv(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "leftuv".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_leftuv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "leftuv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rightuv(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "rightuv".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_rightuv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rightuv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lrwidth(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "lrwidth".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_lrwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lrwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lrscale(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "lrscale".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_lrscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lrscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lroffset(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "lroffset".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_lroffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lroffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_inc(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("inc".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_inc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

    // --- Menu parameters ---
    pub fn with_fillet(mut self, val: SopFilletFillet) -> Self {
        self.base.params.insert(
            "fillet".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fillet_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fillet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dir(mut self, val: SopFilletDir) -> Self {
        self.base.params.insert(
            "dir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_fillettype(mut self, val: SopFilletFillettype) -> Self {
        self.base.params.insert(
            "fillettype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fillettype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fillettype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primtype(mut self, val: SopFilletPrimtype) -> Self {
        self.base.params.insert(
            "primtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_primtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primtype".to_string(),
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
    pub fn with_loop(mut self, val: bool) -> Self {
        self.base.params.insert(
            "loop".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seamless(mut self, val: bool) -> Self {
        self.base.params.insert(
            "seamless".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_seamless_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "seamless".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cut(mut self, val: bool) -> Self {
        self.base.params.insert(
            "cut".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cut_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cut".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFillet {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "fillet"
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
pub struct SopFinalizewaves {
    pub base: crate::core::types::NodeBase,
}

impl SopFinalizewaves {
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

    /// Connects to input 0: "Geometry Edited with Waveform"
    pub fn set_input_geometry_edited_with_waveform(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry Edited with Waveform" and specifies the output index of the target node.
    pub fn set_input_geometry_edited_with_waveform_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }
}

impl crate::core::types::HoudiniNode for SopFinalizewaves {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "finalizewaves"
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
pub enum SopFindinstancesQuicksetup {
    /// Quick Setups ↓
    QuickSetups = 0,
    CopyInstances = 1,
    ReplaceInstances = 2,
    MatchPrototypes = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFindinstancesMode {
    ExtractAndInstance = 0,
    ExtractPrototypes = 1,
    MatchPrototypes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFindinstancesShowcopies {
    Off = 0,
    ColorCopies = 1,
    ColorPerPrototype = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFindinstancesUsescale {
    MatchByScale = 0,
    AllowUniformScaling = 1,
    /// Allow Non-Uniform Scaling
    AllowNonMinusUniformScaling = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFindinstancesPivot {
    Origin = 0,
    Centroid = 1,
}

#[derive(Debug, Clone)]
pub struct SopFindinstances {
    pub base: crate::core::types::NodeBase,
}

impl SopFindinstances {
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

    /// Connects to input 1: "Prototypes"
    pub fn set_input_prototypes(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Prototypes" and specifies the output index of the target node.
    pub fn set_input_prototypes_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
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
    pub fn with_volumetolerance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "volumetolerance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumetolerance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "volumetolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tolerance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tolerance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tolerance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_color(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_color_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_showcopies(mut self, val: SopFindinstancesShowcopies) -> Self {
        self.base.params.insert(
            "showcopies".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_showcopies_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showcopies".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxprepass(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxprepass".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxprepass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxprepass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_quicksetup(mut self, val: SopFindinstancesQuicksetup) -> Self {
        self.base.params.insert(
            "quicksetup".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_quicksetup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "quicksetup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: SopFindinstancesMode) -> Self {
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
    pub fn with_usescale(mut self, val: SopFindinstancesUsescale) -> Self {
        self.base.params.insert(
            "usescale".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_usescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pivot(mut self, val: SopFindinstancesPivot) -> Self {
        self.base.params.insert(
            "pivot".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pivot_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pivot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_transferattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "transferattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transferattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "transferattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ptattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ptattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ptattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vtxattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vtxattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vtxattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vtxattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_cache(mut self, val: bool) -> Self {
        self.base.params.insert(
            "cache".to_string(),
            crate::core::types::ParamValue::Toggle(val),
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
    pub fn with_showlinks(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showlinks".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showlinks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showlinks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_areasort(mut self, val: bool) -> Self {
        self.base.params.insert(
            "areasort".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_areasort_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "areasort".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reorient(mut self, val: bool) -> Self {
        self.base.params.insert(
            "reorient".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reorient_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedfragments(mut self, val: bool) -> Self {
        self.base.params.insert(
            "packedfragments".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_packedfragments_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "packedfragments".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFindinstances {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "findinstances"
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
pub enum SopFindshortestpathMultiplicity {
    FromAnyStartToAnyEnd = 0,
    FromEachStartToAnyEnd = 1,
    FromAnyStartToEachEnd = 2,
    FromEachStartToEachEnd = 3,
    FromEachStartToCorrespondingEnd = 4,
}

#[derive(Debug, Clone)]
pub struct SopFindshortestpath {
    pub base: crate::core::types::NodeBase,
}

impl SopFindshortestpath {
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

    /// Connects to input 0: "Surface Geometry"
    pub fn set_input_surface_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Surface Geometry" and specifies the output index of the target node.
    pub fn set_input_surface_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Alternate Start/End Point Locations (optional)"
    pub fn set_input_alternate_start_end_point_locations_opti(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Alternate Start/End Point Locations (optional)" and specifies the output index of the target node.
    pub fn set_input_alternate_start_end_point_locations_opti_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_maxcost(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxcost".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxcost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxcost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customcost(mut self, val: f32) -> Self {
        self.base.params.insert(
            "customcost".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_customcost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "customcost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_heuristic(mut self, val: f32) -> Self {
        self.base.params.insert(
            "heuristic".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_heuristic_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "heuristic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_multiplicity(mut self, val: SopFindshortestpathMultiplicity) -> Self {
        self.base.params.insert(
            "multiplicity".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_multiplicity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "multiplicity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_startpts(mut self, val: &str) -> Self {
        self.base.params.insert(
            "startpts".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_startpts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "startpts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_endpts(mut self, val: &str) -> Self {
        self.base.params.insert(
            "endpts".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_endpts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "endpts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adjattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "adjattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_adjattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adjattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcost(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputcost".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputcost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputcost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputprevptnum(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputprevptnum".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputprevptnum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputprevptnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputptnum(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputptnum".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputptnum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputptnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathsgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pathsgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathsgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathsgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputstartpt(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputstartpt".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputstartpt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputstartpt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputendpt(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputendpt".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputendpt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputendpt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputpathcost(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputpathcost".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputpathcost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputpathcost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cost(mut self, val: &str) -> Self {
        self.base.params.insert(
            "cost".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primcost(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primcost".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primcost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primcost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angularcost(mut self, val: &str) -> Self {
        self.base.params.insert(
            "angularcost".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_angularcost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "angularcost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directedprims(mut self, val: &str) -> Self {
        self.base.params.insert(
            "directedprims".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_directedprims_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "directedprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_avoidedges(mut self, val: &str) -> Self {
        self.base.params.insert(
            "avoidedges".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_avoidedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "avoidedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enableadjattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableadjattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableadjattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableadjattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputpaths(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputpaths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputpaths_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputpaths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keep(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keep".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableoutputcost(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableoutputcost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputcost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableoutputcost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableoutputprevptnum(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableoutputprevptnum".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputprevptnum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableoutputprevptnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableoutputptnum(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableoutputptnum".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputptnum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableoutputptnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablepathsgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablepathsgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablepathsgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablepathsgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableoutputstartpt(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableoutputstartpt".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputstartpt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableoutputstartpt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableoutputendpt(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableoutputendpt".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputendpt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableoutputendpt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableoutputpathcost(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableoutputpathcost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputpathcost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableoutputpathcost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablemaxcost(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablemaxcost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemaxcost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablemaxcost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecost(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablecost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablecost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableprimcost(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableprimcost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableprimcost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableprimcost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_omitdistance(mut self, val: bool) -> Self {
        self.base.params.insert(
            "omitdistance".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_omitdistance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "omitdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_considerturning(mut self, val: bool) -> Self {
        self.base.params.insert(
            "considerturning".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_considerturning_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "considerturning".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableangularcost(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableangularcost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableangularcost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableangularcost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecustomcost(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablecustomcost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecustomcost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablecustomcost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableprimcustomcost(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableprimcustomcost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableprimcustomcost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableprimcustomcost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overrideheuristic(mut self, val: bool) -> Self {
        self.base.params.insert(
            "overrideheuristic".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideheuristic_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "overrideheuristic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableprimgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableprimgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableprimgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableprimgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledirectedprims(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabledirectedprims".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledirectedprims_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabledirectedprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableavoidedges(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableavoidedges".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableavoidedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableavoidedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFindshortestpath {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "findshortestpath"
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
pub enum SopFitMethod {
    Approximation = 0,
    Interpolation = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFitType {
    Nurbs = 0,
    Bezier = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFitSurftype {
    Rows = 0,
    Columns = 1,
    RowsAndColumns = 2,
    Triangles = 3,
    Quadrilaterals = 4,
    AlternatingTriangles = 5,
    ReverseTriangles = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFitScope {
    Global = 0,
    /// Local (Curves Only)
    LocalCurvesOnly = 1,
    Breakpoints = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFitDataparmu {
    Uniform = 0,
    ChordLength = 1,
    Centripetal = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFitDataparmv {
    Uniform = 0,
    ChordLength = 1,
    Centripetal = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFitCloseu {
    Off = 0,
    On = 1,
    IfPrimitiveDoes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFitClosev {
    Off = 0,
    On = 1,
    IfPrimitiveDoes = 2,
}

#[derive(Debug, Clone)]
pub struct SopFit {
    pub base: crate::core::types::NodeBase,
}

impl SopFit {
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
    pub fn with_smooth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "smooth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smooth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "smooth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
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
    pub fn with_method(mut self, val: SopFitMethod) -> Self {
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
    pub fn with_type(mut self, val: SopFitType) -> Self {
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
    pub fn with_surftype(mut self, val: SopFitSurftype) -> Self {
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
    pub fn with_scope(mut self, val: SopFitScope) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataparmu(mut self, val: SopFitDataparmu) -> Self {
        self.base.params.insert(
            "dataparmu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dataparmu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataparmu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataparmv(mut self, val: SopFitDataparmv) -> Self {
        self.base.params.insert(
            "dataparmv".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dataparmv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataparmv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closeu(mut self, val: SopFitCloseu) -> Self {
        self.base.params.insert(
            "closeu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_closev(mut self, val: SopFitClosev) -> Self {
        self.base.params.insert(
            "closev".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_multipleu(mut self, val: bool) -> Self {
        self.base.params.insert(
            "multipleu".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_multipleu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "multipleu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_multiplev(mut self, val: bool) -> Self {
        self.base.params.insert(
            "multiplev".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_multiplev_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "multiplev".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_corners(mut self, val: bool) -> Self {
        self.base.params.insert(
            "corners".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_corners_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "corners".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFit {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "fit"
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
pub enum SopFlattenEnablemaskattrib {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlattenOrient {
    Direction = 0,
    Rotation = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlattenOriginMode {
    SetUniform = 0,
    UseAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlattenDirMode {
    SetUniform = 0,
    UseAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlattenRotMode {
    SetUniform = 0,
    UseAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlattenFlattentype {
    ToSinglePlane = 0,
    BetweenPlanes = 1,
}

#[derive(Debug, Clone)]
pub struct SopFlatten {
    pub base: crate::core::types::NodeBase,
}

impl SopFlatten {
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

    /// Connects to input 0: "Points or Geometry"
    pub fn set_input_points_or_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Points or Geometry" and specifies the output index of the target node.
    pub fn set_input_points_or_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_mask(mut self, val: f32) -> Self {
        self.base.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Float(val),
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
    pub fn with_distmin(mut self, val: f32) -> Self {
        self.base.params.insert(
            "distmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distmin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "distmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distmax(mut self, val: f32) -> Self {
        self.base.params.insert(
            "distmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distmax_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "distmax".to_string(),
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
    pub fn with_rot(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rot".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rot_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_enablemaskattrib(mut self, val: SopFlattenEnablemaskattrib) -> Self {
        self.base.params.insert(
            "enablemaskattrib".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_enablemaskattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablemaskattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orient(mut self, val: SopFlattenOrient) -> Self {
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
    pub fn with_origin_mode(mut self, val: SopFlattenOriginMode) -> Self {
        self.base.params.insert(
            "origin_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_origin_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "origin_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dir_mode(mut self, val: SopFlattenDirMode) -> Self {
        self.base.params.insert(
            "dir_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dir_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dir_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rot_mode(mut self, val: SopFlattenRotMode) -> Self {
        self.base.params.insert(
            "rot_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rot_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rot_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattentype(mut self, val: SopFlattenFlattentype) -> Self {
        self.base.params.insert(
            "flattentype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_flattentype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "flattentype".to_string(),
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
    pub fn with_originattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "originattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_originattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "originattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dirattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dirattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dirattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dirattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rotattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rotattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rotattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thicknessattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "thicknessattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_thicknessattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "thicknessattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dodistmin(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dodistmin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodistmin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dodistmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodistmax(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dodistmax".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodistmax_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dodistmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usethickness(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usethickness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usethickness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usethickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFlatten {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "flatten"
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
pub enum SopFlipboundaryType {
    Source = 0,
    Sink = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlipboundaryBoundarytype {
    None = 0,
    Velocity = 1,
    Pressure = 2,
}

#[derive(Debug, Clone)]
pub struct SopFlipboundary {
    pub base: crate::core::types::NodeBase,
}

impl SopFlipboundary {
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

    /// Connects to input 0: "Sources"
    pub fn set_input_sources(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sources" and specifies the output index of the target node.
    pub fn set_input_sources_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Container"
    pub fn set_input_container(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Container" and specifies the output index of the target node.
    pub fn set_input_container_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Collisions"
    pub fn set_input_collisions(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Collisions" and specifies the output index of the target node.
    pub fn set_input_collisions_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Geometry to Source"
    pub fn set_input_geometry_to_source(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Geometry to Source" and specifies the output index of the target node.
    pub fn set_input_geometry_to_source_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_activate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pressure".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pressure_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pressure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hydro_pressure(mut self, val: f32) -> Self {
        self.base.params.insert(
            "hydro_pressure".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hydro_pressure_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hydro_pressure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hydro_offset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "hydro_offset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hydro_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hydro_offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressureband(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pressureband".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pressureband_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pressureband".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalevel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scalevel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scalevel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scalevel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normalvel(mut self, val: f32) -> Self {
        self.base.params.insert(
            "normalvel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normalvel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "normalvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_hydro_waterpos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "hydro_waterpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_hydro_waterpos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hydro_waterpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_velocity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: SopFlipboundaryType) -> Self {
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
    pub fn with_boundarytype(mut self, val: SopFlipboundaryBoundarytype) -> Self {
        self.base.params.insert(
            "boundarytype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_boundarytype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "boundarytype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_computevel(mut self, val: bool) -> Self {
        self.base.params.insert(
            "computevel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computevel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "computevel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFlipboundary {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "flipboundary"
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
pub struct SopFlipcollide {
    pub base: crate::core::types::NodeBase,
}

impl SopFlipcollide {
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

    /// Connects to input 0: "Sources"
    pub fn set_input_sources(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sources" and specifies the output index of the target node.
    pub fn set_input_sources_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Container"
    pub fn set_input_container(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Container" and specifies the output index of the target node.
    pub fn set_input_container_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Collisions"
    pub fn set_input_collisions(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Collisions" and specifies the output index of the target node.
    pub fn set_input_collisions_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "New Collision Geometry"
    pub fn set_input_new_collision_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "New Collision Geometry" and specifies the output index of the target node.
    pub fn set_input_new_collision_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_activate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computevelsubstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "computevelsubstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_computevelsubstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "computevelsubstep".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_computevel(mut self, val: bool) -> Self {
        self.base.params.insert(
            "computevel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computevel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "computevel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dovolume(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dovolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dovolume_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dovolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosurface(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dosurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dosurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFlipcollide {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "flipcollide"
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
pub enum SopFlipcontainerAttribrank {
    Scalar = 0,
    Vector = 1,
}

#[derive(Debug, Clone)]
pub struct SopFlipcontainer {
    pub base: crate::core::types::NodeBase,
}

impl SopFlipcontainer {
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

    /// Connects to input 0: "Geometry for Container"
    pub fn set_input_geometry_for_container(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry for Container" and specifies the output index of the target node.
    pub fn set_input_geometry_for_container_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_particlesep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlesep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gridscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gridscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gridscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gridscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gravity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gravity".to_string(),
            crate::core::types::ParamValue::Float(val),
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
    pub fn with_surfacetension(mut self, val: f32) -> Self {
        self.base.params.insert(
            "surfacetension".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacetension_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacetension".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stblurradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stblurradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stblurradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stblurradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_default_viscosity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "default_viscosity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_default_viscosity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "default_viscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slipscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "slipscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_slipscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slipscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vorticitypreserve(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vorticitypreserve".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vorticitypreserve_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vorticitypreserve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributescalarvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("attributescalarvalue{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attributescalarvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("attributescalarvalue{}", index1),
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
    pub fn with_gravitydir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gravitydir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gravitydir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gravitydir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributevectorvalue_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.base.params.insert(
            format!("attributevectorvalue{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_attributevectorvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("attributevectorvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_rest_framedelay(mut self, val: i32) -> Self {
        self.base.params.insert(
            "rest_framedelay".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_rest_framedelay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rest_framedelay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rest_frameoffset(mut self, val: i32) -> Self {
        self.base.params.insert(
            "rest_frameoffset".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_rest_frameoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rest_frameoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_attribrank_inst(mut self, index1: usize, val: SopFlipcontainerAttribrank) -> Self {
        self.base.params.insert(
            format!("attribrank{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_attribrank_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("attribrank{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_volumename_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("volumename{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_volumename_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("volumename{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_implicit(mut self, val: bool) -> Self {
        self.base.params.insert(
            "implicit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_implicit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "implicit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dovaryingdensity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dovaryingdensity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dovaryingdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dovaryingdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosurfacetension(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dosurfacetension".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosurfacetension_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dosurfacetension".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docurvatureblur(mut self, val: bool) -> Self {
        self.base.params.insert(
            "docurvatureblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docurvatureblur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "docurvatureblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "viscosity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "viscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dovaryingviscosity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dovaryingviscosity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dovaryingviscosity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dovaryingviscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableslip(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableslip".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableslip_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableslip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dovorticity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dovorticity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dovorticity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dovorticity".to_string(),
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
    pub fn with_doage(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doage_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dorelativedensity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dorelativedensity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorelativedensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dorelativedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dorest(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dorest".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorest_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dorest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rest_dual(mut self, val: bool) -> Self {
        self.base.params.insert(
            "rest_dual".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rest_dual_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rest_dual".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customvolumename_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("customvolumename{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_customvolumename_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("customvolumename{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFlipcontainer {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "flipcontainer"
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
pub enum SopFlipmodifyvolumeSurfaceinput {
    Geometry = 0,
    Volume = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlipmodifyvolumeAttributeinput {
    AutoInput = 0,
    Attribute = 1,
    Volume = 2,
    Constant = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlipmodifyvolumeAttributetype {
    AutoType = 0,
    Float = 1,
    Vector = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlipmodifyvolumeMode {
    Override = 0,
    Merge = 1,
}

#[derive(Debug, Clone)]
pub struct SopFlipmodifyvolume {
    pub base: crate::core::types::NodeBase,
}

impl SopFlipmodifyvolume {
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

    /// Connects to input 0: "Sources"
    pub fn set_input_sources(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sources" and specifies the output index of the target node.
    pub fn set_input_sources_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Source Geometry or Volumes"
    pub fn set_input_source_geometry_or_volumes(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Source Geometry or Volumes" and specifies the output index of the target node.
    pub fn set_input_source_geometry_or_volumes_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
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
    pub fn with_attributenormalvalue(mut self, val: f32) -> Self {
        self.base.params.insert(
            "attributenormalvalue".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attributenormalvalue_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attributenormalvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gradient_strength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gradient_strength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gradient_strength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gradient_strength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gradient_dist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gradient_dist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gradient_dist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gradient_dist".to_string(),
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

    // --- Float3 parameters ---
    pub fn with_attributevalue(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "attributevalue".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_attributevalue_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attributevalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gradient_center(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gradient_center".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gradient_center_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gradient_center".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gradient_dir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gradient_dir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gradient_dir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gradient_dir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_surfaceinput(mut self, val: SopFlipmodifyvolumeSurfaceinput) -> Self {
        self.base.params.insert(
            "surfaceinput".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surfaceinput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfaceinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributeinput(mut self, val: SopFlipmodifyvolumeAttributeinput) -> Self {
        self.base.params.insert(
            "attributeinput".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_attributeinput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attributeinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributetype(mut self, val: SopFlipmodifyvolumeAttributetype) -> Self {
        self.base.params.insert(
            "attributetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_attributetype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attributetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: SopFlipmodifyvolumeMode) -> Self {
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

    // --- String parameters ---
    pub fn with_surfacename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "surfacename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_surfacename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surfacename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volume(mut self, val: &str) -> Self {
        self.base.params.insert(
            "volume".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_volume_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "volume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "attributename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attributename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attributename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface(mut self, val: &str) -> Self {
        self.base.params.insert(
            "surface".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_surface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "surface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_gradient_clip(mut self, val: bool) -> Self {
        self.base.params.insert(
            "gradient_clip".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_gradient_clip_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gradient_clip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFlipmodifyvolume {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "flipmodifyvolume"
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
pub enum SopFlipsolverBoundarytype {
    VelocityDriven = 0,
    PressureDriven = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlipsolverCollision {
    None = 0,
    Particle = 1,
    MoveOutsideCollision = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlipsolverUseground {
    None = 0,
    GroundPlane = 1,
    HeightField = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlipsolverVeltransfer {
    /// FLIP (Splashy)
    FlipSplashy = 0,
    /// APIC (Swirly)
    ApicSwirly = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlipsolverDropletbehavior {
    BlendWithFluid = 0,
    KillOnDetection = 1,
    KillAtFluid = 2,
}

#[derive(Debug, Clone)]
pub struct SopFlipsolver {
    pub base: crate::core::types::NodeBase,
}

impl SopFlipsolver {
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

    /// Connects to input 0: "Sources"
    pub fn set_input_sources(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sources" and specifies the output index of the target node.
    pub fn set_input_sources_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Container"
    pub fn set_input_container(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Container" and specifies the output index of the target node.
    pub fn set_input_container_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Collisions"
    pub fn set_input_collisions(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Collisions" and specifies the output index of the target node.
    pub fn set_input_collisions_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Boundary Flow"
    pub fn set_input_boundary_flow(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Boundary Flow" and specifies the output index of the target node.
    pub fn set_input_boundary_flow_from(
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
    pub fn trigger_checkpoint_openbasedir(mut self) -> Self {
        self.base.params.insert(
            "checkpoint_openbasedir".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_particlesep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlesep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gridscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gridscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gridscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gridscale".to_string(),
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
    pub fn with_waterline(mut self, val: f32) -> Self {
        self.base.params.insert(
            "waterline".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_waterline_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "waterline".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressureband(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pressureband".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pressureband_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pressureband".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocityband(mut self, val: f32) -> Self {
        self.base.params.insert(
            "velocityband".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velocityband_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "velocityband".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourceband(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sourceband".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sourceband_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sourceband".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transparency(mut self, val: f32) -> Self {
        self.base.params.insert(
            "transparency".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_transparency_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "transparency".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extrapdist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "extrapdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_extrapdist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extrapdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stickscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stickscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickdist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stickdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stickdist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickcells(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stickcells".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stickcells_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickcells".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickbias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stickbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stickbias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sticknormalscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sticknormalscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sticknormalscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sticknormalscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sticktangentscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sticktangentscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sticktangentscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sticktangentscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smoothing(mut self, val: f32) -> Self {
        self.base.params.insert(
            "smoothing".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smoothing_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "smoothing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partsepamount(mut self, val: f32) -> Self {
        self.base.params.insert(
            "partsepamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_partsepamount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partsepamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partsepscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "partsepscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_partsepscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partsepscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dropletmindensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dropletmindensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dropletmindensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dropletmindensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dropletmaxdensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dropletmaxdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dropletmaxdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dropletmaxdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dropletvelblend(mut self, val: f32) -> Self {
        self.base.params.insert(
            "dropletvelblend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dropletvelblend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dropletvelblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_birththreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "birththreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_birththreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "birththreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deaththreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "deaththreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_deaththreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deaththreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oversampling(mut self, val: f32) -> Self {
        self.base.params.insert(
            "oversampling".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oversampling_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "oversampling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oversamplingbandwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "oversamplingbandwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oversamplingbandwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "oversamplingbandwidth".to_string(),
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
    pub fn with_vis_maxspeed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vis_maxspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vis_maxspeed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vis_maxspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_waterorigin(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "waterorigin".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_waterorigin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "waterorigin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_water_velocity(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "water_velocity".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_water_velocity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "water_velocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ground_pos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ground_pos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ground_pos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ground_pos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ground_up(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "ground_up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ground_up_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ground_up".to_string(),
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
    pub fn with_minimumsubsteps(mut self, val: i32) -> Self {
        self.base.params.insert(
            "minimumsubsteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minimumsubsteps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minimumsubsteps".to_string(),
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
    pub fn with_minairregionsize(mut self, val: i32) -> Self {
        self.base.params.insert(
            "minairregionsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minairregionsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minairregionsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partsepiter(mut self, val: i32) -> Self {
        self.base.params.insert(
            "partsepiter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_partsepiter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partsepiter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpoint_version(mut self, val: i32) -> Self {
        self.base.params.insert(
            "checkpoint_version".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_checkpoint_version_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "checkpoint_version".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_explicitcachensteps(mut self, val: i32) -> Self {
        self.base.params.insert(
            "explicitcachensteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_explicitcachensteps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "explicitcachensteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_explicitcachecheckpointspacing(mut self, val: i32) -> Self {
        self.base.params.insert(
            "explicitcachecheckpointspacing".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_explicitcachecheckpointspacing_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "explicitcachecheckpointspacing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_boundarytype(mut self, val: SopFlipsolverBoundarytype) -> Self {
        self.base.params.insert(
            "boundarytype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_boundarytype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "boundarytype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision(mut self, val: SopFlipsolverCollision) -> Self {
        self.base.params.insert(
            "collision".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useground(mut self, val: SopFlipsolverUseground) -> Self {
        self.base.params.insert(
            "useground".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_useground_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useground".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_veltransfer(mut self, val: SopFlipsolverVeltransfer) -> Self {
        self.base.params.insert(
            "veltransfer".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_veltransfer_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "veltransfer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dropletbehavior(mut self, val: SopFlipsolverDropletbehavior) -> Self {
        self.base.params.insert(
            "dropletbehavior".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dropletbehavior_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dropletbehavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_vis_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "vis_ramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_vis_ramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vis_ramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_stickcontrolfield(mut self, val: &str) -> Self {
        self.base.params.insert(
            "stickcontrolfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stickcontrolfield_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickcontrolfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ground_heightfield(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ground_heightfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ground_heightfield_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ground_heightfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpoint_basename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "checkpoint_basename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_checkpoint_basename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "checkpoint_basename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpoint_basedir(mut self, val: &str) -> Self {
        self.base.params.insert(
            "checkpoint_basedir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_checkpoint_basedir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "checkpoint_basedir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_overrideres(mut self, val: bool) -> Self {
        self.base.params.insert(
            "overrideres".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideres_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "overrideres".to_string(),
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
    pub fn with_initialsurface(mut self, val: bool) -> Self {
        self.base.params.insert(
            "initialsurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_initialsurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initialsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dowaterline(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dowaterline".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dowaterline_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dowaterline".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetransparency(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usetransparency".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetransparency_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usetransparency".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablestick(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablestick".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablestick_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablestick".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickusemaxdist(mut self, val: bool) -> Self {
        self.base.params.insert(
            "stickusemaxdist".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickusemaxdist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickusemaxdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickusemaxcells(mut self, val: bool) -> Self {
        self.base.params.insert(
            "stickusemaxcells".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickusemaxcells_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickusemaxcells".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickusecontrolfield(mut self, val: bool) -> Self {
        self.base.params.insert(
            "stickusecontrolfield".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickusecontrolfield_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stickusecontrolfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doincompressibleair(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doincompressibleair".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doincompressibleair_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doincompressibleair".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_applycollisionstoair(mut self, val: bool) -> Self {
        self.base.params.insert(
            "applycollisionstoair".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_applycollisionstoair_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "applycollisionstoair".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partsep(mut self, val: bool) -> Self {
        self.base.params.insert(
            "partsep".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_partsep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partsep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodroplets(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dodroplets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodroplets_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dodroplets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doreseeding(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doreseeding".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doreseeding_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doreseeding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oversamplebounds(mut self, val: bool) -> Self {
        self.base.params.insert(
            "oversamplebounds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_oversamplebounds_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "oversamplebounds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showcollision(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showcollision".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showcollision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showcollision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_pointsasspheres(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vis_pointsasspheres".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vis_pointsasspheres_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vis_pointsasspheres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_colorbyspeed(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vis_colorbyspeed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vis_colorbyspeed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vis_colorbyspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_donarrowband(mut self, val: bool) -> Self {
        self.base.params.insert(
            "donarrowband".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_donarrowband_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "donarrowband".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemgpreconditioner(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usemgpreconditioner".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemgpreconditioner_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usemgpreconditioner".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useadaptivepressure(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useadaptivepressure".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useadaptivepressure_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useadaptivepressure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useadaptiveviscosity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useadaptiveviscosity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useadaptiveviscosity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useadaptiveviscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_explicitcache(mut self, val: bool) -> Self {
        self.base.params.insert(
            "explicitcache".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_explicitcache_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "explicitcache".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpoint_enableversion(mut self, val: bool) -> Self {
        self.base.params.insert(
            "checkpoint_enableversion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_checkpoint_enableversion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "checkpoint_enableversion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFlipsolver {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "flipsolver"
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

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("dopnet1/FORCES")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait SopFlipsolverInnerExt {
    fn force(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn source(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn volume(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> SopFlipsolverInnerExt for crate::core::graph::InnerGraph<'a, SopFlipsolver> {
    fn force(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("dopnet1/FORCES/FORCE")
    }
    fn source(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("dopnet1/FORCES/SOURCE")
    }
    fn volume(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("dopnet1/FORCES/VOLUME")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlipsourceInitialize {
    SourceFlip = 0,
    Sink = 1,
    SinkFlip = 2,
    Collision = 3,
    Pump = 4,
    Expand = 5,
}

#[derive(Debug, Clone)]
pub struct SopFlipsource {
    pub base: crate::core::types::NodeBase,
}

impl SopFlipsource {
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

    /// Connects to input 0: "Geometry to Source From"
    pub fn set_input_geometry_to_source_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Source From" and specifies the output index of the target node.
    pub fn set_input_geometry_to_source_from_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_fogboost(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fogboost".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fogboost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fogboost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particlesep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlesep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitterseed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "jitterseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jitterseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "jitterseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitterscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "jitterscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jitterscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "jitterscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oversampling(mut self, val: f32) -> Self {
        self.base.params.insert(
            "oversampling".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oversampling_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "oversampling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oversamplingbandwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "oversamplingbandwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oversamplingbandwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "oversamplingbandwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_velocity(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_velocity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restoffset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "restoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_restoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_initialize(mut self, val: SopFlipsourceInitialize) -> Self {
        self.base.params.insert(
            "initialize".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_initialize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initialize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_volumename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "volumename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_volumename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "volumename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particlegroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "particlegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_particlegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "particlegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pscale(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pscale".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_shell(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shell".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shell_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shell".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputfog(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputfog".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputfog_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputfog".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remapfog(mut self, val: bool) -> Self {
        self.base.params.insert(
            "remapfog".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remapfog_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remapfog".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createparticles(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createparticles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createparticles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createparticles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dooversampling(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dooversampling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dooversampling_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dooversampling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addpscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addpscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addpscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addpscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addvelocity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addvelocity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addvelocity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addvelocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerest(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablerest".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerest_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablerest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dualrestattributes(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dualrestattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dualrestattributes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dualrestattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFlipsource {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "flipsource"
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
pub enum SopFlipvolumecombineFieldSurfaceres {
    LowResolution = 0,
    HighResolution = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlipvolumecombineFieldVelres {
    LowResolution = 0,
    HighResolution = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlipvolumecombineFieldPresres {
    LowResolution = 0,
    HighResolution = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFlipvolumecombineExtrapolateMode {
    /// Manual (fast)
    ManualFast = 0,
    /// Automatic (slow)
    AutomaticSlow = 1,
}

#[derive(Debug, Clone)]
pub struct SopFlipvolumecombine {
    pub base: crate::core::types::NodeBase,
}

impl SopFlipvolumecombine {
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

    /// Connects to input 0: "Low-Resolution Fields"
    pub fn set_input_low_resolution_fields(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Low-Resolution Fields" and specifies the output index of the target node.
    pub fn set_input_low_resolution_fields_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "High-Resolution Fields"
    pub fn set_input_high_resolution_fields(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "High-Resolution Fields" and specifies the output index of the target node.
    pub fn set_input_high_resolution_fields_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "High-Resolution Container"
    pub fn set_input_high_resolution_container(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "High-Resolution Container" and specifies the output index of the target node.
    pub fn set_input_high_resolution_container_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Clipping Bounding Box"
    pub fn set_input_clipping_bounding_box(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Clipping Bounding Box" and specifies the output index of the target node.
    pub fn set_input_clipping_bounding_box_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_extrapolate_halfwidthworld(mut self, val: f32) -> Self {
        self.base.params.insert(
            "extrapolate_halfwidthworld".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_extrapolate_halfwidthworld_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extrapolate_halfwidthworld".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_field_surfmaskwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "field_surfmaskwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_field_surfmaskwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "field_surfmaskwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_field_surfblursize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "field_surfblursize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_field_surfblursize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "field_surfblursize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_field_blendrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "field_blendrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_field_blendrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "field_blendrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_field_surfbluriter(mut self, val: i32) -> Self {
        self.base.params.insert(
            "field_surfbluriter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_field_surfbluriter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "field_surfbluriter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_field_surfaceres(mut self, val: SopFlipvolumecombineFieldSurfaceres) -> Self {
        self.base.params.insert(
            "field_surfaceres".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_field_surfaceres_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "field_surfaceres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_field_velres(mut self, val: SopFlipvolumecombineFieldVelres) -> Self {
        self.base.params.insert(
            "field_velres".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_field_velres_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "field_velres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_field_presres(mut self, val: SopFlipvolumecombineFieldPresres) -> Self {
        self.base.params.insert(
            "field_presres".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_field_presres_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "field_presres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extrapolate_mode(mut self, val: SopFlipvolumecombineExtrapolateMode) -> Self {
        self.base.params.insert(
            "extrapolate_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_extrapolate_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extrapolate_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_field_blendramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "field_blendramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_field_blendramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "field_blendramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_field_surfacelrhw(mut self, val: bool) -> Self {
        self.base.params.insert(
            "field_surfacelrhw".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_field_surfacelrhw_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "field_surfacelrhw".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extrapolate_enable(mut self, val: bool) -> Self {
        self.base.params.insert(
            "extrapolate_enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_extrapolate_enable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extrapolate_enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_field_surfblur(mut self, val: bool) -> Self {
        self.base.params.insert(
            "field_surfblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_field_surfblur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "field_surfblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFlipvolumecombine {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "flipvolumecombine"
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
pub enum SopFluidcompressDopack {
    Points = 0,
    PackedPoints = 1,
    VdbPoints = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFluidcompressPoscompression {
    None = 0,
    /// 16-bit Fixed Point
    N16MinusBitFixedPoint = 1,
    /// 8-bit Fixed Point
    N8MinusBitFixedPoint = 2,
}

#[derive(Debug, Clone)]
pub struct SopFluidcompress {
    pub base: crate::core::types::NodeBase,
}

impl SopFluidcompress {
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

    /// Connects to input 0: "Particles and Volumes"
    pub fn set_input_particles_and_volumes(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Particles and Volumes" and specifies the output index of the target node.
    pub fn set_input_particles_and_volumes_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Container"
    pub fn set_input_container(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Container" and specifies the output index of the target node.
    pub fn set_input_container_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Collisions"
    pub fn set_input_collisions(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Collisions" and specifies the output index of the target node.
    pub fn set_input_collisions_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_particlesep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlesep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumeband(mut self, val: f32) -> Self {
        self.base.params.insert(
            "volumeband".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumeband_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "volumeband".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velresolutionratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "velresolutionratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velresolutionratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "velresolutionratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_advecttime(mut self, val: f32) -> Self {
        self.base.params.insert(
            "advecttime".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_advecttime_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "advecttime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_advectcfl(mut self, val: f32) -> Self {
        self.base.params.insert(
            "advectcfl".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_advectcfl_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "advectcfl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minspeed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "minspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minspeed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_particleband(mut self, val: i32) -> Self {
        self.base.params.insert(
            "particleband".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_particleband_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "particleband".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_dopack(mut self, val: SopFluidcompressDopack) -> Self {
        self.base.params.insert(
            "dopack".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dopack_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopack".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_poscompression(mut self, val: SopFluidcompressPoscompression) -> Self {
        self.base.params.insert(
            "poscompression".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_poscompression_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "poscompression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_particlekeepattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "particlekeepattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_particlekeepattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "particlekeepattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compressto16bit(mut self, val: &str) -> Self {
        self.base.params.insert(
            "compressto16bit".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_compressto16bit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "compressto16bit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_doparticleband(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doparticleband".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doparticleband_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doparticleband".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doparticlekeepattribs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doparticlekeepattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doparticlekeepattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doparticlekeepattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docompressto16bit(mut self, val: bool) -> Self {
        self.base.params.insert(
            "docompressto16bit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docompressto16bit_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "docompressto16bit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dovolumeband(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dovolumeband".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dovolumeband_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dovolumeband".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumevdbhalf(mut self, val: bool) -> Self {
        self.base.params.insert(
            "volumevdbhalf".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_volumevdbhalf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "volumevdbhalf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keeppressure(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keeppressure".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppressure_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keeppressure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFluidcompress {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "fluidcompress"
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
pub enum SopFluidsourceMethod {
    BuildSdfFromGeometry = 0,
    StampPoints = 1,
    SampleVolumes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFluidsourceVolvis {
    Smoke = 0,
    Isosurface = 1,
    Slice = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFluidsourceSlicePlane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFluidsourceSliceType {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFluidsourceSliceMethod {
    Volume = 0,
    Mesh = 1,
    Points = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFluidsourceNoiseMode {
    Additive = 0,
    Multiplicative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFluidsourceCellMode {
    Additive = 0,
    Multiplicative = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFluidsourceVelMethod {
    StampPoints = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFluidsourceVelVolvis {
    Smoke = 0,
    Streamers = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFluidsourceVelStreamPlaneOrient {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFluidsourcePointmethod {
    DenseGrid = 0,
    SparseVolume = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFluidsourceInitialize {
    SourceSmoke = 0,
    SourceFuel = 1,
    SourceFluid = 2,
    SourceFlip = 3,
    Sink = 4,
    SinkFluid = 5,
    Collision = 6,
    Pump = 7,
    Velocity = 8,
    Expand = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFluidsourcePartritionType {
    Primitives = 0,
    Points = 1,
}

#[derive(Debug, Clone)]
pub struct SopFluidsource {
    pub base: crate::core::types::NodeBase,
}

impl SopFluidsource {
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

    /// Connects to input 0: "Geometry to Turn into a Volume Fluid Source"
    pub fn set_input_geometry_to_turn_into_a_volume_fluid_sou(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Turn into a Volume Fluid Source" and specifies the output index of the target node.
    pub fn set_input_geometry_to_turn_into_a_volume_fluid_sou_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_volumescale_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("volumescale{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumescale_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("volumescale{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "divsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "divsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volborderval(mut self, val: f32) -> Self {
        self.base.params.insert(
            "volborderval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volborderval_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "volborderval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slice_planeoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "slice_planeoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_slice_planeoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slice_planeoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slice_iso_offset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "slice_iso_offset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_slice_iso_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slice_iso_offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eloc(mut self, val: f32) -> Self {
        self.base.params.insert(
            "eloc".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eloc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "eloc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_size(mut self, val: f32) -> Self {
        self.base.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float(val),
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
    pub fn with_edge_thickness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "edge_thickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_edge_thickness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "edge_thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_in_feather_length(mut self, val: f32) -> Self {
        self.base.params.insert(
            "in_feather_length".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_in_feather_length_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "in_feather_length".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bandwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bandwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bandwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bandwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smoothness2(mut self, val: f32) -> Self {
        self.base.params.insert(
            "smoothness2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smoothness2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "smoothness2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_feather(mut self, val: f32) -> Self {
        self.base.params.insert(
            "feather".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_feather_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "feather".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointsdf_edge_location(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pointsdf_edge_location".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pointsdf_edge_location_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointsdf_edge_location".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_destmin(mut self, val: f32) -> Self {
        self.base.params.insert(
            "destmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_destmin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "destmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_voronoi_influence(mut self, val: f32) -> Self {
        self.base.params.insert(
            "voronoi_influence".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_voronoi_influence_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "voronoi_influence".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pulse_duration(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pulse_duration".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pulse_duration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pulse_duration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharpness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "sharpness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sharpness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharpness".to_string(),
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
    pub fn with_grain(mut self, val: f32) -> Self {
        self.base.params.insert(
            "grain".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_grain_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "grain".to_string(),
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
    pub fn with_element_size(mut self, val: f32) -> Self {
        self.base.params.insert(
            "element_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_element_size_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "element_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cell_thresh(mut self, val: f32) -> Self {
        self.base.params.insert(
            "cell_thresh".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cell_thresh_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cell_thresh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cell_offset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "cell_offset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cell_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cell_offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cell_harshness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "cell_harshness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cell_harshness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cell_harshness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cell_min(mut self, val: f32) -> Self {
        self.base.params.insert(
            "cell_min".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cell_min_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cell_min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cell_max(mut self, val: f32) -> Self {
        self.base.params.insert(
            "cell_max".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cell_max_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cell_max".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cell_size(mut self, val: f32) -> Self {
        self.base.params.insert(
            "cell_size".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cell_size_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cell_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frames_to_blur(mut self, val: f32) -> Self {
        self.base.params.insert(
            "frames_to_blur".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frames_to_blur_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "frames_to_blur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blur_offset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blur_offset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blur_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blur_offset".to_string(),
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
    pub fn with_vel_volumescale_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("vel_volumescale{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_volumescale_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("vel_volumescale{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_divsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_divsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_divsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_divsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_volborderval(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_volborderval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_volborderval_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_volborderval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_stream_plane_pos(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_stream_plane_pos".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_stream_plane_pos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_stream_plane_pos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_stream_length(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_stream_length".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_stream_length_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_stream_length".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_stream_separation(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_stream_separation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_stream_separation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_stream_separation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_stream_max(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_stream_max".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_stream_max_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_stream_max".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_pointextrapdist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_pointextrapdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_pointextrapdist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_pointextrapdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_turbscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_turbscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_turbscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_turbscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_turbswirl(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_turbswirl".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_turbswirl_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_turbswirl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_turbrough(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_turbrough".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_turbrough_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_turbrough".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_turboffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_turboffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_turboffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_turboffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_turbatten(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_turbatten".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_turbatten_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_turbatten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_turbpulselength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_turbpulselength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_turbpulselength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_turbpulselength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_turbframeoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_turbframeoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_turbframeoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_turbframeoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_vortexspeed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_vortexspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_vortexspeed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_vortexspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_vortexradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_vortexradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_vortexradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_vortexradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_objscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_objscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_objscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_objscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_objframe(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_objframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_objframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_objframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_objsample(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vel_objsample".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_objsample_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_objsample".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particlesep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlesep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitterseed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "jitterseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jitterseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "jitterseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitterscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "jitterscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jitterscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "jitterscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scatter(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scatter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scatter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scatter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oversampling(mut self, val: f32) -> Self {
        self.base.params.insert(
            "oversampling".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oversampling_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "oversampling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oversamplingbandwidth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "oversamplingbandwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oversamplingbandwidth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "oversamplingbandwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partviscosity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "partviscosity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_partviscosity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partviscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partdensity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "partdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_partdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bound_expansion(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bound_expansion".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bound_expansion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bound_expansion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_slice_range(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "slice_range".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_slice_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slice_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_element_scale(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "element_scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_element_scale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "element_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cell_scale(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "cell_scale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cell_scale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cell_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_uniformvel(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vel_uniformvel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_uniformvel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_uniformvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_vortexpivot(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vel_vortexpivot".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_vortexpivot_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_vortexpivot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_vortexdir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vel_vortexdir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_vortexdir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_vortexdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rest_offset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rest_offset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rest_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rest_offset".to_string(),
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
    pub fn with_geometry_samples(mut self, val: i32) -> Self {
        self.base.params.insert(
            "geometry_samples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_geometry_samples_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geometry_samples".to_string(),
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
    pub fn with_vel_turboctaves(mut self, val: i32) -> Self {
        self.base.params.insert(
            "vel_turboctaves".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vel_turboctaves_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_turboctaves".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: SopFluidsourceMethod) -> Self {
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
    pub fn with_volvis_inst(mut self, index1: usize, val: SopFluidsourceVolvis) -> Self {
        self.base.params.insert(
            format!("volvis{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_volvis_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("volvis{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slice_plane(mut self, val: SopFluidsourceSlicePlane) -> Self {
        self.base.params.insert(
            "slice_plane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_slice_plane_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slice_plane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slice_type(mut self, val: SopFluidsourceSliceType) -> Self {
        self.base.params.insert(
            "slice_type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_slice_type_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slice_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slice_method(mut self, val: SopFluidsourceSliceMethod) -> Self {
        self.base.params.insert(
            "slice_method".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_slice_method_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "slice_method".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noise_mode(mut self, val: SopFluidsourceNoiseMode) -> Self {
        self.base.params.insert(
            "noise_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_noise_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "noise_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cell_mode(mut self, val: SopFluidsourceCellMode) -> Self {
        self.base.params.insert(
            "cell_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cell_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cell_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_method(mut self, val: SopFluidsourceVelMethod) -> Self {
        self.base.params.insert(
            "vel_method".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vel_method_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_method".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_volvis_inst(mut self, index1: usize, val: SopFluidsourceVelVolvis) -> Self {
        self.base.params.insert(
            format!("vel_volvis{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vel_volvis_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("vel_volvis{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_stream_plane_orient(mut self, val: SopFluidsourceVelStreamPlaneOrient) -> Self {
        self.base.params.insert(
            "vel_stream_plane_orient".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vel_stream_plane_orient_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_stream_plane_orient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointmethod(mut self, val: SopFluidsourcePointmethod) -> Self {
        self.base.params.insert(
            "pointmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pointmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initialize(mut self, val: SopFluidsourceInitialize) -> Self {
        self.base.params.insert(
            "initialize".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_initialize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "initialize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partrition_type(mut self, val: SopFluidsourcePartritionType) -> Self {
        self.base.params.insert(
            "partrition_type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_partrition_type_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partrition_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_feather_in_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "feather_in_ramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_feather_in_ramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "feather_in_ramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_feather_out_ramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "feather_out_ramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_feather_out_ramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "feather_out_ramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_vortexramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "vel_vortexramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_vel_vortexramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_vortexramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_source_attribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "source_attribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_source_attribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "source_attribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_source_attribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vel_source_attribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vel_source_attribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_source_attribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("vel_name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vel_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("vel_name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_objpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vel_objpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vel_objpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_objpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particlesource(mut self, val: &str) -> Self {
        self.base.params.insert(
            "particlesource".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_particlesource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "particlesource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partition_attribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "partition_attribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partition_attribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partition_attribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_make_sdf(mut self, val: bool) -> Self {
        self.base.params.insert(
            "make_sdf".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_make_sdf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "make_sdf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visibility_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("visibility{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visibility_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("visibility{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_invert_sign(mut self, val: bool) -> Self {
        self.base.params.insert(
            "invert_sign".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invert_sign_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "invert_sign".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minimum_distance(mut self, val: bool) -> Self {
        self.base.params.insert(
            "minimum_distance".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_minimum_distance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minimum_distance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale_by_source_attribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "scale_by_source_attribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_scale_by_source_attribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scale_by_source_attribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_e_interior(mut self, val: bool) -> Self {
        self.base.params.insert(
            "e_interior".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_e_interior_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "e_interior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remap_feather_in(mut self, val: bool) -> Self {
        self.base.params.insert(
            "remap_feather_in".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remap_feather_in_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remap_feather_in".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remap_feather_out(mut self, val: bool) -> Self {
        self.base.params.insert(
            "remap_feather_out".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remap_feather_out_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remap_feather_out".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_generate_particle_sdf(mut self, val: bool) -> Self {
        self.base.params.insert(
            "generate_particle_sdf".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_generate_particle_sdf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "generate_particle_sdf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale_particle_sdf(mut self, val: bool) -> Self {
        self.base.params.insert(
            "scale_particle_sdf".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_scale_particle_sdf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scale_particle_sdf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_noise(mut self, val: bool) -> Self {
        self.base.params.insert(
            "use_noise".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_noise_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "use_noise".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animate_noise(mut self, val: bool) -> Self {
        self.base.params.insert(
            "animate_noise".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animate_noise_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "animate_noise".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_invert_cells(mut self, val: bool) -> Self {
        self.base.params.insert(
            "invert_cells".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invert_cells_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "invert_cells".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adaptive_cells(mut self, val: bool) -> Self {
        self.base.params.insert(
            "adaptive_cells".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adaptive_cells_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adaptive_cells".to_string(),
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
    pub fn with_vel_visibility_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("vel_visibility{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_visibility_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("vel_visibility{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_add_uniform(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_add_uniform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_add_uniform_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_add_uniform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_invert_sign(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_invert_sign".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_invert_sign_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_invert_sign".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_stream_use_plane(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_stream_use_plane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_stream_use_plane_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_stream_use_plane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_add_curl_noise(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_add_curl_noise".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_add_curl_noise_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_add_curl_noise".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_turbtimedep(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_turbtimedep".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_turbtimedep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_turbtimedep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_add_vortex(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_add_vortex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_add_vortex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_add_vortex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_limitvortex(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_limitvortex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_limitvortex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_limitvortex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_objapply(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vel_objapply".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_objapply_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel_objapply".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createparticles(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createparticles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createparticles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createparticles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doscatter(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doscatter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doscatter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doscatter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dooversampling(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dooversampling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dooversampling_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dooversampling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addpartviscosity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addpartviscosity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addpartviscosity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addpartviscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addpartdensity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addpartdensity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addpartdensity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addpartdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_rest(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enable_rest".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_rest_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enable_rest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dual_rest_attributes(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dual_rest_attributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dual_rest_attributes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dual_rest_attributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_auto_bounds(mut self, val: bool) -> Self {
        self.base.params.insert(
            "auto_bounds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_auto_bounds_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "auto_bounds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_partitioning(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enable_partitioning".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_partitioning_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enable_partitioning".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFluidsource {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "fluidsource"
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
pub enum SopFontType {
    BezierCurvesAndPolygons = 0,
    BeziersOnly = 1,
    PolygonsOnly = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFontHalign {
    Left = 0,
    Center = 1,
    Right = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFontValign {
    FirstLine = 0,
    Top = 1,
    Middle = 2,
    Bottom = 3,
}

#[derive(Debug, Clone)]
pub struct SopFont {
    pub base: crate::core::types::NodeBase,
}

impl SopFont {
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
    pub fn with_fontsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fontsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fontsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fontsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oblique(mut self, val: f32) -> Self {
        self.base.params.insert(
            "oblique".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oblique_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "oblique".to_string(),
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
    pub fn with_tracking(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "tracking".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_tracking_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tracking".to_string(),
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
    pub fn with_type(mut self, val: SopFontType) -> Self {
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
    pub fn with_halign(mut self, val: SopFontHalign) -> Self {
        self.base.params.insert(
            "halign".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_halign_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "halign".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_valign(mut self, val: SopFontValign) -> Self {
        self.base.params.insert(
            "valign".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_valign_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "valign".to_string(),
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
    pub fn with_text(mut self, val: &str) -> Self {
        self.base.params.insert(
            "text".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_text_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "text".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_use_descender(mut self, val: bool) -> Self {
        self.base.params.insert(
            "use_descender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_descender_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "use_descender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_autokern(mut self, val: bool) -> Self {
        self.base.params.insert(
            "autokern".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autokern_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "autokern".to_string(),
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
    pub fn with_addattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFont {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "font"
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
pub struct SopForce {
    pub base: crate::core::types::NodeBase,
}

impl SopForce {
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
    pub fn with_radial(mut self, val: f32) -> Self {
        self.base.params.insert(
            "radial".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radial_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "radial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axial(mut self, val: f32) -> Self {
        self.base.params.insert(
            "axial".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_axial_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "axial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vortex(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vortex".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vortex_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vortex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spiral(mut self, val: f32) -> Self {
        self.base.params.insert(
            "spiral".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spiral_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "spiral".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
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

    // --- Toggle parameters ---
    pub fn with_doradial(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doradial".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doradial_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doradial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doaxis(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doaxis".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doaxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopForce {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "force"
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
pub enum SopForeachFortype {
    EachGroup = 0,
    EachAttributeValue = 1,
    EachNumber = 2,
    /// Each Primitive/Point
    EachPrimitivePoint = 3,
}

#[derive(Debug, Clone)]
pub struct SopForeach {
    pub base: crate::core::types::NodeBase,
}

impl SopForeach {
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

    /// Connects to input 0: "Input to Detect Groups/Attributes from"
    pub fn set_input_input_to_detect_groups_attributes_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input to Detect Groups/Attributes from" and specifies the output index of the target node.
    pub fn set_input_input_to_detect_groups_attributes_from_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Auxiliary Input"
    pub fn set_input_auxiliary_input(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Auxiliary Input" and specifies the output index of the target node.
    pub fn set_input_auxiliary_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Auxiliary Input"
    pub fn set_input_auxiliary_input_1(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Auxiliary Input" and specifies the output index of the target node.
    pub fn set_input_auxiliary_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Auxiliary Input"
    pub fn set_input_auxiliary_input_2(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Auxiliary Input" and specifies the output index of the target node.
    pub fn set_input_auxiliary_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_attribtol(mut self, val: f32) -> Self {
        self.base.params.insert(
            "attribtol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attribtol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attribtol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_numrange(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "numrange".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_numrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "numrange".to_string(),
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
    pub fn with_stopcondition(mut self, val: i32) -> Self {
        self.base.params.insert(
            "stopcondition".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stopcondition_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stopcondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_fortype(mut self, val: SopForeachFortype) -> Self {
        self.base.params.insert(
            "fortype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fortype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fortype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_forstamp(mut self, val: &str) -> Self {
        self.base.params.insert(
            "forstamp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_forstamp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "forstamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_foridxstamp(mut self, val: &str) -> Self {
        self.base.params.insert(
            "foridxstamp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_foridxstamp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "foridxstamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_groupmask(mut self, val: &str) -> Self {
        self.base.params.insert(
            "groupmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_groupmask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "groupmask".to_string(),
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

    // --- Toggle parameters ---
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
    pub fn with_mergeresults(mut self, val: bool) -> Self {
        self.base.params.insert(
            "mergeresults".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mergeresults_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mergeresults".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eachpoint(mut self, val: bool) -> Self {
        self.base.params.insert(
            "eachpoint".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_eachpoint_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "eachpoint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopForeach {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "foreach"
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
pub trait SopForeachInnerExt {
    fn each1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> SopForeachInnerExt for crate::core::graph::InnerGraph<'a, SopForeach> {
    fn each1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("each1")
    }
}

#[derive(Debug, Clone)]
pub struct SopFractal {
    pub base: crate::core::types::NodeBase,
}

impl SopFractal {
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

    /// Connects to input 0: "Source data"
    pub fn set_input_source_data(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Source data" and specifies the output index of the target node.
    pub fn set_input_source_data_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_smooth(mut self, val: f32) -> Self {
        self.base.params.insert(
            "smooth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smooth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "smooth".to_string(),
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

    // --- Float3 parameters ---
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
    pub fn with_divs(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("divs".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_divs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "divs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_nmlattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "nmlattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nmlattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "nmlattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_fixed(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fixed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fixed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fixed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vtxnms(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vtxnms".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vtxnms_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vtxnms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFractal {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "fractal"
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
pub enum SopFrankenmuscleGrouptype {
    GuessFromGroup = 0,
    Breakpoints = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone)]
pub struct SopFrankenmuscle {
    pub base: crate::core::types::NodeBase,
}

impl SopFrankenmuscle {
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

    /// Connects to input 0: "Solid Muscles"
    pub fn set_input_solid_muscles(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Solid Muscles" and specifies the output index of the target node.
    pub fn set_input_solid_muscles_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Muscle ID Source"
    pub fn set_input_muscle_id_source(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Muscle ID Source" and specifies the output index of the target node.
    pub fn set_input_muscle_id_source_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_maxdist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxdist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "falloff".to_string(),
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

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopFrankenmuscleGrouptype) -> Self {
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
    pub fn with_isolateid(mut self, val: &str) -> Self {
        self.base.params.insert(
            "isolateid".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_isolateid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "isolateid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keepmuscleid(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keepmuscleid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepmuscleid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keepmuscleid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_isolatemuscle(mut self, val: bool) -> Self {
        self.base.params.insert(
            "isolatemuscle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_isolatemuscle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "isolatemuscle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFrankenmuscle {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "frankenmuscle"
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
pub enum SopFrankenmusclepaintStrokeAttribtype {
    Color = 0,
    Float = 1,
    Integer = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFrankenmusclepaintLmboperation {
    PaintFg = 0,
    Smooth = 1,
    Erase = 2,
    SampleFg = 3,
    PaintBg = 4,
    SampleBg = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFrankenmusclepaintLmboperation2 {
    PaintFg = 0,
    Smooth = 1,
    Erase = 2,
    SampleFg = 3,
    PaintBg = 4,
    SampleBg = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFrankenmusclepaintLmboperation3 {
    PaintFg = 0,
    Smooth = 1,
    Erase = 2,
    SampleFg = 3,
    PaintBg = 4,
    SampleBg = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFrankenmusclepaintMmboperation {
    PaintFg = 0,
    Smooth = 1,
    Erase = 2,
    SampleFg = 3,
    PaintBg = 4,
    SampleBg = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFrankenmusclepaintMmboperation2 {
    PaintFg = 0,
    Smooth = 1,
    Erase = 2,
    SampleFg = 3,
    PaintBg = 4,
    SampleBg = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFrankenmusclepaintMmboperation3 {
    PaintFg = 0,
    Smooth = 1,
    Erase = 2,
    SampleFg = 3,
    PaintBg = 4,
    SampleBg = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFrankenmusclepaintStrokeOperation {
    PaintFg = 0,
    Smooth = 1,
    Erase = 2,
    SampleFg = 3,
    PaintBg = 4,
    SampleBg = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFrankenmusclepaintStrokePaintmode {
    Over = 0,
    Add = 1,
    Maximum = 2,
    Minimum = 3,
    Multiply = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFrankenmusclepaintStrokeShape {
    Volume = 0,
    Surface = 1,
    Screen = 2,
    Fill = 3,
    NearestPoint = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFrankenmusclepaintRecachemethod {
    OriginalValues = 0,
    /// Re-Send Rays
    ReMinusSendRays = 1,
    PrimitiveUv = 2,
    TextureUv = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFrankenmusclepaintStrokeProjtype {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
    ScreenPlane = 3,
    Geometry = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFrankenmusclepaintAttribtype {
    Color = 0,
    Float = 1,
    Integer = 2,
}

#[derive(Debug, Clone)]
pub struct SopFrankenmusclepaint {
    pub base: crate::core::types::NodeBase,
}

impl SopFrankenmusclepaint {
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

    /// Connects to input 0: "Geometry to Paint"
    pub fn set_input_geometry_to_paint(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Paint" and specifies the output index of the target node.
    pub fn set_input_geometry_to_paint_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_reset(mut self) -> Self {
        self.base
            .params
            .insert("reset".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_recache(mut self) -> Self {
        self.base.params.insert(
            "recache".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_erasestrokes(mut self) -> Self {
        self.base.params.insert(
            "erasestrokes".to_string(),
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
    pub fn with_strokegeo(mut self, val: &str) -> Self {
        self.base.params.insert(
            "strokegeo".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_strokegeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "strokegeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bakedgeo(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bakedgeo".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_bakedgeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bakedgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_isectgeo(mut self, val: &str) -> Self {
        self.base.params.insert(
            "isectgeo".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_isectgeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "isectgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activestrokegeo(mut self, val: &str) -> Self {
        self.base.params.insert(
            "activestrokegeo".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_activestrokegeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activestrokegeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unsavedbakedgeo(mut self, val: &str) -> Self {
        self.base.params.insert(
            "unsavedbakedgeo".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_unsavedbakedgeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unsavedbakedgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_fgfloat(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fgfloat".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fgfloat_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fgfloat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bgfloat(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bgfloat".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bgfloat_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bgfloat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_radius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stroke_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_radius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_screensize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stroke_screensize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_screensize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_screensize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_opacity(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stroke_opacity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_opacity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_opacity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_softedge(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stroke_softedge".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_softedge_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_softedge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visibilitybias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "visibilitybias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_visibilitybias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "visibilitybias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_opacitypressure(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stroke_opacitypressure".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_opacitypressure_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_opacitypressure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_radiuspressure(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stroke_radiuspressure".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_radiuspressure_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_radiuspressure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_float(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stroke_float".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_float_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_float".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_radius_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("stroke{}_radius", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_radius_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_radius", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_opacity_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("stroke{}_opacity", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stroke_opacity_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_opacity", index1),
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
    pub fn with_mirror_t(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "mirror_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_mirror_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mirror_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mirror_dir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "mirror_dir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_mirror_dir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mirror_dir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_color(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "stroke_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_stroke_color_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_projcenter(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "stroke_projcenter".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_stroke_projcenter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_projcenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_color_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.base.params.insert(
            format!("stroke{}_color", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_stroke_color_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_color", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_projcenter_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.base.params.insert(
            format!("stroke{}_projcenter", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_stroke_projcenter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_projcenter", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_projdir_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.base.params.insert(
            format!("stroke{}_projdir", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_stroke_projdir_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_projdir", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_attribute(mut self, val: i32) -> Self {
        self.base.params.insert(
            "attribute".to_string(),
            crate::core::types::ParamValue::Int(val),
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
    pub fn with_fgint(mut self, val: i32) -> Self {
        self.base.params.insert(
            "fgint".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_fgint_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fgint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bgint(mut self, val: i32) -> Self {
        self.base.params.insert(
            "bgint".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bgint_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bgint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_tool(mut self, val: i32) -> Self {
        self.base.params.insert(
            "stroke_tool".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stroke_tool_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_tool".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_int(mut self, val: i32) -> Self {
        self.base.params.insert(
            "stroke_int".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stroke_int_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_int".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_tool_inst(mut self, index1: usize, val: i32) -> Self {
        self.base.params.insert(
            format!("stroke{}_tool", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stroke_tool_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_tool", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_projtype_inst(mut self, index1: usize, val: i32) -> Self {
        self.base.params.insert(
            format!("stroke{}_projtype", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stroke_projtype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_projtype", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_stroke_attribtype(mut self, val: SopFrankenmusclepaintStrokeAttribtype) -> Self {
        self.base.params.insert(
            "stroke_attribtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stroke_attribtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_attribtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lmboperation(mut self, val: SopFrankenmusclepaintLmboperation) -> Self {
        self.base.params.insert(
            "lmboperation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lmboperation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lmboperation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lmboperation2(mut self, val: SopFrankenmusclepaintLmboperation2) -> Self {
        self.base.params.insert(
            "lmboperation2".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lmboperation2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lmboperation2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lmboperation3(mut self, val: SopFrankenmusclepaintLmboperation3) -> Self {
        self.base.params.insert(
            "lmboperation3".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lmboperation3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lmboperation3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mmboperation(mut self, val: SopFrankenmusclepaintMmboperation) -> Self {
        self.base.params.insert(
            "mmboperation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mmboperation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mmboperation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mmboperation2(mut self, val: SopFrankenmusclepaintMmboperation2) -> Self {
        self.base.params.insert(
            "mmboperation2".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mmboperation2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mmboperation2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mmboperation3(mut self, val: SopFrankenmusclepaintMmboperation3) -> Self {
        self.base.params.insert(
            "mmboperation3".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mmboperation3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mmboperation3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_operation(mut self, val: SopFrankenmusclepaintStrokeOperation) -> Self {
        self.base.params.insert(
            "stroke_operation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stroke_operation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_operation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_paintmode(mut self, val: SopFrankenmusclepaintStrokePaintmode) -> Self {
        self.base.params.insert(
            "stroke_paintmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stroke_paintmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_paintmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_shape(mut self, val: SopFrankenmusclepaintStrokeShape) -> Self {
        self.base.params.insert(
            "stroke_shape".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stroke_shape_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_shape".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_recachemethod(mut self, val: SopFrankenmusclepaintRecachemethod) -> Self {
        self.base.params.insert(
            "recachemethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_recachemethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "recachemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_projtype(mut self, val: SopFrankenmusclepaintStrokeProjtype) -> Self {
        self.base.params.insert(
            "stroke_projtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stroke_projtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_projtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribtype(mut self, val: SopFrankenmusclepaintAttribtype) -> Self {
        self.base.params.insert(
            "attribtype_".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_attribtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attribtype_".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_stroke_group(mut self, val: &str) -> Self {
        self.base.params.insert(
            "stroke_group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stroke_group_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_attrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "stroke_attrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stroke_attrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_attrib".to_string(),
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
    pub fn with_strokegeofile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "strokegeofile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_strokegeofile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "strokegeofile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bakedgeofile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bakedgeofile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bakedgeofile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bakedgeofile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_data_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_data", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stroke_data_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_data", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_metadata_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_metadata", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stroke_metadata_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_metadata", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_displaygroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "displaygroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displaygroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displaygroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedisplay(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usedisplay".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedisplay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usedisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_connectivity(mut self, val: bool) -> Self {
        self.base.params.insert(
            "stroke_connectivity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stroke_connectivity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_connectivity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_frontface(mut self, val: bool) -> Self {
        self.base.params.insert(
            "stroke_frontface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stroke_frontface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_frontface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_visibility(mut self, val: bool) -> Self {
        self.base.params.insert(
            "stroke_visibility".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stroke_visibility_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stroke_visibility".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showpath(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savecache(mut self, val: bool) -> Self {
        self.base.params.insert(
            "savecache".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savecache_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "savecache".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_livemode(mut self, val: bool) -> Self {
        self.base.params.insert(
            "livemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_livemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "livemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docaching(mut self, val: bool) -> Self {
        self.base.params.insert(
            "docaching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docaching_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "docaching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domirror(mut self, val: bool) -> Self {
        self.base.params.insert(
            "domirror".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domirror_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "domirror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stroke_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("stroke{}_enable", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stroke_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("stroke{}_enable", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFrankenmusclepaint {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "frankenmusclepaint"
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
pub enum SopFurType {
    Polygon = 0,
    Nurbs = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFurSkinattribclass {
    Primitive = 0,
    Point = 1,
    Vertex = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFurGuideattribclass {
    Point = 0,
    Vertex = 1,
}

#[derive(Debug, Clone)]
pub struct SopFur {
    pub base: crate::core::types::NodeBase,
}

impl SopFur {
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

    /// Connects to input 1: "Guide Geometry"
    pub fn set_input_guide_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Guide Geometry" and specifies the output index of the target node.
    pub fn set_input_guide_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Clump Geometry"
    pub fn set_input_clump_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Clump Geometry" and specifies the output index of the target node.
    pub fn set_input_clump_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Parting Line Geometry"
    pub fn set_input_parting_line_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Parting Line Geometry" and specifies the output index of the target node.
    pub fn set_input_parting_line_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_display(mut self, val: f32) -> Self {
        self.base.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Float(val),
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
    pub fn with_guideradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "guideradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guideradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guideradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clumpradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "clumpradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clumpradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clumpradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partingradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "partingradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_partingradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partingradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_displayboundsmin(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "displayboundsmin".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_displayboundsmin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displayboundsmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayboundsmax(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "displayboundsmax".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_displayboundsmax_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displayboundsmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_segs(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("segs".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_segs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "segs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

    // --- Menu parameters ---
    pub fn with_type(mut self, val: SopFurType) -> Self {
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
    pub fn with_skinattribclass(mut self, val: SopFurSkinattribclass) -> Self {
        self.base.params.insert(
            "skinattribclass".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_skinattribclass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinattribclass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideattribclass(mut self, val: SopFurGuideattribclass) -> Self {
        self.base.params.insert(
            "guideattribclass".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guideattribclass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guideattribclass".to_string(),
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
    pub fn with_skinshader(mut self, val: &str) -> Self {
        self.base.params.insert(
            "skinshader".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skinshader_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skinshader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skintextureattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "skintextureattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skintextureattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skintextureattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideshader(mut self, val: &str) -> Self {
        self.base.params.insert(
            "guideshader".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guideshader_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guideshader".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidetextureattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "guidetextureattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guidetextureattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidetextureattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_limitdisplaytobounds(mut self, val: bool) -> Self {
        self.base.params.insert(
            "limitdisplaytobounds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitdisplaytobounds_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limitdisplaytobounds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useclosestclump(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useclosestclump".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useclosestclump_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useclosestclump".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removeunclumpedhairs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "removeunclumpedhairs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removeunclumpedhairs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "removeunclumpedhairs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removeunguidedhairs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "removeunguidedhairs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removeunguidedhairs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "removeunguidedhairs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setid(mut self, val: bool) -> Self {
        self.base.params.insert(
            "setid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "setid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFur {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "fur"
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
pub enum SopFuseSnaptype {
    NearPoints = 0,
    Grid = 1,
    SpecifiedPoints = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFuseAlgorithm {
    /// Least Target Point Number (for cloud reduction)
    LeastTargetPointNumberForCloudReduction = 0,
    /// Closest Target Point (for disjoint pieces)
    ClosestTargetPointForDisjointPieces = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFuseTargetclass {
    Points = 0,
    Vertices = 1,
    Primitives = 2,
    Detail = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFusePositionsnapmethod {
    AverageValue = 0,
    LeastPointNumber = 1,
    GreatestPointNumber = 2,
    MaximumValue = 3,
    MinimumValue = 4,
    Mode = 5,
    Median = 6,
    Sum = 7,
    SumOfSquares = 8,
    RootMeanSquare = 9,
    WeightedAverage = 10,
    WeightedSum = 11,
    MinimumWeight = 12,
    MaximumWeight = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFuseMatchtype {
    EqualAttributeValues = 0,
    UnequalAttributeValues = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFuseGridtype {
    GridSpacing = 0,
    GridLines = 1,
    PowerOf2GridLines = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFuseGridround {
    Nearest = 0,
    Down = 1,
    Up = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFuseAttribsnapmethod {
    Maximum = 0,
    Minimum = 1,
    Average = 2,
    Mode = 3,
    Median = 4,
    Sum = 5,
    SumOfSquares = 6,
    RootMeanSquare = 7,
    FirstMatch = 8,
    LastMatch = 9,
    Concatenate = 10,
    WeightedAverage = 11,
    WeightedSum = 12,
    MinimumWeight = 13,
    MaximumWeight = 14,
    ConcatenateInWeightOrder = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopFuseGrouppropagation {
    LeastPointNumber = 0,
    GreatestPointNumber = 1,
    Union = 2,
    Intersect = 3,
    MostCommon = 4,
}

#[derive(Debug, Clone)]
pub struct SopFuse {
    pub base: crate::core::types::NodeBase,
}

impl SopFuse {
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

    /// Connects to input 0: "Query Geometry"
    pub fn set_input_query_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Query Geometry" and specifies the output index of the target node.
    pub fn set_input_query_geometry_from(
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
    pub fn with_tol3d(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tol3d".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tol3d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tol3d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchtol(mut self, val: f32) -> Self {
        self.base.params.insert(
            "matchtol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_matchtol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "matchtol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gridtol(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gridtol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gridtol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gridtol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gridspacing(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gridspacing".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gridspacing_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gridspacing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gridlines(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gridlines".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gridlines_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gridlines".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gridoffset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gridoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gridoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gridoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_gridpow2(mut self, val: [i32; 3]) -> Self {
        self.base.params.insert(
            "gridpow2".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_gridpow2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gridpow2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_snaptype(mut self, val: SopFuseSnaptype) -> Self {
        self.base.params.insert(
            "snaptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_snaptype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "snaptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_algorithm(mut self, val: SopFuseAlgorithm) -> Self {
        self.base.params.insert(
            "algorithm".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_algorithm_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "algorithm".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetclass(mut self, val: SopFuseTargetclass) -> Self {
        self.base.params.insert(
            "targetclass".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_targetclass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetclass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_positionsnapmethod(mut self, val: SopFusePositionsnapmethod) -> Self {
        self.base.params.insert(
            "positionsnapmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_positionsnapmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "positionsnapmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchtype(mut self, val: SopFuseMatchtype) -> Self {
        self.base.params.insert(
            "matchtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_matchtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "matchtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gridtype(mut self, val: SopFuseGridtype) -> Self {
        self.base.params.insert(
            "gridtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_gridtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gridtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gridround(mut self, val: SopFuseGridround) -> Self {
        self.base.params.insert(
            "gridround".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_gridround_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gridround".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribsnapmethod_inst(
        mut self,
        index1: usize,
        val: SopFuseAttribsnapmethod,
    ) -> Self {
        self.base.params.insert(
            format!("attribsnapmethod{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_attribsnapmethod_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("attribsnapmethod{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_grouppropagation_inst(
        mut self,
        index1: usize,
        val: SopFuseGrouppropagation,
    ) -> Self {
        self.base.params.insert(
            format!("grouppropagation{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_grouppropagation_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("grouppropagation{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_querygroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "querygroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_querygroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "querygroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_posattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "posattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_posattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "posattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetptattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetptattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetptattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetptattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_positionsnapweightname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "positionsnapweightname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_positionsnapweightname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "positionsnapweightname".to_string(),
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
    pub fn with_matchattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "matchattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matchattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "matchattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_snappedgroupname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "snappedgroupname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_snappedgroupname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "snappedgroupname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_snappedattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "snappedattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_snappedattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "snappedattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointattribnames_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("pointattribnames{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointattribnames_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("pointattribnames{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointattribweightname_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("pointattribweightname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointattribweightname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("pointattribweightname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointgroupnames_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("pointgroupnames{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointgroupnames_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("pointgroupnames{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usetargetgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usetargetgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetargetgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usetargetgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modifyboth(mut self, val: bool) -> Self {
        self.base.params.insert(
            "modifyboth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_modifyboth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "modifyboth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetol3d(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usetol3d".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetol3d_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usetol3d".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepositionsnapmethod(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usepositionsnapmethod".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepositionsnapmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usepositionsnapmethod".to_string(),
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
    pub fn with_usematchattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usematchattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usematchattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usematchattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usegridtol(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usegridtol".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegridtol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usegridtol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_consolidatesnappedpoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "consolidatesnappedpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_consolidatesnappedpoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "consolidatesnappedpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepconsolidatedpoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keepconsolidatedpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepconsolidatedpoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keepconsolidatedpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deldegen(mut self, val: bool) -> Self {
        self.base.params.insert(
            "deldegen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deldegen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deldegen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deldegenpoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "deldegenpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deldegenpoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deldegenpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_delunusedpoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "delunusedpoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_delunusedpoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "delunusedpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_recomputenml(mut self, val: bool) -> Self {
        self.base.params.insert(
            "recomputenml".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_recomputenml_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "recomputenml".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createsnappedgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createsnappedgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createsnappedgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createsnappedgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createsnappedattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createsnappedattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createsnappedattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createsnappedattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopFuse {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "fuse"
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
