#[derive(Debug, Clone)]
pub struct SopBakegsplat {
    pub base: crate::core::types::NodeBase,
}

impl SopBakegsplat {
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

    /// Connects to input 0: "Raw GSPlats"
    pub fn set_input_raw_gsplats(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Raw GSPlats" and specifies the output index of the target node.
    pub fn set_input_raw_gsplats_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Toggle parameters ---
    pub fn with_linearize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "linearize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_linearize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "linearize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gsplat(mut self, val: bool) -> Self {
        self.base.params.insert(
            "gsplat".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_gsplat_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gsplat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sphcoeff(mut self, val: bool) -> Self {
        self.base.params.insert(
            "sphcoeff".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sphcoeff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sphcoeff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deleteattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "deleteattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deleteattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deleteattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_noshadowcast(mut self, val: bool) -> Self {
        self.base.params.insert(
            "noshadowcast".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_noshadowcast_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "noshadowcast".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBakegsplat {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "bakegsplat"
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
pub struct SopBakeode {
    pub base: crate::core::types::NodeBase,
}

impl SopBakeode {
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
    pub fn with_keepgeo(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keepgeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepgeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keepgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBakeode {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "bakeode"
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
pub struct SopBakevex {
    pub base: crate::core::types::NodeBase,
}

impl SopBakevex {
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

    /// Connects to input 0: "Geometry Whose Shader to Bake"
    pub fn set_input_geometry_whose_shader_to_bake(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry Whose Shader to Bake" and specifies the output index of the target node.
    pub fn set_input_geometry_whose_shader_to_bake_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
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
    pub fn with_cf(mut self, val: &str) -> Self {
        self.base.params.insert(
            "Cf".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cf_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Cf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_of(mut self, val: &str) -> Self {
        self.base.params.insert(
            "Of".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_of_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Of".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_af(mut self, val: &str) -> Self {
        self.base.params.insert(
            "Af".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_af_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "Af".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: &str) -> Self {
        self.base.params.insert(
            "P".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "P".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_doshade(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doshade".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doshade_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doshade".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodisp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dodisp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodisp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dodisp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBakevex {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "bakevex"
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
pub struct SopBakevolume {
    pub base: crate::core::types::NodeBase,
}

impl SopBakevolume {
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

    /// Connects to input 0: "Volume to Bake"
    pub fn set_input_volume_to_bake(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Volume to Bake" and specifies the output index of the target node.
    pub fn set_input_volume_to_bake_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Points to Sample"
    pub fn set_input_points_to_sample(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Points to Sample" and specifies the output index of the target node.
    pub fn set_input_points_to_sample_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_radiance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "radiance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radiance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "radiance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_absorption(mut self, val: f32) -> Self {
        self.base.params.insert(
            "absorption".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_absorption_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "absorption".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scattering(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scattering".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scattering_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scattering".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emission(mut self, val: f32) -> Self {
        self.base.params.insert(
            "emission".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_emission_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "emission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iso(mut self, val: f32) -> Self {
        self.base.params.insert(
            "iso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_iso_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stepsizescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stepsizescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stepsizescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stepsizescale".to_string(),
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
    pub fn with_multiscatterboost(mut self, val: f32) -> Self {
        self.base.params.insert(
            "multiscatterboost".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_multiscatterboost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "multiscatterboost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_k(mut self, val: f32) -> Self {
        self.base
            .params
            .insert("k".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_k_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "k".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opacitycutoff(mut self, val: f32) -> Self {
        self.base.params.insert(
            "opacitycutoff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_opacitycutoff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "opacitycutoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_light(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "light".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_light_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "light".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lightcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_absorbcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "absorbcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_absorbcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "absorbcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scattercolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "scattercolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scattercolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scattercolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_emitcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "emitcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_emitcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "emitcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_numrays(mut self, val: i32) -> Self {
        self.base.params.insert(
            "numrays".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numrays_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "numrays".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gatherrays(mut self, val: i32) -> Self {
        self.base.params.insert(
            "gatherrays".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_gatherrays_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gatherrays".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_raydepth(mut self, val: i32) -> Self {
        self.base.params.insert(
            "raydepth".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_raydepth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "raydepth".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_computestepsize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "computestepsize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computestepsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "computestepsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dogather(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dogather".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dogather_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dogather".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_multi(mut self, val: bool) -> Self {
        self.base.params.insert(
            "multi".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_multi_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "multi".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_multionly(mut self, val: bool) -> Self {
        self.base.params.insert(
            "multionly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_multionly_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "multionly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_chromaticscatter(mut self, val: bool) -> Self {
        self.base.params.insert(
            "chromaticscatter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_chromaticscatter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "chromaticscatter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optimize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "optimize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_optimize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "optimize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBakevolume {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "bakevolume"
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
pub enum SopBallisticpathLaunchmethod {
    Free = 0,
    Targeted = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBallisticpathTargetingmethod {
    Life = 0,
    Height = 1,
    HeightPlane = 2,
    InitialAngle = 3,
    TargetAngle = 4,
    MinimalEnergy = 5,
}

#[derive(Debug, Clone)]
pub struct SopBallisticpath {
    pub base: crate::core::types::NodeBase,
}

impl SopBallisticpath {
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

    /// Connects to input 0: "Points with Velocity Attribute"
    pub fn set_input_points_with_velocity_attribute(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Points with Velocity Attribute" and specifies the output index of the target node.
    pub fn set_input_points_with_velocity_attribute_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_lifefree(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lifefree".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lifefree_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lifefree".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lifetarget(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lifetarget".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lifetarget_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lifetarget".to_string(),
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
    pub fn with_angleinitial(mut self, val: f32) -> Self {
        self.base.params.insert(
            "angleinitial".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angleinitial_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "angleinitial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angletarget(mut self, val: f32) -> Self {
        self.base.params.insert(
            "angletarget".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angletarget_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "angletarget".to_string(),
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
    pub fn with_clipheight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "clipheight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clipheight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "clipheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_vscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_heightplanepos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "heightplanepos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_heightplanepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "heightplanepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_heightplanenormal(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "heightplanenormal".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_heightplanenormal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "heightplanenormal".to_string(),
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

    // --- Int parameters ---
    pub fn with_fps(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("fps".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_fps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fps".to_string(),
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
    pub fn with_bouncenum(mut self, val: i32) -> Self {
        self.base.params.insert(
            "bouncenum".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bouncenum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bouncenum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_launchmethod(mut self, val: SopBallisticpathLaunchmethod) -> Self {
        self.base.params.insert(
            "launchmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_launchmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "launchmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetingmethod(mut self, val: SopBallisticpathTargetingmethod) -> Self {
        self.base.params.insert(
            "targetingmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_targetingmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetingmethod".to_string(),
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
    pub fn with_targetposattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetposattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetposattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetposattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisiongeopath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "collisiongeopath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collisiongeopath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collisiongeopath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copyattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "copyattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_copyattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "copyattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathnumattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pathnumattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathnumattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathnumattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathpointnumattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pathpointnumattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathpointnumattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathpointnumattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathpointidxattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pathpointidxattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathpointidxattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathpointidxattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_launchspeedattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "launchspeedattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_launchspeedattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "launchspeedattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_endtimeattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "endtimeattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_endtimeattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "endtimeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relposattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "relposattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_relposattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "relposattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_clip(mut self, val: bool) -> Self {
        self.base.params.insert(
            "clip".to_string(),
            crate::core::types::ParamValue::Toggle(val),
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
    pub fn with_docollision(mut self, val: bool) -> Self {
        self.base.params.insert(
            "docollision".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docollision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "docollision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doattribcopy(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doattribcopy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doattribcopy_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doattribcopy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doattribpromote(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doattribpromote".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doattribpromote_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doattribpromote".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createvattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createvattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createvattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createvattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createpathnumattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createpathnumattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createpathnumattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createpathnumattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createpathpointsnumattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createpathpointsnumattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createpathpointsnumattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createpathpointsnumattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createpathpointidxattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createpathpointidxattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createpathpointidxattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createpathpointidxattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createlaunchspeed(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createlaunchspeed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createlaunchspeed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createlaunchspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createendtime(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createendtime".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createendtime_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createendtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createrelpos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createrelpos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createrelpos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createrelpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBallisticpath {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "ballisticpath"
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
pub enum SopBasisUparmtype {
    Unchanged = 0,
    Uniform = 1,
    ChordLength = 2,
    Centripetal = 3,
    /// Manual: Single
    ManualSingle = 4,
    /// Manual: Propagated
    ManualPropagated = 5,
    Knotslide = 6,
    ApproximateArcLength = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBasisVparmtype {
    Unchanged = 0,
    Uniform = 1,
    ChordLength = 2,
    Centripetal = 3,
    /// Manual: Single
    ManualSingle = 4,
    /// Manual: Propagated
    ManualPropagated = 5,
    Knotslide = 6,
    ApproximateArcLength = 7,
}

#[derive(Debug, Clone)]
pub struct SopBasis {
    pub base: crate::core::types::NodeBase,
}

impl SopBasis {
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

    /// Connects to input 0: "Spline Primitives"
    pub fn set_input_spline_primitives(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Spline Primitives" and specifies the output index of the target node.
    pub fn set_input_spline_primitives_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_uread(mut self) -> Self {
        self.base
            .params
            .insert("uread".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_vread(mut self) -> Self {
        self.base
            .params
            .insert("vread".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_ubias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "ubias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ubias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ubias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uorigin(mut self, val: f32) -> Self {
        self.base.params.insert(
            "uorigin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uorigin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uorigin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ulength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "ulength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ulength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ulength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "uscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vbias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vbias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vorigin(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vorigin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vorigin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vorigin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vlength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vlength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_urange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "urange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_urange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "urange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "vrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_vrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vrange".to_string(),
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
    pub fn with_uparmtype(mut self, val: SopBasisUparmtype) -> Self {
        self.base.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vparmtype(mut self, val: SopBasisVparmtype) -> Self {
        self.base.params.insert(
            "vparmtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vparmtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vparmtype".to_string(),
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
    pub fn with_uknots(mut self, val: &str) -> Self {
        self.base.params.insert(
            "uknots".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uknots_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uknots".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vknots(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vknots".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vknots_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vknots".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_ubasis(mut self, val: bool) -> Self {
        self.base.params.insert(
            "ubasis".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ubasis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ubasis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uconcat(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uconcat".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uconcat_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uconcat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_udoorigin(mut self, val: bool) -> Self {
        self.base.params.insert(
            "udoorigin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_udoorigin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "udoorigin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_udolength(mut self, val: bool) -> Self {
        self.base.params.insert(
            "udolength".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_udolength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "udolength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_udoscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "udoscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_udoscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "udoscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uraise(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uraise".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uraise_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uraise".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vbasis(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vbasis".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vbasis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vbasis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vconcat(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vconcat".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vconcat_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vconcat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vdoorigin(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vdoorigin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vdoorigin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vdoorigin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vdolength(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vdolength".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vdolength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vdolength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vdoscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vdoscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vdoscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vdoscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vraise(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vraise".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vraise_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vraise".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBasis {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "basis"
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
pub enum SopBendGrouptype {
    GuessFromGroup = 0,
    Vertices = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBendMaskmode {
    MaskOff = 0,
    MaskOn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBendBendmode {
    Angle = 0,
    Direction = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBendBendscalemodeAngle {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBendBendscalemodeDir {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBendTwistscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBendLengthscalescalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBendTaperaxes {
    X = 0,
    Y = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBendTapermode {
    Linear = 0,
    Smooth = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBendTaperscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBendSquishscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBendUpvectorcontrol {
    XAxis = 0,
    YAxis = 1,
    ZAxis = 2,
    Custom = 3,
}

#[derive(Debug, Clone)]
pub struct SopBend {
    pub base: crate::core::types::NodeBase,
}

impl SopBend {
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

    /// Connects to input 0: "Geometry to Bend"
    pub fn set_input_geometry_to_bend(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Bend" and specifies the output index of the target node.
    pub fn set_input_geometry_to_bend_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Rest Geometry"
    pub fn set_input_rest_geometry(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Rest Geometry" and specifies the output index of the target node.
    pub fn set_input_rest_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_setcaptureregion(mut self) -> Self {
        self.base.params.insert(
            "setcaptureregion".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_bend(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_twist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "twist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_twist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "twist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lengthscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lengthscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lengthscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengthscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_taper(mut self, val: f32) -> Self {
        self.base.params.insert(
            "taper".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_taper_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "taper".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_squish(mut self, val: f32) -> Self {
        self.base.params.insert(
            "squish".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_squish_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "squish".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_squishpivot(mut self, val: f32) -> Self {
        self.base.params.insert(
            "squishpivot".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_squishpivot_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "squishpivot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "upangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_upangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "upangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_derivative_stepsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "derivative_stepsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_derivative_stepsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "derivative_stepsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_goaldir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "goaldir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goaldir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "goaldir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "up".to_string(),
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

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopBendGrouptype) -> Self {
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
    pub fn with_maskmode(mut self, val: SopBendMaskmode) -> Self {
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
    pub fn with_bendmode(mut self, val: SopBendBendmode) -> Self {
        self.base.params.insert(
            "bendmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bendmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bendmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendscalemode_angle(mut self, val: SopBendBendscalemodeAngle) -> Self {
        self.base.params.insert(
            "bendscalemode_angle".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bendscalemode_angle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bendscalemode_angle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendscalemode_dir(mut self, val: SopBendBendscalemodeDir) -> Self {
        self.base.params.insert(
            "bendscalemode_dir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bendscalemode_dir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bendscalemode_dir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_twistscalemode(mut self, val: SopBendTwistscalemode) -> Self {
        self.base.params.insert(
            "twistscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_twistscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "twistscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lengthscalescalemode(mut self, val: SopBendLengthscalescalemode) -> Self {
        self.base.params.insert(
            "lengthscalescalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lengthscalescalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengthscalescalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_taperaxes(mut self, val: SopBendTaperaxes) -> Self {
        self.base.params.insert(
            "taperaxes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_taperaxes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "taperaxes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tapermode(mut self, val: SopBendTapermode) -> Self {
        self.base.params.insert(
            "tapermode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tapermode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tapermode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_taperscalemode(mut self, val: SopBendTaperscalemode) -> Self {
        self.base.params.insert(
            "taperscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_taperscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "taperscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_squishscalemode(mut self, val: SopBendSquishscalemode) -> Self {
        self.base.params.insert(
            "squishscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_squishscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "squishscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upvectorcontrol(mut self, val: SopBendUpvectorcontrol) -> Self {
        self.base.params.insert(
            "upvectorcontrol".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_upvectorcontrol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "upvectorcontrol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_taperprofile(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "taperprofile".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_taperprofile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "taperprofile".to_string(),
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
    pub fn with_bend_attrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bend_attrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bend_attrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bend_attrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_twist_attrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "twist_attrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_twist_attrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "twist_attrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lengthscale_attrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lengthscale_attrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lengthscale_attrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengthscale_attrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_taper_attrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "taper_attrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_taper_attrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "taper_attrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_squish_attrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "squish_attrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_squish_attrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "squish_attrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outattrib".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_dodeform(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dodeform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodeform_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dodeform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limit_deformation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "limit_deformation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limit_deformation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limit_deformation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_symmetricdeformation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "symmetricdeformation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_symmetricdeformation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "symmetricdeformation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablebend(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablebend".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablebend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablebend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletwist(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletwist".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletwist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletwist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_continuoustwist(mut self, val: bool) -> Self {
        self.base.params.insert(
            "continuoustwist".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_continuoustwist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "continuoustwist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablelengthscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablelengthscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablelengthscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablelengthscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preservevolume(mut self, val: bool) -> Self {
        self.base.params.insert(
            "preservevolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preservevolume_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "preservevolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletaper(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletaper".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletaper_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletaper".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableramp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableramp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableoutattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableoutattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableoutattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_userest(mut self, val: bool) -> Self {
        self.base.params.insert(
            "userest".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userest_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "userest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updateaffectednmls(mut self, val: bool) -> Self {
        self.base.params.insert(
            "updateaffectednmls".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_updateaffectednmls_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "updateaffectednmls".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vlength(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vlength".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vlength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBend {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "bend"
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
pub enum SopBetaTissuesolverSolvertype {
    Fem = 0,
    Vellum = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBetaTissuesolverFemintegratortype {
    FirstOrder = 0,
    SecondOrder = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBetaTissuesolverVellumintegratortype {
    FirstOrder = 0,
    SecondOrder = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBetaTissuesolverTargetscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBetaTissuesolverTargetdampingscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBetaTissuesolverVelblendscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBetaTissuesolverCollisiondetection {
    UseSolverDefault = 0,
    UseVolumeCollisions = 1,
    UseSurfaceCollisions = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBetaTissuesolverPreviewdisplay {
    Off = 0,
    Animated = 1,
    AnimatedWithShrinkage = 2,
}

#[derive(Debug, Clone)]
pub struct SopBetaTissuesolver {
    pub base: crate::core::types::NodeBase,
}

impl SopBetaTissuesolver {
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

    /// Connects to input 1: "Fascia"
    pub fn set_input_fascia(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Fascia" and specifies the output index of the target node.
    pub fn set_input_fascia_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Target Constraint Geometry"
    pub fn set_input_target_constraint_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Target Constraint Geometry" and specifies the output index of the target node.
    pub fn set_input_target_constraint_geometry_from(
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
    pub fn with_targetdampingratio(mut self, val: f32) -> Self {
        self.base.params.insert(
            "targetdampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetdampingratio_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetdampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchvelblend(mut self, val: f32) -> Self {
        self.base.params.insert(
            "stretchvelblend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchvelblend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stretchvelblend".to_string(),
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

    // --- Float3 parameters ---
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

    // --- Int parameters ---
    pub fn with_solvertype(mut self, val: SopBetaTissuesolverSolvertype) -> Self {
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

    // --- Menu parameters ---
    pub fn with_femintegratortype(mut self, val: SopBetaTissuesolverFemintegratortype) -> Self {
        self.base.params.insert(
            "femintegratortype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_femintegratortype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "femintegratortype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumintegratortype(
        mut self,
        val: SopBetaTissuesolverVellumintegratortype,
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
    pub fn with_targetscalemode(mut self, val: SopBetaTissuesolverTargetscalemode) -> Self {
        self.base.params.insert(
            "targetscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_targetscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetdampingscalemode(
        mut self,
        val: SopBetaTissuesolverTargetdampingscalemode,
    ) -> Self {
        self.base.params.insert(
            "targetdampingscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_targetdampingscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetdampingscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velblendscalemode(mut self, val: SopBetaTissuesolverVelblendscalemode) -> Self {
        self.base.params.insert(
            "velblendscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_velblendscalemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "velblendscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisiondetection(mut self, val: SopBetaTissuesolverCollisiondetection) -> Self {
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
    pub fn with_previewdisplay(mut self, val: SopBetaTissuesolverPreviewdisplay) -> Self {
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

    // --- String parameters ---
    pub fn with_tissueboundary(mut self, val: &str) -> Self {
        self.base.params.insert(
            "tissueboundary".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tissueboundary_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tissueboundary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fasciaboundary(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fasciaboundary".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fasciaboundary_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fasciaboundary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetdampingattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetdampingattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetdampingattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetdampingattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velblendattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "velblendattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_velblendattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "velblendattrib".to_string(),
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

    // --- Toggle parameters ---
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
    pub fn with_enabletarget(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletarget".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletarget_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletarget".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablevelblend(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablevelblend".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablevelblend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablevelblend".to_string(),
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
    pub fn with_enablesubdermalcollision(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablesubdermalcollision".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesubdermalcollision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablesubdermalcollision".to_string(),
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
}

impl crate::core::types::HoudiniNode for SopBetaTissuesolver {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "beta::tissuesolver"
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
pub enum SopBlastGrouptype {
    GuessFromGroup = 0,
    Breakpoints = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone)]
pub struct SopBlast {
    pub base: crate::core::types::NodeBase,
}

impl SopBlast {
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

    /// Connects to input 0: "Input geometry"
    pub fn set_input_input_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input geometry" and specifies the output index of the target node.
    pub fn set_input_input_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopBlastGrouptype) -> Self {
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

    // --- Toggle parameters ---
    pub fn with_computenorms(mut self, val: bool) -> Self {
        self.base.params.insert(
            "computenorms".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computenorms_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "computenorms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_negate(mut self, val: bool) -> Self {
        self.base.params.insert(
            "negate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_negate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "negate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fillhole(mut self, val: bool) -> Self {
        self.base.params.insert(
            "fillhole".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fillhole_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fillhole".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removegrp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "removegrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removegrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "removegrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBlast {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "blast"
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
pub enum SopBlendshapesGrouptype {
    GuessFromGroup = 0,
    Vertices = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBlendshapesVoxelblend {
    None = 0,
    ByGridIndex = 1,
    ByVoxelPosition = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBlendshapesMaskmode {
    None = 0,
    SetFromAttribute = 1,
    ScaleFromAttribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBlendshapesMaskattribmode {
    FromEachInput = 0,
    FromFirstInput = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBlendshapesBlendmask {
    /// ![BUTTONS_mask_off]No Mask
    NoMask = 0,
    /// ![BUTTONS_mask_default]Default Mask
    DefaultMask = 1,
    /// ![BUTTONS_mask_custom]Custom Mask From This Input
    CustomMaskFromThisInput = 2,
    /// ![BUTTONS_mask_from_input]Custom Mask From First Input
    CustomMaskFromFirstInput = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBlendshapesBlendmaskmode {
    SetFromAttribute = 0,
    ScaleFromAttribute = 1,
}

#[derive(Debug, Clone)]
pub struct SopBlendshapes {
    pub base: crate::core::types::NodeBase,
}

impl SopBlendshapes {
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

    // --- Button parameters ---
    pub fn trigger_updatechannels(mut self) -> Self {
        self.base.params.insert(
            "updatechannels".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_blend_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("blend{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("blend{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_selectionidx(mut self, val: i32) -> Self {
        self.base.params.insert(
            "selectionidx".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_selectionidx_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "selectionidx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopBlendshapesGrouptype) -> Self {
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
    pub fn with_voxelblend(mut self, val: SopBlendshapesVoxelblend) -> Self {
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
    pub fn with_maskmode(mut self, val: SopBlendshapesMaskmode) -> Self {
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
    pub fn with_maskattribmode(mut self, val: SopBlendshapesMaskattribmode) -> Self {
        self.base.params.insert(
            "maskattribmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskattribmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maskattribmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendmask_inst(mut self, index1: usize, val: SopBlendshapesBlendmask) -> Self {
        self.base.params.insert(
            format!("blendmask{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendmask_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("blendmask{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendmaskmode_inst(
        mut self,
        index1: usize,
        val: SopBlendshapesBlendmaskmode,
    ) -> Self {
        self.base.params.insert(
            format!("blendmaskmode{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendmaskmode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("blendmaskmode{}", index1),
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
    pub fn with_blendmaskattrib_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("blendmaskattrib{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blendmaskattrib_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("blendmaskattrib{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_diff(mut self, val: bool) -> Self {
        self.base.params.insert(
            "diff".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_diff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "diff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachedeltas(mut self, val: bool) -> Self {
        self.base.params.insert(
            "cachedeltas".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cachedeltas_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cachedeltas".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pack(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pack".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pack_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pack".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packfirst(mut self, val: bool) -> Self {
        self.base.params.insert(
            "packfirst".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_packfirst_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "packfirst".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weightperpack(mut self, val: bool) -> Self {
        self.base.params.insert(
            "weightperpack".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_weightperpack_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "weightperpack".to_string(),
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
}

impl crate::core::types::HoudiniNode for SopBlendshapes {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "blendshapes"
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
pub enum SopBlockBeginMethod {
    FetchFeedback = 0,
    ExtractPieceOrPoint = 1,
    FetchMetadata = 2,
    FetchInput = 3,
}

#[derive(Debug, Clone)]
pub struct SopBlockBegin {
    pub base: crate::core::types::NodeBase,
}

impl SopBlockBegin {
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

    /// Connects to input 0: "Initial Geometry"
    pub fn set_input_initial_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Initial Geometry" and specifies the output index of the target node.
    pub fn set_input_initial_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_resetcookpass(mut self) -> Self {
        self.base.params.insert(
            "resetcookpass".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_createmetablock(mut self) -> Self {
        self.base.params.insert(
            "createmetablock".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: SopBlockBeginMethod) -> Self {
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
    pub fn with_blockpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "blockpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blockpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blockpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBlockBegin {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "block_begin"
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
pub enum SopBlockEndItermethod {
    AutoDetectFromInputs = 0,
    ByPiecesOrPoints = 1,
    ByCount = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBlockEndMethod {
    FeedbackEachIteration = 0,
    MergeEachIteration = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBlockEndClass {
    Primitives = 0,
    Points = 1,
}

#[derive(Debug, Clone)]
pub struct SopBlockEnd {
    pub base: crate::core::types::NodeBase,
}

impl SopBlockEnd {
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

    /// Connects to input 0: "Nodes to Iterate Over"
    pub fn set_input_nodes_to_iterate_over(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Nodes to Iterate Over" and specifies the output index of the target node.
    pub fn set_input_nodes_to_iterate_over_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Geometry Pieces to Loop Over"
    pub fn set_input_geometry_pieces_to_loop_over(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Geometry Pieces to Loop Over" and specifies the output index of the target node.
    pub fn set_input_geometry_pieces_to_loop_over_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_resetcookpass(mut self) -> Self {
        self.base.params.insert(
            "resetcookpass".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_startvalue(mut self, val: f32) -> Self {
        self.base.params.insert(
            "startvalue".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_startvalue_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "startvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_increment(mut self, val: f32) -> Self {
        self.base.params.insert(
            "increment".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_increment_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "increment".to_string(),
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
    pub fn with_singlepass(mut self, val: i32) -> Self {
        self.base.params.insert(
            "singlepass".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_singlepass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "singlepass".to_string(),
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
    pub fn with_itermethod(mut self, val: SopBlockEndItermethod) -> Self {
        self.base.params.insert(
            "itermethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_itermethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "itermethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_method(mut self, val: SopBlockEndMethod) -> Self {
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
    pub fn with_class(mut self, val: SopBlockEndClass) -> Self {
        self.base.params.insert(
            "class".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_class_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "class".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_blockpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "blockpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blockpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blockpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_templatepath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "templatepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_templatepath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "templatepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stopattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "stopattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stopattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stopattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useattrib".to_string(),
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
    pub fn with_dosinglepass(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dosinglepass".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosinglepass_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dosinglepass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_multithread(mut self, val: bool) -> Self {
        self.base.params.insert(
            "multithread".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_multithread_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "multithread".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBlockEnd {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "block_end"
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
pub enum SopBonecapturebiharmonicColor {
    DefaultSourceColor = 0,
    ColorByCaptureRegion = 1,
}

#[derive(Debug, Clone)]
pub struct SopBonecapturebiharmonic {
    pub base: crate::core::types::NodeBase,
}

impl SopBonecapturebiharmonic {
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

    /// Connects to input 1: "Capture Geometry"
    pub fn set_input_capture_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Capture Geometry" and specifies the output index of the target node.
    pub fn set_input_capture_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_difftol(mut self, val: f32) -> Self {
        self.base.params.insert(
            "difftol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_difftol_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "difftol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendfactor(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blendfactor".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blendfactor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendfactor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_zeroweightcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "zeroweightcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_zeroweightcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "zeroweightcolor".to_string(),
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

    // --- Menu parameters ---
    pub fn with_color(mut self, val: SopBonecapturebiharmonicColor) -> Self {
        self.base.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_destroyweights(mut self, val: bool) -> Self {
        self.base.params.insert(
            "destroyweights".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_destroyweights_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "destroyweights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcapturetets(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputcapturetets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputcapturetets_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputcapturetets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbose(mut self, val: bool) -> Self {
        self.base.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_verbose_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBonecapturebiharmonic {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "bonecapturebiharmonic"
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
pub enum SopBonecapturelinesResample {
    Off = 0,
    ByMaxAxisFraction = 1,
    ByMaxSegmentLength = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBonecapturelinesCookat {
    CaptureFrame = 0,
    AnyFrame = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBonecapturelinesCaptureregionsop {
    DisplaySops = 0,
    RenderSops = 1,
    /// SOPs named "cregion"
    SopsNamedCregion = 2,
}

#[derive(Debug, Clone)]
pub struct SopBonecapturelines {
    pub base: crate::core::types::NodeBase,
}

impl SopBonecapturelines {
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

    /// Connects to input 0: "Extra Regions"
    pub fn set_input_extra_regions(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Extra Regions" and specifies the output index of the target node.
    pub fn set_input_extra_regions_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_forcecook(mut self) -> Self {
        self.base.params.insert(
            "forcecook".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_maxaxisfraction(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxaxisfraction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxaxisfraction_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxaxisfraction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxlength(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxlength_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_excludethreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "excludethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_excludethreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "excludethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fusethreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "fusethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fusethreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fusethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_captframe(mut self, val: f32) -> Self {
        self.base.params.insert(
            "captframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_captframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "captframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_resample(mut self, val: SopBonecapturelinesResample) -> Self {
        self.base.params.insert(
            "resample".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_cookat(mut self, val: SopBonecapturelinesCookat) -> Self {
        self.base.params.insert(
            "cookat".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cookat_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cookat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_captureregionsop(mut self, val: SopBonecapturelinesCaptureregionsop) -> Self {
        self.base.params.insert(
            "captureregionsop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_captureregionsop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "captureregionsop".to_string(),
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
    pub fn with_rootpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rootpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rootpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rootpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extraregions(mut self, val: &str) -> Self {
        self.base.params.insert(
            "extraregions".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_extraregions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extraregions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_excludeshortbones(mut self, val: bool) -> Self {
        self.base.params.insert(
            "excludeshortbones".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_excludeshortbones_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "excludeshortbones".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usebonelink(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usebonelink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebonelink_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usebonelink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecaptpose(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usecaptpose".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecaptpose_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usecaptpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosubnets(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dosubnets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosubnets_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dosubnets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relativeskel(mut self, val: bool) -> Self {
        self.base.params.insert(
            "relativeskel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_relativeskel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "relativeskel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBonecapturelines {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "bonecapturelines"
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
pub enum SopBonedeformMethod {
    Linear = 0,
    DualQuaternion = 1,
    BlendDualQuaternionAndLinear = 2,
    FromInputGeometry = 3,
}

#[derive(Debug, Clone)]
pub struct SopBonedeform {
    pub base: crate::core::types::NodeBase,
}

impl SopBonedeform {
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

    /// Connects to input 1: "Rest Point Transforms"
    pub fn set_input_rest_point_transforms(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Rest Point Transforms" and specifies the output index of the target node.
    pub fn set_input_rest_point_transforms_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Deform Point Transforms"
    pub fn set_input_deform_point_transforms(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Deform Point Transforms" and specifies the output index of the target node.
    pub fn set_input_deform_point_transforms_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: SopBonedeformMethod) -> Self {
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
    pub fn with_dqblendattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dqblendattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dqblendattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dqblendattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonetransformpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bonetransformpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bonetransformpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonetransformpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonetransformtargetpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bonetransformtargetpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bonetransformtargetpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonetransformtargetpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonetransformregionpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bonetransformregionpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bonetransformregionpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonetransformregionpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_otherattribs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "otherattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_otherattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "otherattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_donormal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "donormal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_donormal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "donormal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deletecaptureattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "deletecaptureattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deletecaptureattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deletecaptureattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deletepointtcolors(mut self, val: bool) -> Self {
        self.base.params.insert(
            "deletepointtcolors".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deletepointtcolors_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deletepointtcolors".to_string(),
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

impl crate::core::types::HoudiniNode for SopBonedeform {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "bonedeform"
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
pub struct SopBonelink {
    pub base: crate::core::types::NodeBase,
}

impl SopBonelink {
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

    /// Connects to input 0: "Custom Link Geometry"
    pub fn set_input_custom_link_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Custom Link Geometry" and specifies the output index of the target node.
    pub fn set_input_custom_link_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Proxy Geometry"
    pub fn set_input_proxy_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Proxy Geometry" and specifies the output index of the target node.
    pub fn set_input_proxy_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Capture Regions"
    pub fn set_input_capture_regions(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Capture Regions" and specifies the output index of the target node.
    pub fn set_input_capture_regions_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_linkscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "linkscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_linkscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "linkscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_linkfinsize(mut self, val: f32) -> Self {
        self.base.params.insert(
            "linkfinsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_linkfinsize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "linkfinsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_linkcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "linkcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_linkcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "linkcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_proxyscale(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "proxyscale".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_proxyscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "proxyscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_snappos_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.base.params.insert(
            format!("snappos{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_snappos_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("snappos{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_showlink(mut self, val: i32) -> Self {
        self.base.params.insert(
            "showlink".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_showlink_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_linktype(mut self, val: i32) -> Self {
        self.base.params.insert(
            "linktype".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_linktype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "linktype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uselinkcolor(mut self, val: i32) -> Self {
        self.base.params.insert(
            "uselinkcolor".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uselinkcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uselinkcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showlinkfin(mut self, val: i32) -> Self {
        self.base.params.insert(
            "showlinkfin".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_showlinkfin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showlinkfin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showproxy(mut self, val: i32) -> Self {
        self.base.params.insert(
            "showproxy".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_showproxy_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showproxy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showcapture(mut self, val: i32) -> Self {
        self.base.params.insert(
            "showcapture".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_showcapture_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showcapture".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_packname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "packname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_packname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "packname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_packbone(mut self, val: bool) -> Self {
        self.base.params.insert(
            "packbone".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_packbone_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "packbone".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBonelink {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "bonelink"
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
pub enum SopBonesolidifyTposeswitch {
    UseInitializationFrame = 0,
    FromAttibute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBonesolidifySizing {
    Uniform = 0,
    Adaptive = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBonesolidifyBoneanimationtype {
    PointDeform = 0,
    UseExistingBonecaptureWeights = 1,
    TransformRigidPieces = 2,
}

#[derive(Debug, Clone)]
pub struct SopBonesolidify {
    pub base: crate::core::types::NodeBase,
}

impl SopBonesolidify {
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

    /// Connects to input 0: "Bone Surface Geometry"
    pub fn set_input_bone_surface_geometry(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Bone Surface Geometry" and specifies the output index of the target node.
    pub fn set_input_bone_surface_geometry_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Optional Bone Surface Triangles"
    pub fn set_input_optional_bone_surface_triangles(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Optional Bone Surface Triangles" and specifies the output index of the target node.
    pub fn set_input_optional_bone_surface_triangles_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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

    // --- Int parameters ---
    pub fn with_tposeswitch(mut self, val: SopBonesolidifyTposeswitch) -> Self {
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
    pub fn with_initframe(mut self, val: i32) -> Self {
        self.base.params.insert(
            "initframe".to_string(),
            crate::core::types::ParamValue::Int(val),
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
    pub fn with_bonepiecemin(mut self, val: i32) -> Self {
        self.base.params.insert(
            "bonepiecemin".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_bonepiecemin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonepiecemin".to_string(),
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

    // --- Menu parameters ---
    pub fn with_sizing(mut self, val: SopBonesolidifySizing) -> Self {
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
    pub fn with_boneanimationtype(mut self, val: SopBonesolidifyBoneanimationtype) -> Self {
        self.base.params.insert(
            "boneanimationtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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

    // --- String parameters ---
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
    pub fn with_rigpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rigpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rigpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rigpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useinputmesh(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useinputmesh".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useinputmesh_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useinputmesh".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remeshsurfaces(mut self, val: bool) -> Self {
        self.base.params.insert(
            "remeshsurfaces".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remeshsurfaces_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remeshsurfaces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablepieceremoval(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablepieceremoval".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablepieceremoval_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablepieceremoval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBonesolidify {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "bonesolidify"
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
pub enum SopBooleanAsurface {
    Solid = 0,
    Surface = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBooleanBsurface {
    Solid = 0,
    Surface = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBooleanBooleanop {
    Union = 0,
    Intersect = 1,
    Subtract = 2,
    Shatter = 3,
    Custom = 4,
    Separator = 5,
    Seam = 6,
    /// _separator_
    Separator1 = 7,
    Detect = 8,
    Resolve = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBooleanSubtractchoices {
    /// A - B
    AMinusB = 0,
    /// B - A
    BMinusA = 1,
    Both = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBooleanShatterchoices {
    PiecesOfA = 0,
    PiecesOfB = 1,
    Both = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBooleanWindingop {
    ARange = 0,
    BRange = 1,
    Both = 2,
    AtLeastOne = 3,
    ExactlyOne = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBooleanDetriangulate {
    NoPolygons = 0,
    OnlyUnchangedPolygons = 1,
    AllPolygons = 2,
}

#[derive(Debug, Clone)]
pub struct SopBoolean {
    pub base: crate::core::types::NodeBase,
}

impl SopBoolean {
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

    /// Connects to input 0: "Geometry A"
    pub fn set_input_geometry_a(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry A" and specifies the output index of the target node.
    pub fn set_input_geometry_a_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Geometry B"
    pub fn set_input_geometry_b(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Geometry B" and specifies the output index of the target node.
    pub fn set_input_geometry_b_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_lengththreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lengththreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lengththreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengththreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_adepth(mut self, val: [i32; 2]) -> Self {
        self.base.params.insert(
            "adepth".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_adepth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bdepth(mut self, val: [i32; 2]) -> Self {
        self.base.params.insert(
            "bdepth".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_bdepth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bdepth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_asurface(mut self, val: SopBooleanAsurface) -> Self {
        self.base.params.insert(
            "asurface".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_asurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "asurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bsurface(mut self, val: SopBooleanBsurface) -> Self {
        self.base.params.insert(
            "bsurface".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bsurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_booleanop(mut self, val: SopBooleanBooleanop) -> Self {
        self.base.params.insert(
            "booleanop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_booleanop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "booleanop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subtractchoices(mut self, val: SopBooleanSubtractchoices) -> Self {
        self.base.params.insert(
            "subtractchoices".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_subtractchoices_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "subtractchoices".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shatterchoices(mut self, val: SopBooleanShatterchoices) -> Self {
        self.base.params.insert(
            "shatterchoices".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shatterchoices_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shatterchoices".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windingop(mut self, val: SopBooleanWindingop) -> Self {
        self.base.params.insert(
            "windingop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_windingop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "windingop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_detriangulate(mut self, val: SopBooleanDetriangulate) -> Self {
        self.base.params.insert(
            "detriangulate".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_detriangulate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "detriangulate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_agroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "agroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_agroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "agroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axapolys(mut self, val: &str) -> Self {
        self.base.params.insert(
            "axapolys".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_axapolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "axapolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axbpolys(mut self, val: &str) -> Self {
        self.base.params.insert(
            "axbpolys".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_axbpolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "axbpolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axalist(mut self, val: &str) -> Self {
        self.base.params.insert(
            "axalist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_axalist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "axalist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_axblist(mut self, val: &str) -> Self {
        self.base.params.insert(
            "axblist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_axblist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "axblist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_apolys(mut self, val: &str) -> Self {
        self.base.params.insert(
            "apolys".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_apolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "apolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ainsideb(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ainsideb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ainsideb_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ainsideb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aoutsideb(mut self, val: &str) -> Self {
        self.base.params.insert(
            "aoutsideb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aoutsideb_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aoutsideb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bpolys(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bpolys".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bpolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bpolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_binsidea(mut self, val: &str) -> Self {
        self.base.params.insert(
            "binsidea".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_binsidea_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "binsidea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boutsidea(mut self, val: &str) -> Self {
        self.base.params.insert(
            "boutsidea".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_boutsidea_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "boutsidea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aboverlap(mut self, val: &str) -> Self {
        self.base.params.insert(
            "aboverlap".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aboverlap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aboverlap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aonlypieces(mut self, val: &str) -> Self {
        self.base.params.insert(
            "aonlypieces".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aonlypieces_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aonlypieces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bonlypieces(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bonlypieces".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bonlypieces_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bonlypieces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_abpieces(mut self, val: &str) -> Self {
        self.base.params.insert(
            "abpieces".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_abpieces_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "abpieces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reversedpolys(mut self, val: &str) -> Self {
        self.base.params.insert(
            "reversedpolys".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reversedpolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reversedpolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aaseamedges(mut self, val: &str) -> Self {
        self.base.params.insert(
            "aaseamedges".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_aaseamedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "aaseamedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bbseamedges(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bbseamedges".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bbseamedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bbseamedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_abseamedges(mut self, val: &str) -> Self {
        self.base.params.insert(
            "abseamedges".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_abseamedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "abseamedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_resolvea(mut self, val: bool) -> Self {
        self.base.params.insert(
            "resolvea".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resolvea_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resolvea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolveb(mut self, val: bool) -> Self {
        self.base.params.insert(
            "resolveb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resolveb_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resolveb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opencurvesonly(mut self, val: bool) -> Self {
        self.base.params.insert(
            "opencurvesonly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_opencurvesonly_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "opencurvesonly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_generateaaseams(mut self, val: bool) -> Self {
        self.base.params.insert(
            "generateaaseams".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_generateaaseams_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "generateaaseams".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_generatebbseams(mut self, val: bool) -> Self {
        self.base.params.insert(
            "generatebbseams".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_generatebbseams_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "generatebbseams".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_generateabseams(mut self, val: bool) -> Self {
        self.base.params.insert(
            "generateabseams".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_generateabseams_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "generateabseams".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mergenbrs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "mergenbrs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mergenbrs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mergenbrs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removeinlinepoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "removeinlinepoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removeinlinepoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "removeinlinepoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniqueseams(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniqueseams".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniqueseams_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniqueseams".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_correctnormals(mut self, val: bool) -> Self {
        self.base.params.insert(
            "correctnormals".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_correctnormals_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "correctnormals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useaxapolys(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useaxapolys".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useaxapolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useaxapolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useaxbpolys(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useaxbpolys".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useaxbpolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useaxbpolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useaxalist(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useaxalist".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useaxalist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useaxalist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useaxblist(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useaxblist".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useaxblist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useaxblist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collapsetinyedges(mut self, val: bool) -> Self {
        self.base.params.insert(
            "collapsetinyedges".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collapsetinyedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collapsetinyedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useapolys(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useapolys".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useapolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useapolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useainsideb(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useainsideb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useainsideb_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useainsideb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useaoutsideb(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useaoutsideb".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useaoutsideb_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useaoutsideb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usebpolys(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usebpolys".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebpolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usebpolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usebinsidea(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usebinsidea".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebinsidea_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usebinsidea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useboutsidea(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useboutsidea".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useboutsidea_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useboutsidea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useaboverlap(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useaboverlap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useaboverlap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useaboverlap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useaonlypieces(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useaonlypieces".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useaonlypieces_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useaonlypieces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usebonlypieces(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usebonlypieces".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebonlypieces_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usebonlypieces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useabpieces(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useabpieces".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useabpieces_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useabpieces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usereversedpolys(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usereversedpolys".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usereversedpolys_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usereversedpolys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useaaseamedges(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useaaseamedges".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useaaseamedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useaaseamedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usebbseamedges(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usebbseamedges".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebbseamedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usebbseamedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useabseamedges(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useabseamedges".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useabseamedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useabseamedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBoolean {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "boolean"
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
pub enum SopBooleanfractureNamemethod {
    Overwrite = 0,
    Append = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBooleanfractureComputeexteriornormals {
    PreserveExistingNormals = 0,
    RecomputeNormals = 1,
    DoNotComputeNormals = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBooleanfractureInputsurface {
    Solid = 0,
    Surface = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBooleanfractureCuttingsurface {
    Solid = 0,
    Surface = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBooleanfractureDetriangulate {
    NoPolygons = 0,
    OnlyUnchangedPolygons = 1,
    AllPolygons = 2,
}

#[derive(Debug, Clone)]
pub struct SopBooleanfracture {
    pub base: crate::core::types::NodeBase,
}

impl SopBooleanfracture {
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

    /// Connects to input 0: "Geometry to Fracture"
    pub fn set_input_geometry_to_fracture(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Geometry to Fracture" and specifies the output index of the target node.
    pub fn set_input_geometry_to_fracture_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Cutting Surface"
    pub fn set_input_cutting_surface(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Cutting Surface" and specifies the output index of the target node.
    pub fn set_input_cutting_surface_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_interiorcuspangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "interiorcuspangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_interiorcuspangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "interiorcuspangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exteriorcuspangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "exteriorcuspangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exteriorcuspangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exteriorcuspangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lengththreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lengththreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lengththreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lengththreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_namemethod(mut self, val: SopBooleanfractureNamemethod) -> Self {
        self.base.params.insert(
            "namemethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_namemethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "namemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computeexteriornormals(
        mut self,
        val: SopBooleanfractureComputeexteriornormals,
    ) -> Self {
        self.base.params.insert(
            "computeexteriornormals".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_computeexteriornormals_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "computeexteriornormals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputsurface(mut self, val: SopBooleanfractureInputsurface) -> Self {
        self.base.params.insert(
            "inputsurface".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inputsurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cuttingsurface(mut self, val: SopBooleanfractureCuttingsurface) -> Self {
        self.base.params.insert(
            "cuttingsurface".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cuttingsurface_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cuttingsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_detriangulate(mut self, val: SopBooleanfractureDetriangulate) -> Self {
        self.base.params.insert(
            "detriangulate".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_detriangulate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "detriangulate".to_string(),
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
    pub fn with_fracturenamespace(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fracturenamespace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fracturenamespace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fracturenamespace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nameprefix(mut self, val: &str) -> Self {
        self.base.params.insert(
            "nameprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nameprefix_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "nameprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribnameprefix(mut self, val: &str) -> Self {
        self.base.params.insert(
            "attribnameprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribnameprefix_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attribnameprefix".to_string(),
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
    pub fn with_ptattributes(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ptattributes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ptattributes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ptattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vtxattributes(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vtxattributes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vtxattributes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vtxattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primattributes(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primattributes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primattributes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiorgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "interiorgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_interiorgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "interiorgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exteriorgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "exteriorgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_exteriorgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exteriorgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cutpiecesgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "cutpiecesgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cutpiecesgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cutpiecesgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cutsurfacegroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "cutsurfacegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cutsurfacegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cutsurfacegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiorseamedges(mut self, val: &str) -> Self {
        self.base.params.insert(
            "interiorseamedges".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_interiorseamedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "interiorseamedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exteriorseamedges(mut self, val: &str) -> Self {
        self.base.params.insert(
            "exteriorseamedges".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_exteriorseamedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exteriorseamedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_computeinteriornormals(mut self, val: bool) -> Self {
        self.base.params.insert(
            "computeinteriornormals".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeinteriornormals_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "computeinteriornormals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputpieceattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputpieceattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputpieceattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputpieceattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copycuttingsurfaceattribs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "copycuttingsurfaceattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_copycuttingsurfaceattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "copycuttingsurfaceattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputinteriorgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputinteriorgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputinteriorgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputinteriorgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputexteriorgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputexteriorgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputexteriorgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputexteriorgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mergegroups(mut self, val: bool) -> Self {
        self.base.params.insert(
            "mergegroups".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mergegroups_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mergegroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcutpiecesgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputcutpiecesgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputcutpiecesgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputcutpiecesgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcutsurfacegroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputcutsurfacegroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputcutsurfacegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputcutsurfacegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputinteriorseamedges(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputinteriorseamedges".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputinteriorseamedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputinteriorseamedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputexteriorseamedges(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputexteriorseamedges".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputexteriorseamedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputexteriorseamedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removeinlinepoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "removeinlinepoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removeinlinepoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "removeinlinepoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collapsetinyedges(mut self, val: bool) -> Self {
        self.base.params.insert(
            "collapsetinyedges".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collapsetinyedges_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "collapsetinyedges".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBooleanfracture {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "booleanfracture"
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
pub enum SopBoundGrouptype {
    GuessFromGroup = 0,
    Breakpoints = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBoundBoundtype {
    Box = 0,
    Sphere = 1,
    Rectangle = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBoundOrient {
    XAxis = 0,
    YAxis = 1,
    ZAxis = 2,
}

#[derive(Debug, Clone)]
pub struct SopBound {
    pub base: crate::core::types::NodeBase,
}

impl SopBound {
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
    pub fn with_minradius(mut self, val: f32) -> Self {
        self.base.params.insert(
            "minradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minradius_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minradius".to_string(),
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
    pub fn with_minsize(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "minsize".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_minpad(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "minpad".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_minpad_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "minpad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxpad(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "maxpad".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_maxpad_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxpad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_refinementiterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "refinementiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_refinementiterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "refinementiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_divs(mut self, val: [i32; 3]) -> Self {
        self.base.params.insert(
            "divs".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_divs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "divs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopBoundGrouptype) -> Self {
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
    pub fn with_boundtype(mut self, val: SopBoundBoundtype) -> Self {
        self.base.params.insert(
            "boundtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_boundtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "boundtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orient(mut self, val: SopBoundOrient) -> Self {
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
    pub fn with_boundsgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "boundsgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_boundsgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "boundsgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xformattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "xformattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xformattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xformattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_radiiattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "radiiattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_radiiattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "radiiattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeporiginal(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keepOriginal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeporiginal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keepOriginal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createboundinggeo(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createboundinggeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createboundinggeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createboundinggeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createempty(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createempty".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createempty_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createempty".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientedbbox(mut self, val: bool) -> Self {
        self.base.params.insert(
            "orientedbbox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_orientedbbox_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orientedbbox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodivs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dodivs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodivs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dodivs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rebar(mut self, val: bool) -> Self {
        self.base.params.insert(
            "rebar".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rebar_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rebar".to_string(),
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
    pub fn with_orientedbrect(mut self, val: bool) -> Self {
        self.base.params.insert(
            "orientedbrect".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_orientedbrect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orientedbrect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addboundsgroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addboundsgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addboundsgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addboundsgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addxformattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addxformattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addxformattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addxformattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addradiiattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "addradiiattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addradiiattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addradiiattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBound {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "bound"
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
pub enum SopBoxType {
    Polygon = 0,
    PolygonMesh = 1,
    Mesh = 2,
    Nurbs = 3,
    Bezier = 4,
    Points = 5,
    Primitive = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBoxSurftype {
    Rows = 0,
    Columns = 1,
    RowsAndColumns = 2,
    Triangles = 3,
    Quadrilaterals = 4,
    AlternatingTriangles = 5,
    ReverseTriangles = 6,
}

#[derive(Debug, Clone)]
pub struct SopBox {
    pub base: crate::core::types::NodeBase,
}

impl SopBox {
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

    // --- Int3 parameters ---
    pub fn with_divrate(mut self, val: [i32; 3]) -> Self {
        self.base.params.insert(
            "divrate".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_divrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "divrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orderrate(mut self, val: [i32; 3]) -> Self {
        self.base.params.insert(
            "orderrate".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_orderrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orderrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divs(mut self, val: [i32; 3]) -> Self {
        self.base.params.insert(
            "divs".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_divs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "divs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: SopBoxType) -> Self {
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
    pub fn with_surftype(mut self, val: SopBoxSurftype) -> Self {
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

    // --- Toggle parameters ---
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
    pub fn with_dodivs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dodivs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodivs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dodivs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rebar(mut self, val: bool) -> Self {
        self.base.params.insert(
            "rebar".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rebar_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rebar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientedbbox(mut self, val: bool) -> Self {
        self.base.params.insert(
            "orientedbbox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_orientedbbox_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orientedbbox".to_string(),
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
}

impl crate::core::types::HoudiniNode for SopBox {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "box"
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
pub enum SopBreakShape {
    Grid = 0,
    Box = 1,
    Sphere = 2,
}

#[derive(Debug, Clone)]
pub struct SopBreak {
    pub base: crate::core::types::NodeBase,
}

impl SopBreak {
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

    /// Connects to input 0: "Polygons to Break"
    pub fn set_input_polygons_to_break(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Polygons to Break" and specifies the output index of the target node.
    pub fn set_input_polygons_to_break_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Optional Cutting Object"
    pub fn set_input_optional_cutting_object(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Optional Cutting Object" and specifies the output index of the target node.
    pub fn set_input_optional_cutting_object_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_volume_bias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "volume_bias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volume_bias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "volume_bias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skip_percentage(mut self, val: f32) -> Self {
        self.base.params.insert(
            "skip_percentage".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_skip_percentage_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "skip_percentage".to_string(),
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
    pub fn with_jitteramount(mut self, val: f32) -> Self {
        self.base.params.insert(
            "jitteramount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jitteramount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "jitteramount".to_string(),
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
    pub fn with_sphere_freq(mut self, val: i32) -> Self {
        self.base.params.insert(
            "sphere_freq".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sphere_freq_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sphere_freq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frac_depth(mut self, val: i32) -> Self {
        self.base.params.insert(
            "frac_depth".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_frac_depth_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "frac_depth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: i32) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Int(val),
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

    // --- Int3 parameters ---
    pub fn with_divrate(mut self, val: [i32; 3]) -> Self {
        self.base.params.insert(
            "divrate".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_divrate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "divrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_shape(mut self, val: SopBreakShape) -> Self {
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
    pub fn with_inside_group(mut self, val: &str) -> Self {
        self.base.params.insert(
            "inside_group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inside_group_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inside_group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ntype(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ntype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ntype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ntype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keep_outside(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keep_outside".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keep_outside_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keep_outside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keep_inside(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keep_inside".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keep_inside_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keep_inside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doinside(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doInside".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doinside_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doInside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visualize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "visualize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visualize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "visualize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doskipping(mut self, val: bool) -> Self {
        self.base.params.insert(
            "doSkipping".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doskipping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "doSkipping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_custom(mut self, val: bool) -> Self {
        self.base.params.insert(
            "use_custom".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_custom_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "use_custom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_insidetest(mut self, val: bool) -> Self {
        self.base.params.insert(
            "insidetest".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_insidetest_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "insidetest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closed(mut self, val: bool) -> Self {
        self.base.params.insert(
            "closed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "closed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dojitter(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dojitter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dojitter_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dojitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBreak {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "break"
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
pub enum SopBridgeBridge {
    AllPrimitives = 0,
    GroupsOfNPrimitives = 1,
    SkipEveryNthPrimitive = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SopBridgeFrenet {
    TheFrenetFrameOfTheFace = 0,
    TheNormalOfTheFace = 1,
}

#[derive(Debug, Clone)]
pub struct SopBridge {
    pub base: crate::core::types::NodeBase,
}

impl SopBridge {
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

    /// Connects to input 0: "Profiles and/or 3D Faces"
    pub fn set_input_profiles_and_or_3d_faces(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Profiles and/or 3D Faces" and specifies the output index of the target node.
    pub fn set_input_profiles_and_or_3d_faces_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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
    pub fn with_rotatet(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "rotatet".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_rotatet_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rotatet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalet(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "scalet".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scalet_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scalet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalec(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "scalec".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_scalec_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scalec".to_string(),
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
    pub fn with_isodivs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "isodivs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_isodivs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "isodivs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdivs(mut self, val: i32) -> Self {
        self.base.params.insert(
            "sdivs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sdivs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sdivs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_bridge(mut self, val: SopBridgeBridge) -> Self {
        self.base.params.insert(
            "bridge".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bridge_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bridge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frenet(mut self, val: SopBridgeFrenet) -> Self {
        self.base.params.insert(
            "frenet".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_frenet_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "frenet".to_string(),
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
    pub fn with_circular(mut self, val: bool) -> Self {
        self.base.params.insert(
            "circular".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_circular_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "circular".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_csharp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "csharp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_csharp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "csharp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBridge {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "bridge"
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
pub enum SopBulgeGrouptype {
    GuessFromGroup = 0,
    Breakpoints = 1,
    Edges = 2,
    Points = 3,
    Primitives = 4,
}

#[derive(Debug, Clone)]
pub struct SopBulge {
    pub base: crate::core::types::NodeBase,
}

impl SopBulge {
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

    /// Connects to input 0: "Points to Bulge"
    pub fn set_input_points_to_bulge(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Points to Bulge" and specifies the output index of the target node.
    pub fn set_input_points_to_bulge_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Magnet"
    pub fn set_input_magnet(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Magnet" and specifies the output index of the target node.
    pub fn set_input_magnet_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_mag(mut self, val: f32) -> Self {
        self.base.params.insert(
            "mag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_grouptype(mut self, val: SopBulgeGrouptype) -> Self {
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
    pub fn with_deformgrp(mut self, val: &str) -> Self {
        self.base.params.insert(
            "deformGrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deformgrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "deformGrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_magnetgrp(mut self, val: &str) -> Self {
        self.base.params.insert(
            "magnetGrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_magnetgrp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "magnetGrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_nml(mut self, val: bool) -> Self {
        self.base.params.insert(
            "nml".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_nml_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "nml".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for SopBulge {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "bulge"
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
