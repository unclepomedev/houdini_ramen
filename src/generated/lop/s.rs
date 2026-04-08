#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopScenedoctorValidatekind {
    Message = 0,
    Warning = 1,
    Error = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopScenedoctorKindaction {
    AlertOnly = 0,
    FixAndAlert = 1,
    FixSilently = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopScenedoctorValidategprim {
    Message = 0,
    Warning = 1,
    Error = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopScenedoctorGprimaction {
    AlertOnly = 0,
    FixAndAlert = 1,
    FixSilently = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopScenedoctorValidateprimvars {
    Message = 0,
    Warning = 1,
    Error = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopScenedoctorPrimvarsaction {
    AlertOnly = 0,
    FixAndAlert = 1,
    FixSilently = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopScenedoctorValidatevalueclips {
    Message = 0,
    Warning = 1,
    Error = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopScenedoctorValueclipsaction {
    AlertOnly = 0,
    FixAndAlert = 1,
    FixSilently = 2,
}

#[derive(Debug, Clone)]
pub struct LopScenedoctor {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopScenedoctor {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_validatekind(mut self, val: LopScenedoctorValidatekind) -> Self {
        self.params.insert(
            "validatekind".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_validatekind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "validatekind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindaction(mut self, val: LopScenedoctorKindaction) -> Self {
        self.params.insert(
            "kindaction".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_kindaction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kindaction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_validategprim(mut self, val: LopScenedoctorValidategprim) -> Self {
        self.params.insert(
            "validategprim".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_validategprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "validategprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gprimaction(mut self, val: LopScenedoctorGprimaction) -> Self {
        self.params.insert(
            "gprimaction".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_gprimaction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gprimaction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_validateprimvars(mut self, val: LopScenedoctorValidateprimvars) -> Self {
        self.params.insert(
            "validateprimvars".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_validateprimvars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "validateprimvars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primvarsaction(mut self, val: LopScenedoctorPrimvarsaction) -> Self {
        self.params.insert(
            "primvarsaction".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_primvarsaction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primvarsaction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_validatevalueclips(mut self, val: LopScenedoctorValidatevalueclips) -> Self {
        self.params.insert(
            "validatevalueclips".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_validatevalueclips_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "validatevalueclips".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_valueclipsaction(mut self, val: LopScenedoctorValueclipsaction) -> Self {
        self.params.insert(
            "valueclipsaction".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_valueclipsaction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "valueclipsaction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collectionname(mut self, val: &str) -> Self {
        self.params.insert(
            "collectionname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collectionname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collectionname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_python_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("python{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_python_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("python{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dovalidatekind(mut self, val: bool) -> Self {
        self.params.insert(
            "dovalidatekind".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dovalidatekind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dovalidatekind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dovalidategprim(mut self, val: bool) -> Self {
        self.params.insert(
            "dovalidategprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dovalidategprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dovalidategprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dovalidateprimvars(mut self, val: bool) -> Self {
        self.params.insert(
            "dovalidateprimvars".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dovalidateprimvars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dovalidateprimvars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dovalidatevalueclips(mut self, val: bool) -> Self {
        self.params.insert(
            "dovalidatevalueclips".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dovalidatevalueclips_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dovalidatevalueclips".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docollection(mut self, val: bool) -> Self {
        self.params.insert(
            "docollection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docollection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docollection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopScenedoctor {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "scenedoctor"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum LopSceneimportSopimportmode {
    Display = 0,
    Render = 1,
    Default = 2,
}

#[derive(Debug, Clone)]
pub struct LopSceneimport {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSceneimport {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_setmissingwidths(mut self, val: f32) -> Self {
        self.params.insert(
            "setmissingwidths".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setmissingwidths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_substeps(mut self, val: i32) -> Self {
        self.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substeps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_frange(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "frange".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_frange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sopimportmode(mut self, val: LopSceneimportSopimportmode) -> Self {
        self.params.insert(
            "sopimportmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sopimportmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sopimportmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_objdestpath(mut self, val: &str) -> Self {
        self.params.insert(
            "objdestpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objdestpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objdestpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rootobject(mut self, val: &str) -> Self {
        self.params.insert(
            "rootobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rootobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_objects(mut self, val: &str) -> Self {
        self.params.insert(
            "objects".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_forceobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "forceobjects".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_forceobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forceobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_excludeobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "excludeobjects".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_excludeobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "excludeobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter(mut self, val: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_collectionprim(mut self, val: &str) -> Self {
        self.params.insert(
            "collectionprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collectionprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collectionprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedusdhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "packedusdhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_packedusdhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "packedusdhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_otherprimhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "otherprimhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_otherprimhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "otherprimhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "packedhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_packedhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "packedhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_agenthandling(mut self, val: &str) -> Self {
        self.params.insert(
            "agenthandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_agenthandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "agenthandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nurbscurvehandling(mut self, val: &str) -> Self {
        self.params.insert(
            "nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nurbscurvehandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nurbssurfhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nurbssurfhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindschema(mut self, val: &str) -> Self {
        self.params.insert(
            "kindschema".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kindschema_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kindschema".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathattr(mut self, val: &str) -> Self {
        self.params.insert(
            "pathattr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subdgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "subdgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subdgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_authortimesamples(mut self, val: &str) -> Self {
        self.params.insert(
            "authortimesamples".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_authortimesamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "authortimesamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topologyhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "topologyhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topologyhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topologyhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribs(mut self, val: &str) -> Self {
        self.params.insert(
            "attribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indexattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "indexattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "constantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalarconstantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boolattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "boolattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boolattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uintattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "uintattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uintattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uintattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uint64attribs(mut self, val: &str) -> Self {
        self.params.insert(
            "uint64attribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uint64attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uint64attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_assetpathattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "assetpathattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_assetpathattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "assetpathattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_staticattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "staticattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "staticattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partitionattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "partitionattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partitionattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsetgroups(mut self, val: &str) -> Self {
        self.params.insert(
            "subsetgroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsetgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "customattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matdestpath(mut self, val: &str) -> Self {
        self.params.insert(
            "matdestpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matdestpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matdestpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_materials(mut self, val: &str) -> Self {
        self.params.insert(
            "materials".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_materials_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "materials".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_importalltimesamples(mut self, val: bool) -> Self {
        self.params.insert(
            "importalltimesamples".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importalltimesamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importalltimesamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importobjects(mut self, val: bool) -> Self {
        self.params.insert(
            "importobjects".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_includeinvisible(mut self, val: bool) -> Self {
        self.params.insert(
            "includeinvisible".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_includeinvisible_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "includeinvisible".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fixpathconflicts(mut self, val: bool) -> Self {
        self.params.insert(
            "fixpathconflicts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fixpathconflicts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fixpathconflicts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flatten(mut self, val: bool) -> Self {
        self.params.insert(
            "flatten".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flatten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flatten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ignoreinputs(mut self, val: bool) -> Self {
        self.params.insert(
            "ignoreinputs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ignoreinputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ignoreinputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importbundles(mut self, val: bool) -> Self {
        self.params.insert(
            "importbundles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importbundles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importbundles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importgeo(mut self, val: bool) -> Self {
        self.params.insert(
            "importgeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_asreference(mut self, val: bool) -> Self {
        self.params.insert(
            "asreference".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_asreference_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "asreference".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setsubdscheme(mut self, val: bool) -> Self {
        self.params.insert(
            "setsubdscheme".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setsubdscheme_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setsubdscheme".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_packedusdhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_packedusdhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_packedusdhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_packedusdhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_otherprimhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_otherprimhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_otherprimhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_otherprimhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_defineonlyleafprims(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_defineonlyleafprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defineonlyleafprims(mut self, val: bool) -> Self {
        self.params.insert(
            "defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_defineonlyleafprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_packedhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_packedhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_packedhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_packedhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_agenthandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_agenthandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_agenthandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_agenthandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_nurbscurvehandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_nurbscurvehandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_nurbssurfhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_nurbssurfhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_kindschema(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_kindschema".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_kindschema_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_kindschema".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_pathattr(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_pathattr".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_pathattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_pathattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_prefixabsolutepaths(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_prefixabsolutepaths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_prefixabsolutepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_prefixabsolutepaths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefixabsolutepaths(mut self, val: bool) -> Self {
        self.params.insert(
            "prefixabsolutepaths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefixabsolutepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefixabsolutepaths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_heightfieldconvert(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_heightfieldconvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_heightfieldconvert(mut self, val: bool) -> Self {
        self.params.insert(
            "heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_heightfieldconvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_polygonsassubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_polygonsassubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert(
            "polygonsassubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polygonsassubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_subdgroup(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_subdgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_subdgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_reversepolygons".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_reversepolygons".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert(
            "reversepolygons".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reversepolygons".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_setmissingwidths(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_setmissingwidths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_setmissingwidths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_setdefaultprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_setdefaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "setdefaultprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setdefaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_topologyhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_topologyhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_topologyhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_topologyhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_attribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_attribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_indexattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_indexattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_indexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_constantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_constantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_constantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_scalarconstantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_boolattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_boolattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_boolattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_uintattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_uintattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_uintattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_uintattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_uint64attribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_uint64attribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_uint64attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_uint64attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_assetpathattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_assetpathattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_assetpathattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_assetpathattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_staticattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_staticattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_staticattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_partitionattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_partitionattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_partitionattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_subsetgroups(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_subsetgroups".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_subsetgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_customattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_customattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_customattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_translateuvtost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_translateuvtost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "translateuvtost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "translateuvtost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importmats(mut self, val: bool) -> Self {
        self.params.insert(
            "importmats".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importmats_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importmats".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattenmats(mut self, val: bool) -> Self {
        self.params.insert(
            "flattenmats".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flattenmats_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattenmats".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importobjmats(mut self, val: bool) -> Self {
        self.params.insert(
            "importobjmats".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importobjmats_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importobjmats".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importsopmats(mut self, val: bool) -> Self {
        self.params.insert(
            "importsopmats".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importsopmats_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importsopmats".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collectionassignment(mut self, val: bool) -> Self {
        self.params.insert(
            "collectionassignment".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collectionassignment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collectionassignment".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbosemessages(mut self, val: bool) -> Self {
        self.params.insert(
            "verbosemessages".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_verbosemessages_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verbosemessages".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSceneimport {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sceneimport"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum LopScopeCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone)]
pub struct LopScope {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopScope {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float2 parameters ---
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primcount(mut self, val: i32) -> Self {
        self.params.insert(
            "primcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_createprims(mut self, val: LopScopeCreateprims) -> Self {
        self.params.insert(
            "createprims".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initforedit(mut self, val: i32) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_initforedit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primtype(mut self, val: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primkind(mut self, val: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specifier(mut self, val: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_specifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_classancestor(mut self, val: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_classancestor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopScope {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "scope"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum LopSetextentsMode {
    Automatic = 0,
    MinAndMax = 1,
    FromBounds = 2,
}

#[derive(Debug, Clone)]
pub struct LopSetextents {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSetextents {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float2 parameters ---
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_min(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "min".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "min".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_max(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "max".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: LopSetextentsMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primitives(mut self, val: &str) -> Self {
        self.params.insert(
            "primitives".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primitives_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primitives".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_worldspacebounds(mut self, val: bool) -> Self {
        self.params.insert(
            "worldspacebounds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_worldspacebounds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worldspacebounds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSetextents {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "setextents"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct LopSetvariant {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSetvariant {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_variantsetindex_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("variantsetindex{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_variantsetindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantsetindex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantnameindex_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("variantnameindex{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_variantnameindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantnameindex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_primpattern_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("primpattern{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primpattern{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantset_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("variantset{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_variantset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantset{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("variantname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_variantname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantsetuseindex_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("variantsetuseindex{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_variantsetuseindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantsetuseindex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantnameuseindex_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("variantnameuseindex{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_variantnameuseindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantnameuseindex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSetvariant {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "setvariant"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct LopShadowcatcher {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopShadowcatcher {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matpath(mut self, val: &str) -> Self {
        self.params.insert(
            "matpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_blocklights(mut self, val: bool) -> Self {
        self.params.insert(
            "blocklights".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_blocklights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blocklights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_replacematerial(mut self, val: bool) -> Self {
        self.params.insert(
            "replacematerial".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_replacematerial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "replacematerial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopShadowcatcher {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shadowcatcher"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct LopShopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopShopnet {
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

impl crate::core::types::HoudiniNode for LopShopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shopnet"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct LopShotlayeredit {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopShotlayeredit {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_newlayerindex(mut self, val: i32) -> Self {
        self.params.insert(
            "newlayerindex".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_newlayerindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newlayerindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_lop_activeshots(mut self, val: &str) -> Self {
        self.params.insert(
            "lop_activeshots".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lop_activeshots_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lop_activeshots".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fallback(mut self, val: &str) -> Self {
        self.params.insert(
            "fallback".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fallback_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fallback".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetlayer(mut self, val: &str) -> Self {
        self.params.insert(
            "targetlayer".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetlayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parentlayer(mut self, val: &str) -> Self {
        self.params.insert(
            "parentlayer".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentlayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_newlayer(mut self, val: &str) -> Self {
        self.params.insert(
            "newlayer".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_newlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newlayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablefallback(mut self, val: bool) -> Self {
        self.params.insert(
            "enablefallback".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefallback_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefallback".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clearsourcelayer(mut self, val: bool) -> Self {
        self.params.insert(
            "clearsourcelayer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clearsourcelayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clearsourcelayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createnewlayer(mut self, val: bool) -> Self {
        self.params.insert(
            "createnewlayer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createnewlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createnewlayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopShotlayeredit {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shotlayeredit"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("workspace")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopShotlayereditInnerExt {
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopShotlayereditInnerExt for crate::core::graph::InnerGraph<'a, LopShotlayeredit> {
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("workspace/output0")
    }
}

#[derive(Debug, Clone)]
pub struct LopShotload {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopShotload {
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
    pub fn trigger_reload(mut self) -> Self {
        self.params
            .insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- String parameters ---
    pub fn with_activeshots(mut self, val: &str) -> Self {
        self.params.insert(
            "activeshots".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_activeshots_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activeshots".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_descriptive(mut self, val: &str) -> Self {
        self.params.insert(
            "descriptive".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_descriptive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "descriptive".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_current_shot(mut self, val: &str) -> Self {
        self.params.insert(
            "current_shot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_current_shot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "current_shot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unitversionargs_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("unitversionargs{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_unitversionargs_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("unitversionargs{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_context_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("context{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_context_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("context{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_versionstring_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("versionstring{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_versionstring_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("versionstring{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_allowdirectedit(mut self, val: bool) -> Self {
        self.params.insert(
            "allowdirectedit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowdirectedit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowdirectedit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directedit(mut self, val: bool) -> Self {
        self.params.insert(
            "directedit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_directedit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directedit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clearsourcelayer(mut self, val: bool) -> Self {
        self.params.insert(
            "clearsourcelayer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clearsourcelayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clearsourcelayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopShotload {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shotload"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum LopShotoutputTrange {
    RenderCurrentFrame = 0,
    RenderSpecificFrameRange = 1,
    /// Render Specific Frame Range Only (Strict)
    RenderSpecificFrameRangeOnlyStrict = 2,
    RenderFrameRangeFromStage = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopShotoutputOutputmode {
    Render = 0,
    Publish = 1,
    PublishAndRender = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopShotoutputResScaleMenu {
    /// 1/10 (One Tenth Resolution)
    N110OneTenthResolution = 0,
    /// 1/5 (One Fifth Resolution)
    N15OneFifthResolution = 1,
    /// 1/4 (One Quarter Resolution)
    N14OneQuarterResolution = 2,
    /// 1/3 (One Third Resolution)
    N13OneThirdResolution = 3,
    /// 1/2 (One Half Resolution)
    N12OneHalfResolution = 4,
    /// 2/3 (Two Thirds Resolution)
    N23TwoThirdsResolution = 5,
    /// 3/4 (Three Quarters Resolution)
    N34ThreeQuartersResolution = 6,
    FullResolution = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopShotoutputHuskMplayzoommenu {
    /// 1/10 (One Tenth Resolution)
    N110OneTenthResolution = 0,
    /// 1/5 (One Fifth Resolution)
    N15OneFifthResolution = 1,
    /// 1/4 (One Quarter Resolution)
    N14OneQuarterResolution = 2,
    /// 1/3 (One Third Resolution)
    N13OneThirdResolution = 3,
    /// 1/2 (One Half Resolution)
    N12OneHalfResolution = 4,
    /// 2/3 (Two Thirds Resolution)
    N23TwoThirdsResolution = 5,
    /// 3/4 (Three Quarters Resolution)
    N34ThreeQuartersResolution = 6,
    FullResolution = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopShotoutputVerboseMenu {
    None = 0,
    RenderingStatistics = 1,
    FullVerbosity = 2,
    FullWithVexProfiling = 3,
    /// Full with VEX Profiling and NAN Checks (Slow)
    FullWithVexProfilingAndNanChecksSlow = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopShotoutputVexprofile {
    NoVexProfiling = 0,
    ExecutionProfiling = 1,
    /// Profiling and NAN Detection (Slow)
    ProfilingAndNanDetectionSlow = 2,
}

#[derive(Debug, Clone)]
pub struct LopShotoutput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopShotoutput {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert(
            "execute".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderpreview(mut self) -> Self {
        self.params.insert(
            "renderpreview".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_pdgexecute(mut self) -> Self {
        self.params.insert(
            "pdgexecute".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cook_controls_cancelcook(mut self) -> Self {
        self.params.insert(
            "_cook_controls_cancelcook".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_husk_tex_mempct(mut self, val: f32) -> Self {
        self.params.insert(
            "husk_tex_mempct".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_husk_tex_mempct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_mempct".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tex_filespct(mut self, val: f32) -> Self {
        self.params.insert(
            "husk_tex_filespct".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_husk_tex_filespct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_filespct".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tex_filesreserve(mut self, val: f32) -> Self {
        self.params.insert(
            "husk_tex_filesreserve".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_husk_tex_filesreserve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_filesreserve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionfloatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("optionfloatvalue{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_optionfloatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionfloatvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("f".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_foffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "foffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_foffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "foffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_res_scale(mut self, val: i32) -> Self {
        self.params.insert(
            "res_scale".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_res_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_restartdelegateframes(mut self, val: i32) -> Self {
        self.params.insert(
            "husk_restartdelegateframes".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_husk_restartdelegateframes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_restartdelegateframes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_snapshotinterval(mut self, val: i32) -> Self {
        self.params.insert(
            "snapshotinterval".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_snapshotinterval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "snapshotinterval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_timelimit(mut self, val: i32) -> Self {
        self.params.insert(
            "husk_timelimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_husk_timelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_timelimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_mplayzoom(mut self, val: i32) -> Self {
        self.params.insert(
            "husk_mplayzoom".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_husk_mplayzoom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_mplayzoom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tex_mem(mut self, val: i32) -> Self {
        self.params.insert(
            "husk_tex_mem".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_husk_tex_mem_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_mem".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tex_files(mut self, val: i32) -> Self {
        self.params.insert(
            "husk_tex_files".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_husk_tex_files_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_files".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tileindex(mut self, val: i32) -> Self {
        self.params.insert(
            "husk_tileindex".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_husk_tileindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tileindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbose(mut self, val: i32) -> Self {
        self.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_verbose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vexprofile(mut self, val: LopShotoutputVexprofile) -> Self {
        self.params.insert(
            "vexprofile".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_vexprofile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vexprofile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxthreads(mut self, val: i32) -> Self {
        self.params.insert(
            "maxthreads".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxthreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_res_user(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "res_user".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_res_user_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res_user".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tilemaxres(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "husk_tilemaxres".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_husk_tilemaxres_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tilemaxres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tilecount(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "husk_tilecount".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_husk_tilecount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tilecount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: LopShotoutputTrange) -> Self {
        self.params.insert(
            "trange".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputmode(mut self, val: LopShotoutputOutputmode) -> Self {
        self.params.insert(
            "outputmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_outputmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_res_scale_menu(mut self, val: LopShotoutputResScaleMenu) -> Self {
        self.params.insert(
            "res_scale_menu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_res_scale_menu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res_scale_menu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_res_usermenu(mut self, val: i32) -> Self {
        self.params.insert(
            "res_userMenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_res_usermenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "res_userMenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_mplayzoommenu(mut self, val: LopShotoutputHuskMplayzoommenu) -> Self {
        self.params.insert(
            "husk_mplayzoommenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_husk_mplayzoommenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_mplayzoommenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbose_menu(mut self, val: LopShotoutputVerboseMenu) -> Self {
        self.params.insert(
            "verbose_menu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_verbose_menu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verbose_menu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputprocessors(mut self, val: i32) -> Self {
        self.params.insert(
            "outputprocessors".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_outputprocessors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputprocessors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_shots(mut self, val: &str) -> Self {
        self.params.insert(
            "shots".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shots_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shots".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderer(mut self, val: &str) -> Self {
        self.params.insert(
            "renderer".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rendersettings(mut self, val: &str) -> Self {
        self.params.insert(
            "rendersettings".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rendersettings_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendersettings".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderpass(mut self, val: &str) -> Self {
        self.params.insert(
            "renderpass".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderpass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderpass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderpasses(mut self, val: &str) -> Self {
        self.params.insert(
            "renderpasses".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderpasses_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderpasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_override_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "override_camera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_override_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "override_camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputimage(mut self, val: &str) -> Self {
        self.params.insert(
            "outputimage".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputimage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_manualoutputimage(mut self, val: &str) -> Self {
        self.params.insert(
            "manualoutputimage".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_manualoutputimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "manualoutputimage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_override_res(mut self, val: &str) -> Self {
        self.params.insert(
            "override_res".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_override_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "override_res".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_headlight(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_headlight".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_headlight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_headlight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_populationmask(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_populationmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_populationmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_populationmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_purpose(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_purpose".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_purpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_purpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_complexity(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_complexity".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_complexity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_complexity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_metadata_key(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_metadata_key".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_metadata_key_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_metadata_key".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_metadata_value(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_metadata_value".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_metadata_value_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_metadata_value".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_mplayname(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_mplayname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_mplayname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_mplayname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_mplayaovs(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_mplayaovs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_mplayaovs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_mplayaovs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tex_res(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_tex_res".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_tex_res_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_res".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tex_memmode(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_tex_memmode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_tex_memmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_memmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tex_filemode(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_tex_filemode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_tex_filemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tex_filemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_prerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_prerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_preframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_preframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_presnapshot(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_presnapshot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_presnapshot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_presnapshot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_postsnapshot(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_postsnapshot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_postsnapshot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_postsnapshot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_postframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_postframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_postrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_postrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tiletempdir(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_tiletempdir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_tiletempdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tiletempdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tilesuffix(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_tilesuffix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_tilesuffix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tilesuffix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_windowsconsole(mut self, val: &str) -> Self {
        self.params.insert(
            "windowsconsole".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_windowsconsole_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "windowsconsole".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_usdtrace(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_usdtrace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_usdtrace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_usdtrace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_chromefile(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_chromefile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_chromefile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_chromefile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_stdout(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_stdout".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_stdout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_stdout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_stderr(mut self, val: &str) -> Self {
        self.params.insert(
            "husk_stderr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_stderr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_stderr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_source_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_source{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_source_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("husk_sc_source{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_file_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_file{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_file_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("husk_sc_file{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_cop_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_cop{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_cop_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("husk_sc_cop{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_filterlist_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_filterlist{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_filterlist_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("husk_sc_filterlist{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_label_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_label{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_label_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("husk_sc_label{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_in_aov_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_in{}_aov{}", index1, index2),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_in_aov_inst_expr(
        mut self,
        index1: usize,
        index2: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("husk_sc_in{}_aov{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_in_cop_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_in{}_cop{}", index1, index2),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_in_cop_inst_expr(
        mut self,
        index1: usize,
        index2: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("husk_sc_in{}_cop{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_out_cop_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_out{}_cop{}", index1, index2),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_out_cop_inst_expr(
        mut self,
        index1: usize,
        index2: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("husk_sc_out{}_cop{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_out_aov_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.params.insert(
            format!("husk_sc_out{}_aov{}", index1, index2),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_husk_sc_out_aov_inst_expr(
        mut self,
        index1: usize,
        index2: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("husk_sc_out{}_aov{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rendercommand(mut self, val: &str) -> Self {
        self.params.insert(
            "rendercommand".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rendercommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendercommand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolvercontextstringurlprefix_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringurlprefix{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resolvercontextstringurlprefix_inst_expr(
        mut self,
        index1: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("resolvercontextstringurlprefix{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolvercontextstringvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resolvercontextstringvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantselectionset_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("variantselectionset{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_variantselectionset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantselectionset{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantselectionvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("variantselectionvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_variantselectionvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantselectionvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lopoutput(mut self, val: &str) -> Self {
        self.params.insert(
            "lopoutput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lopoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lopoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deletefiles(mut self, val: &str) -> Self {
        self.params.insert(
            "deletefiles".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deletefiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deletefiles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savestyle(mut self, val: &str) -> Self {
        self.params.insert(
            "savestyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savestyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savestyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savepattern(mut self, val: &str) -> Self {
        self.params.insert(
            "savepattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savepattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savepattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savetodirectory_directory(mut self, val: &str) -> Self {
        self.params.insert(
            "savetodirectory_directory".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savetodirectory_directory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savetodirectory_directory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultprim(mut self, val: &str) -> Self {
        self.params.insert(
            "defaultprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_defaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optionname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optionname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optiontype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optiontype{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optiontype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optiontype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionstrvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("optionstrvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_optionstrvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionstrvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targettopnetwork(mut self, val: &str) -> Self {
        self.params.insert(
            "targettopnetwork".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targettopnetwork_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targettopnetwork".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_allframesatonce(mut self, val: bool) -> Self {
        self.params.insert(
            "allframesatonce".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allframesatonce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allframesatonce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_multishot(mut self, val: bool) -> Self {
        self.params.insert(
            "multishot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_multishot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "multishot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displaypdg(mut self, val: bool) -> Self {
        self.params.insert(
            "displaypdg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displaypdg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displaypdg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_gpu(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_gpu".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_gpu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_gpu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerenderpass(mut self, val: bool) -> Self {
        self.params.insert(
            "enablerenderpass".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerenderpass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablerenderpass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerenderpasses(mut self, val: bool) -> Self {
        self.params.insert(
            "enablerenderpasses".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerenderpasses_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablerenderpasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputimageshowwarning(mut self, val: bool) -> Self {
        self.params.insert(
            "outputimageshowwarning".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputimageshowwarning_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputimageshowwarning".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_instantshutter(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_instantshutter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_instantshutter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_instantshutter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_enable_headlight(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_enable_headlight".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_enable_headlight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_enable_headlight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_dopopulationmask(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_dopopulationmask".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_dopopulationmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_dopopulationmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_rendersettingsandcams(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_rendersettingsandcams".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_rendersettingsandcams_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_rendersettingsandcams".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_legacyexr(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_legacyexr".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_legacyexr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_legacyexr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_delegateprod(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_delegateprod".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_delegateprod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_delegateprod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_restartdelegate(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_restartdelegate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_restartdelegate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_restartdelegate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosnapshot(mut self, val: bool) -> Self {
        self.params.insert(
            "dosnapshot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosnapshot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dosnapshot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_dotimelimit(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_dotimelimit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_dotimelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_dotimelimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_timelimitperimage(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_timelimitperimage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_timelimitperimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_timelimitperimage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_mplay(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_mplay".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_mplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_mplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tprerender(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tprerender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tpreframe(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tpreframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tpresnapshot(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tpresnapshot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tpresnapshot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tpresnapshot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tpostsnapshot(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tpostsnapshot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tpostsnapshot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tpostsnapshot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tpostframe(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tpostframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tpostrender(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tpostrender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_tile(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_tile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_tile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_tile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_autotile(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_autotile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_autotile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_autotile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_debug(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_debug".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_debug_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_debug".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_log(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_log".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_log_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_log".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_logappend(mut self, val: bool) -> Self {
        self.params.insert(
            "husk_logappend".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_logappend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "husk_logappend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_husk_sc_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("husk_sc_enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_husk_sc_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("husk_sc_enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolvercontextstringenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("resolvercontextstringenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resolvercontextstringenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantselectionenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("variantselectionenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_variantselectionenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantselectionenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soho_foreground(mut self, val: bool) -> Self {
        self.params.insert(
            "soho_foreground".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_soho_foreground_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soho_foreground".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alfprogress(mut self, val: bool) -> Self {
        self.params.insert(
            "alfprogress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alfprogress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alfprogress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domaxthreads(mut self, val: bool) -> Self {
        self.params.insert(
            "domaxthreads".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domaxthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domaxthreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initsim(mut self, val: bool) -> Self {
        self.params.insert(
            "initsim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_initsim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initsim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reportnetwork(mut self, val: bool) -> Self {
        self.params.insert(
            "reportnetwork".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reportnetwork_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reportnetwork".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_striplayerbreaks(mut self, val: bool) -> Self {
        self.params.insert(
            "striplayerbreaks".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_striplayerbreaks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "striplayerbreaks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strippostlayers(mut self, val: bool) -> Self {
        self.params.insert(
            "strippostlayers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_strippostlayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strippostlayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trackprimexistence(mut self, val: bool) -> Self {
        self.params.insert(
            "trackprimexistence".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_trackprimexistence_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trackprimexistence".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usenetworksafesave(mut self, val: bool) -> Self {
        self.params.insert(
            "usenetworksafesave".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usenetworksafesave_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usenetworksafesave".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_errorsavingimplicitpaths(mut self, val: bool) -> Self {
        self.params.insert(
            "errorsavingimplicitpaths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_errorsavingimplicitpaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "errorsavingimplicitpaths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savefilesfromdisk(mut self, val: bool) -> Self {
        self.params.insert(
            "savefilesfromdisk".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savefilesfromdisk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savefilesfromdisk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattenfilelayers(mut self, val: bool) -> Self {
        self.params.insert(
            "flattenfilelayers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flattenfilelayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattenfilelayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flattensoplayers(mut self, val: bool) -> Self {
        self.params.insert(
            "flattensoplayers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flattensoplayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattensoplayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputprocessor_removehfs(mut self, val: bool) -> Self {
        self.params.insert(
            "outputprocessor_removehfs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputprocessor_removehfs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputprocessor_removehfs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableoutputprocessor_savetodirectory(mut self, val: bool) -> Self {
        self.params.insert(
            "enableoutputprocessor_savetodirectory".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputprocessor_savetodirectory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableoutputprocessor_savetodirectory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableoutputprocessor_simplerelativepaths(mut self, val: bool) -> Self {
        self.params.insert(
            "enableoutputprocessor_simplerelativepaths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputprocessor_simplerelativepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableoutputprocessor_simplerelativepaths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_requiredefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "requiredefaultprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_requiredefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requiredefaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savetimeinfo(mut self, val: bool) -> Self {
        self.params.insert(
            "savetimeinfo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savetimeinfo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savetimeinfo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clearhoudinicustomdata(mut self, val: bool) -> Self {
        self.params.insert(
            "clearhoudinicustomdata".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clearhoudinicustomdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clearhoudinicustomdata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ensuremetricsset(mut self, val: bool) -> Self {
        self.params.insert(
            "ensuremetricsset".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ensuremetricsset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ensuremetricsset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setropcook(mut self, val: bool) -> Self {
        self.params.insert(
            "setropcook".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setropcook_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setropcook".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_optionenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("optionenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_optionenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("optionenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tprerender(mut self, val: bool) -> Self {
        self.params.insert(
            "tprerender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tprerender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpreframe(mut self, val: bool) -> Self {
        self.params.insert(
            "tpreframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpreframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostframe(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tpostrender(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostrender".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostrender".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopShotoutput {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shotoutput"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct LopShotsplit {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopShotsplit {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_newlayerindex(mut self, val: i32) -> Self {
        self.params.insert(
            "newlayerindex".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_newlayerindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newlayerindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_targetlayer(mut self, val: &str) -> Self {
        self.params.insert(
            "targetlayer".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetlayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parentlayer(mut self, val: &str) -> Self {
        self.params.insert(
            "parentlayer".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentlayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_newlayer(mut self, val: &str) -> Self {
        self.params.insert(
            "newlayer".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_newlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newlayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputshotpattern_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("outputshotpattern{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputshotpattern_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("outputshotpattern{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultshot_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("defaultshot{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_defaultshot_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("defaultshot{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_description_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("description{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_description_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("description{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enabletargetlayer(mut self, val: bool) -> Self {
        self.params.insert(
            "enabletargetlayer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletargetlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabletargetlayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clearsourcelayer(mut self, val: bool) -> Self {
        self.params.insert(
            "clearsourcelayer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clearsourcelayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clearsourcelayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createnewlayer(mut self, val: bool) -> Self {
        self.params.insert(
            "createnewlayer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createnewlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createnewlayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopShotsplit {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shotsplit"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct LopShotswitch {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl LopShotswitch {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }

    // --- Int parameters ---
    pub fn with_input(mut self, val: i32) -> Self {
        self.params.insert(
            "input".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_input_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "input".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_shotsplitpath(mut self, val: &str) -> Self {
        self.params.insert(
            "shotsplitpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shotsplitpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shotsplitpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopShotswitch {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shotswitch"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum LopSimulationproxyConvexgranularity {
    ConvexHullPerPrimitive = 0,
    ConvexHullPerConnectedPiece = 1,
    DecomposeEachConnectedPiece = 2,
}

#[derive(Debug, Clone)]
pub struct LopSimulationproxy {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSimulationproxy {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert(
            "execute".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_percenttokeep(mut self, val: f32) -> Self {
        self.params.insert(
            "percenttokeep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_percenttokeep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "percenttokeep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_voxelsize(mut self, val: f32) -> Self {
        self.params.insert(
            "voxelsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_voxelsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voxelsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dilation(mut self, val: f32) -> Self {
        self.params.insert(
            "dilation".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dilation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dilation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vdbpercenttokeep(mut self, val: f32) -> Self {
        self.params.insert(
            "vdbpercenttokeep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vdbpercenttokeep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vdbpercenttokeep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxconcavity(mut self, val: f32) -> Self {
        self.params.insert(
            "maxconcavity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxconcavity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxconcavity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_finalpercenttokeep(mut self, val: f32) -> Self {
        self.params.insert(
            "finalpercenttokeep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_finalpercenttokeep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "finalpercenttokeep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_convexgranularity(mut self, val: LopSimulationproxyConvexgranularity) -> Self {
        self.params.insert(
            "convexgranularity".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_convexgranularity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convexgranularity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importtraversal(mut self, val: &str) -> Self {
        self.params.insert(
            "importtraversal".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_importtraversal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importtraversal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_file(mut self, val: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_excludeexisting(mut self, val: bool) -> Self {
        self.params.insert(
            "excludeexisting".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_excludeexisting_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "excludeexisting".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayproxy(mut self, val: bool) -> Self {
        self.params.insert(
            "displayproxy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayproxy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayproxy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loadfromdisk(mut self, val: bool) -> Self {
        self.params.insert(
            "loadfromdisk".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadfromdisk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loadfromdisk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_polyreduce(mut self, val: bool) -> Self {
        self.params.insert(
            "polyreduce".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_polyreduce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polyreduce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vdbresurface(mut self, val: bool) -> Self {
        self.params.insert(
            "vdbresurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vdbresurface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vdbresurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convex(mut self, val: bool) -> Self {
        self.params.insert(
            "convex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_convex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_finalpolyreduce(mut self, val: bool) -> Self {
        self.params.insert(
            "finalpolyreduce".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_finalpolyreduce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "finalpolyreduce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSimulationproxy {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "simulationproxy"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("modify/modify")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopSimulationproxyInnerExt {
    fn attribdelete1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopSimulationproxyInnerExt for crate::core::graph::InnerGraph<'a, LopSimulationproxy> {
    fn attribdelete1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("modify/modify/attribdelete1")
    }
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("modify/modify/output0")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSopcreateCopycontents {
    AddSopLayerAsNewSublayer = 0,
    MergeSopLayerIntoExistingActiveLayer = 1,
    CopySopLayerIntoNewActiveLayer = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSopcreateXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSopcreateRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopSopcreate {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSopcreate {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_fillmaterials(mut self) -> Self {
        self.params.insert(
            "fillmaterials".to_string(),
            crate::core::types::ParamValue::Button,
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
    pub fn with_setmissingwidths(mut self, val: f32) -> Self {
        self.params.insert(
            "setmissingwidths".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setmissingwidths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_copycontents(mut self, val: LopSopcreateCopycontents) -> Self {
        self.params.insert(
            "copycontents".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_copycontents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copycontents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: LopSopcreateXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: LopSopcreateRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_bindmaterials(mut self, val: &str) -> Self {
        self.params.insert(
            "bindmaterials".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindmaterials_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindmaterials".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reftype(mut self, val: &str) -> Self {
        self.params.insert(
            "reftype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reftype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reftype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refprim(mut self, val: &str) -> Self {
        self.params.insert(
            "refprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "refprimpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parentprimkind(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimkind".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentprimkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimkind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showlopstage(mut self, val: &str) -> Self {
        self.params.insert(
            "showlopstage".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_showlopstage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showlopstage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_grouptype(mut self, val: &str) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "pathprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savepath(mut self, val: &str) -> Self {
        self.params.insert(
            "savepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedusdhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "packedusdhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_packedusdhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "packedusdhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_otherprimhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "otherprimhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_otherprimhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "otherprimhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "packedhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_packedhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "packedhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nurbscurvehandling(mut self, val: &str) -> Self {
        self.params.insert(
            "nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nurbscurvehandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nurbssurfhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nurbssurfhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindschema(mut self, val: &str) -> Self {
        self.params.insert(
            "kindschema".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kindschema_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kindschema".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathattr(mut self, val: &str) -> Self {
        self.params.insert(
            "pathattr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subdgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "subdgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subdgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_authortimesamples(mut self, val: &str) -> Self {
        self.params.insert(
            "authortimesamples".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_authortimesamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "authortimesamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topologyhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "topologyhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topologyhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topologyhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribs(mut self, val: &str) -> Self {
        self.params.insert(
            "attribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indexattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "indexattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "constantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalarconstantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boolattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "boolattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boolattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uintattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "uintattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uintattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uintattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uint64attribs(mut self, val: &str) -> Self {
        self.params.insert(
            "uint64attribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uint64attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uint64attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_assetpathattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "assetpathattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_assetpathattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "assetpathattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_staticattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "staticattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "staticattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partitionattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "partitionattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partitionattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsetgroups(mut self, val: &str) -> Self {
        self.params.insert(
            "subsetgroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsetgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "customattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matnode_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("matnode{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matnode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("matnode{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("matpath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("matpath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geopath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("geopath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geopath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("geopath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_asreference(mut self, val: bool) -> Self {
        self.params.insert(
            "asreference".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_asreference_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "asreference".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ineditlayerblock(mut self, val: bool) -> Self {
        self.params.insert(
            "ineditlayerblock".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ineditlayerblock_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ineditlayerblock".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adjustxformsforinput(mut self, val: bool) -> Self {
        self.params.insert(
            "adjustxformsforinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adjustxformsforinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adjustxformsforinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instanceable(mut self, val: bool) -> Self {
        self.params.insert(
            "instanceable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_instanceable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instanceable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_group(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_group".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_grouptype(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_grouptype".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_pathprefix(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_pathprefix".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_pathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_pathprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_savepath(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_savepath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_savepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_savepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_packedusdhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_packedusdhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_packedusdhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_packedusdhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_otherprimhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_otherprimhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_otherprimhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_otherprimhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_defineonlyleafprims(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_defineonlyleafprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defineonlyleafprims(mut self, val: bool) -> Self {
        self.params.insert(
            "defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_defineonlyleafprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_packedhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_packedhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_packedhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_packedhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_nurbscurvehandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_nurbscurvehandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_nurbssurfhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_nurbssurfhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_kindschema(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_kindschema".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_kindschema_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_kindschema".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_pathattr(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_pathattr".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_pathattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_pathattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_prefixabsolutepaths(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_prefixabsolutepaths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_prefixabsolutepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_prefixabsolutepaths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefixabsolutepaths(mut self, val: bool) -> Self {
        self.params.insert(
            "prefixabsolutepaths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefixabsolutepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefixabsolutepaths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_heightfieldconvert(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_heightfieldconvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_heightfieldconvert(mut self, val: bool) -> Self {
        self.params.insert(
            "heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_heightfieldconvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_polygonsassubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_polygonsassubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert(
            "polygonsassubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polygonsassubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_subdgroup(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_subdgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_subdgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_reversepolygons".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_reversepolygons".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert(
            "reversepolygons".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reversepolygons".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_setmissingwidths(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_setmissingwidths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_setmissingwidths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_setdefaultprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_setdefaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "setdefaultprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setdefaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_topologyhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_topologyhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_topologyhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_topologyhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_attribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_attribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_indexattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_indexattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_indexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_constantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_constantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_constantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_scalarconstantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_boolattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_boolattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_boolattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_uintattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_uintattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_uintattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_uintattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_uint64attribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_uint64attribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_uint64attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_uint64attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_assetpathattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_assetpathattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_assetpathattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_assetpathattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_staticattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_staticattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_staticattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_partitionattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_partitionattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_partitionattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_subsetgroups(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_subsetgroups".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_subsetgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_customattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_customattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_customattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_translateuvtost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_translateuvtost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "translateuvtost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "translateuvtost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSopcreate {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sopcreate"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("sopnet/create")
    }
}

#[derive(Debug, Clone)]
pub struct LopSopcrowdimport {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSopcrowdimport {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("f".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "behavior".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "behavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_agentgeometry(mut self, val: &str) -> Self {
        self.params.insert(
            "agentgeometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_agentgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "agentgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geopurpose(mut self, val: &str) -> Self {
        self.params.insert(
            "geopurpose".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geopurpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geopurpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skelpurpose(mut self, val: &str) -> Self {
        self.params.insert(
            "skelpurpose".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skelpurpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skelpurpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skelvisibility(mut self, val: &str) -> Self {
        self.params.insert(
            "skelvisibility".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_skelvisibility_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skelvisibility".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "pathprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animsavepath(mut self, val: &str) -> Self {
        self.params.insert(
            "animsavepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animsavepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animsavepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geosavepath(mut self, val: &str) -> Self {
        self.params.insert(
            "geosavepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geosavepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geosavepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindmaterials(mut self, val: &str) -> Self {
        self.params.insert(
            "bindmaterials".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindmaterials_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindmaterials".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_attribs(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_attribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_indexattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_indexattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_indexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_constantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_constantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_constantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_scalarconstantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_boolattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_boolattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_boolattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_partitionattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_partitionattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_partitionattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_subsetgroups(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_subsetgroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_subsetgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_customattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeo_customattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeo_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_customattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_attribs(mut self, val: &str) -> Self {
        self.params.insert(
            "animgeo_attribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animgeo_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_constantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "animgeo_constantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animgeo_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_constantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_scalarconstantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "animgeo_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animgeo_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_boolattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "animgeo_boolattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animgeo_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_boolattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_staticattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "animgeo_staticattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animgeo_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_staticattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_customattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "animgeo_customattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_animgeo_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_customattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enableemission(mut self, val: bool) -> Self {
        self.params.insert(
            "enableemission".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableemission_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableemission".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_pathprefix(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_pathprefix".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_pathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_pathprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_animsavepath(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_animsavepath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_animsavepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_animsavepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_geosavepath(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_geosavepath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_geosavepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_geosavepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enableattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enableattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enableattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enableattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enableindexattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enableindexattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enableindexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enableindexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enableconstantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enableconstantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enableconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enableconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enablescalarconstantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enablescalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enablescalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enablescalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enableboolattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enableboolattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enableboolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enableboolattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enablepartitionattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enablepartitionattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enablepartitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enablepartitionattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enableprefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enableprefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enableprefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enableprefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enablesubsetgroups(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enablesubsetgroups".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enablesubsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enablesubsetgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enablecustomattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enablecustomattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enablecustomattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enablecustomattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_enabletranslateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_enabletranslateuvtost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_enabletranslateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_enabletranslateuvtost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeo_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "restgeo_translateuvtost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_restgeo_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeo_translateuvtost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_enableattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "animgeo_enableattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animgeo_enableattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_enableattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_enableconstantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "animgeo_enableconstantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animgeo_enableconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_enableconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_enablescalarconstantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "animgeo_enablescalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animgeo_enablescalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_enablescalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_enableboolattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "animgeo_enableboolattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animgeo_enableboolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_enableboolattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_enablestaticattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "animgeo_enablestaticattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animgeo_enablestaticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_enablestaticattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animgeo_enablecustomattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "animgeo_enablecustomattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animgeo_enablecustomattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animgeo_enablecustomattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSopcrowdimport {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sopcrowdimport"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum LopSopimportCopycontents {
    AddSopLayerAsNewSublayer = 0,
    MergeSopLayerIntoExistingActiveLayer = 1,
    CopySopLayerIntoNewActiveLayer = 2,
}

#[derive(Debug, Clone)]
pub struct LopSopimport {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSopimport {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_setmissingwidths(mut self, val: f32) -> Self {
        self.params.insert(
            "setmissingwidths".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setmissingwidths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_copycontents(mut self, val: LopSopimportCopycontents) -> Self {
        self.params.insert(
            "copycontents".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_copycontents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copycontents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindmaterials(mut self, val: &str) -> Self {
        self.params.insert(
            "bindmaterials".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindmaterials_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindmaterials".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reftype(mut self, val: &str) -> Self {
        self.params.insert(
            "reftype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reftype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reftype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refprim(mut self, val: &str) -> Self {
        self.params.insert(
            "refprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "refprimpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parentprimkind(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimkind".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentprimkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimkind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_grouptype(mut self, val: &str) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "pathprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savepath(mut self, val: &str) -> Self {
        self.params.insert(
            "savepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedusdhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "packedusdhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_packedusdhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "packedusdhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_otherprimhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "otherprimhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_otherprimhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "otherprimhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "packedhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_packedhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "packedhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_agenthandling(mut self, val: &str) -> Self {
        self.params.insert(
            "agenthandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_agenthandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "agenthandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nurbscurvehandling(mut self, val: &str) -> Self {
        self.params.insert(
            "nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nurbscurvehandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nurbssurfhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nurbssurfhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindschema(mut self, val: &str) -> Self {
        self.params.insert(
            "kindschema".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kindschema_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kindschema".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathattr(mut self, val: &str) -> Self {
        self.params.insert(
            "pathattr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subdgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "subdgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subdgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_authortimesamples(mut self, val: &str) -> Self {
        self.params.insert(
            "authortimesamples".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_authortimesamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "authortimesamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topologyhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "topologyhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topologyhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topologyhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribs(mut self, val: &str) -> Self {
        self.params.insert(
            "attribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indexattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "indexattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "constantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalarconstantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boolattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "boolattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boolattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uintattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "uintattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uintattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uintattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uint64attribs(mut self, val: &str) -> Self {
        self.params.insert(
            "uint64attribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uint64attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uint64attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_assetpathattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "assetpathattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_assetpathattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "assetpathattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_staticattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "staticattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "staticattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partitionattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "partitionattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partitionattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsetgroups(mut self, val: &str) -> Self {
        self.params.insert(
            "subsetgroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsetgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "customattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_asreference(mut self, val: bool) -> Self {
        self.params.insert(
            "asreference".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_asreference_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "asreference".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adjustxformsforinput(mut self, val: bool) -> Self {
        self.params.insert(
            "adjustxformsforinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adjustxformsforinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adjustxformsforinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instanceable(mut self, val: bool) -> Self {
        self.params.insert(
            "instanceable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_instanceable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instanceable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_group(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_group".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_grouptype(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_grouptype".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_pathprefix(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_pathprefix".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_pathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_pathprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_savepath(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_savepath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_savepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_savepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_packedusdhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_packedusdhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_packedusdhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_packedusdhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_otherprimhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_otherprimhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_otherprimhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_otherprimhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_defineonlyleafprims(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_defineonlyleafprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defineonlyleafprims(mut self, val: bool) -> Self {
        self.params.insert(
            "defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_defineonlyleafprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_packedhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_packedhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_packedhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_packedhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_agenthandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_agenthandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_agenthandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_agenthandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_nurbscurvehandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_nurbscurvehandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_nurbssurfhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_nurbssurfhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_kindschema(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_kindschema".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_kindschema_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_kindschema".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_pathattr(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_pathattr".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_pathattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_pathattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_prefixabsolutepaths(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_prefixabsolutepaths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_prefixabsolutepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_prefixabsolutepaths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefixabsolutepaths(mut self, val: bool) -> Self {
        self.params.insert(
            "prefixabsolutepaths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefixabsolutepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefixabsolutepaths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_heightfieldconvert(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_heightfieldconvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_heightfieldconvert(mut self, val: bool) -> Self {
        self.params.insert(
            "heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_heightfieldconvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_polygonsassubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_polygonsassubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert(
            "polygonsassubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polygonsassubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_subdgroup(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_subdgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_subdgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_reversepolygons".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_reversepolygons".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert(
            "reversepolygons".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reversepolygons".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_setmissingwidths(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_setmissingwidths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_setmissingwidths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_setdefaultprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_setdefaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "setdefaultprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setdefaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_topologyhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_topologyhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_topologyhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_topologyhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_attribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_attribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_indexattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_indexattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_indexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_constantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_constantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_constantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_scalarconstantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_boolattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_boolattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_boolattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_uintattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_uintattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_uintattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_uintattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_uint64attribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_uint64attribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_uint64attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_uint64attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_assetpathattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_assetpathattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_assetpathattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_assetpathattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_staticattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_staticattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_staticattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_partitionattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_partitionattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_partitionattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_subsetgroups(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_subsetgroups".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_subsetgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_customattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_customattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_customattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_translateuvtost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_translateuvtost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "translateuvtost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "translateuvtost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSopimport {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sopimport"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum LopSopmodifyTimesample {
    Static = 0,
    Animated = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSopmodifyViewportlod {
    FullGeometry = 0,
    PointCloud = 1,
    BoundingBox = 2,
    Centroid = 3,
    Hidden = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSopmodifyPivot {
    Origin = 0,
    Centroid = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSopmodifyCopycontents {
    AddSopLayerAsNewSublayer = 0,
    MergeSopLayerIntoExistingActiveLayer = 1,
    CopySopLayerIntoNewActiveLayer = 2,
}

#[derive(Debug, Clone)]
pub struct LopSopmodify {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSopmodify {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_importtime(mut self, val: f32) -> Self {
        self.params.insert(
            "importtime".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_importtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_staticimportframe(mut self, val: f32) -> Self {
        self.params.insert(
            "staticimportframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_staticimportframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "staticimportframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setmissingwidths(mut self, val: f32) -> Self {
        self.params.insert(
            "setmissingwidths".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setmissingwidths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_timesample(mut self, val: LopSopmodifyTimesample) -> Self {
        self.params.insert(
            "timesample".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_timesample_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timesample".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viewportlod(mut self, val: LopSopmodifyViewportlod) -> Self {
        self.params.insert(
            "viewportlod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viewportlod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viewportlod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pivot(mut self, val: LopSopmodifyPivot) -> Self {
        self.params.insert(
            "pivot".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pivot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pivot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copycontents(mut self, val: LopSopmodifyCopycontents) -> Self {
        self.params.insert(
            "copycontents".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_copycontents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copycontents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pathattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nameattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "nameattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nameattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importtraversal(mut self, val: &str) -> Self {
        self.params.insert(
            "importtraversal".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_importtraversal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importtraversal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_purpose(mut self, val: &str) -> Self {
        self.params.insert(
            "purpose".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_purpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "purpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unpack_pathattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "unpack_pathattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_unpack_pathattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unpack_pathattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unpack_nameattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "unpack_nameattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_unpack_nameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unpack_nameattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transfer_attrs(mut self, val: &str) -> Self {
        self.params.insert(
            "transfer_attrs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transfer_attrs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transfer_attrs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_import_primvars(mut self, val: &str) -> Self {
        self.params.insert(
            "import_primvars".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_import_primvars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "import_primvars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importattributes(mut self, val: &str) -> Self {
        self.params.insert(
            "importattributes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_importattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nontransformingprimvars(mut self, val: &str) -> Self {
        self.params.insert(
            "nontransformingprimvars".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nontransformingprimvars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nontransformingprimvars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindmaterials(mut self, val: &str) -> Self {
        self.params.insert(
            "bindmaterials".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindmaterials_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindmaterials".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reftype(mut self, val: &str) -> Self {
        self.params.insert(
            "reftype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reftype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reftype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refprim(mut self, val: &str) -> Self {
        self.params.insert(
            "refprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "refprimpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refprimpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parentprimkind(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimkind".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentprimkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimkind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showlopstage(mut self, val: &str) -> Self {
        self.params.insert(
            "showlopstage".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_showlopstage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showlopstage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_grouptype(mut self, val: &str) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "pathprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savepath(mut self, val: &str) -> Self {
        self.params.insert(
            "savepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedusdhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "packedusdhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_packedusdhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "packedusdhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_otherprimhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "otherprimhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_otherprimhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "otherprimhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_packedhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "packedhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_packedhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "packedhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nurbscurvehandling(mut self, val: &str) -> Self {
        self.params.insert(
            "nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nurbscurvehandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nurbssurfhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nurbssurfhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindschema(mut self, val: &str) -> Self {
        self.params.insert(
            "kindschema".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kindschema_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kindschema".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathattr(mut self, val: &str) -> Self {
        self.params.insert(
            "pathattr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subdgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "subdgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subdgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_authortimesamples(mut self, val: &str) -> Self {
        self.params.insert(
            "authortimesamples".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_authortimesamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "authortimesamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topologyhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "topologyhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topologyhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topologyhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribs(mut self, val: &str) -> Self {
        self.params.insert(
            "attribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indexattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "indexattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "constantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalarconstantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boolattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "boolattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boolattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uintattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "uintattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uintattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uintattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uint64attribs(mut self, val: &str) -> Self {
        self.params.insert(
            "uint64attribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uint64attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uint64attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_assetpathattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "assetpathattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_assetpathattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "assetpathattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_staticattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "staticattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "staticattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partitionattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "partitionattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partitionattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsetgroups(mut self, val: &str) -> Self {
        self.params.insert(
            "subsetgroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsetgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "customattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_addpathattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "addpathattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addpathattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addpathattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addnameattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "addnameattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addnameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addnameattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unpacktopolygons(mut self, val: bool) -> Self {
        self.params.insert(
            "unpacktopolygons".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unpacktopolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unpacktopolygons".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importinheritedprimvars(mut self, val: bool) -> Self {
        self.params.insert(
            "importinheritedprimvars".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importinheritedprimvars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importinheritedprimvars".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_translatesttouv(mut self, val: bool) -> Self {
        self.params.insert(
            "translatesttouv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_translatesttouv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "translatesttouv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_layerbreak(mut self, val: bool) -> Self {
        self.params.insert(
            "layerbreak".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_layerbreak_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "layerbreak".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_asreference(mut self, val: bool) -> Self {
        self.params.insert(
            "asreference".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_asreference_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "asreference".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ineditlayerblock(mut self, val: bool) -> Self {
        self.params.insert(
            "ineditlayerblock".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ineditlayerblock_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ineditlayerblock".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adjustxformsforinput(mut self, val: bool) -> Self {
        self.params.insert(
            "adjustxformsforinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adjustxformsforinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adjustxformsforinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_applyinverseusdxform(mut self, val: bool) -> Self {
        self.params.insert(
            "applyinverseusdxform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_applyinverseusdxform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "applyinverseusdxform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instanceable(mut self, val: bool) -> Self {
        self.params.insert(
            "instanceable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_instanceable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instanceable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_group(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_group".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_grouptype(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_grouptype".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_grouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_pathprefix(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_pathprefix".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_pathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_pathprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_savepath(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_savepath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_savepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_savepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_packedusdhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_packedusdhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_packedusdhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_packedusdhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_otherprimhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_otherprimhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_otherprimhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_otherprimhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_defineonlyleafprims(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_defineonlyleafprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defineonlyleafprims(mut self, val: bool) -> Self {
        self.params.insert(
            "defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_defineonlyleafprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defineonlyleafprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_packedhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_packedhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_packedhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_packedhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_nurbscurvehandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_nurbscurvehandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_nurbssurfhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_nurbssurfhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_kindschema(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_kindschema".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_kindschema_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_kindschema".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_pathattr(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_pathattr".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_pathattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_pathattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_heightfieldconvert(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_heightfieldconvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_heightfieldconvert(mut self, val: bool) -> Self {
        self.params.insert(
            "heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_heightfieldconvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "heightfieldconvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_polygonsassubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_polygonsassubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert(
            "polygonsassubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polygonsassubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_subdgroup(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_subdgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_subdgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_reversepolygons".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_reversepolygons".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert(
            "reversepolygons".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reversepolygons".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_setmissingwidths(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_setmissingwidths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_setmissingwidths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_setdefaultprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_setdefaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "setdefaultprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setdefaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_topologyhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_topologyhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_topologyhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_topologyhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_attribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_attribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_indexattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_indexattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_indexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_constantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_constantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_constantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_scalarconstantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_boolattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_boolattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_boolattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_uintattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_uintattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_uintattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_uintattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_uint64attribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_uint64attribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_uint64attribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_uint64attribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_assetpathattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_assetpathattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_assetpathattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_assetpathattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_staticattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_staticattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_staticattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_partitionattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_partitionattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_partitionattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_subsetgroups(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_subsetgroups".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_subsetgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_customattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_customattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_customattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_translateuvtost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_translateuvtost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "translateuvtost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "translateuvtost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSopmodify {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sopmodify"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("modify/modify")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopSopmodifyInnerExt {
    fn out(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopSopmodifyInnerExt for crate::core::graph::InnerGraph<'a, LopSopmodify> {
    fn out(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("modify/modify/OUT")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSopnetXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSopnetRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSopnetPreXform {
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
pub enum LopSopnetUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct LopSopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSopnet {
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
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "up".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_shop_materialopts(mut self, val: i32) -> Self {
        self.params.insert(
            "shop_materialopts".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_shop_materialopts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialopts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: LopSopnetXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: LopSopnetRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pre_xform(mut self, val: LopSopnetPreXform) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: LopSopnetUparmtype) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shop_materialpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showlopstage(mut self, val: &str) -> Self {
        self.params.insert(
            "showlopstage".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_showlopstage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showlopstage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_shadeopen(mut self, val: bool) -> Self {
        self.params.insert(
            "vport_shadeopen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_shadeopen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_shadeopen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vport_displayassubdiv(mut self, val: bool) -> Self {
        self.params.insert(
            "vport_displayassubdiv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vport_displayassubdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vport_displayassubdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sopnet"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum LopSphereCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSphereXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSphereRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopSphere {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSphere {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_ycb(mut self, val: f32) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_ycb".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_ycb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_ycb".to_string(),
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

    // --- Float2 parameters ---
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_p8a(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_p8a".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_p8a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_p8a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primcount(mut self, val: i32) -> Self {
        self.params.insert(
            "primcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_createprims(mut self, val: LopSphereCreateprims) -> Self {
        self.params.insert(
            "createprims".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initforedit(mut self, val: i32) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_initforedit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initforedit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: LopSphereXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: LopSphereRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primtype(mut self, val: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primkind(mut self, val: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specifier(mut self, val: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_specifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_classancestor(mut self, val: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_classancestor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classancestor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_radius_control(mut self, val: &str) -> Self {
        self.params.insert(
            "radius_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_radius_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radius_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_control_qmb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_control_qmb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsdisplaycolor_control_qmb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayColor_control_qmb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_control_zpb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_control_zpb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_primvarsdisplayopacity_control_zpb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__primvarsdisplayOpacity_control_zpb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doublesided_control(mut self, val: &str) -> Self {
        self.params.insert(
            "doubleSided_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_doublesided_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doubleSided_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_xformoptransform_control_6fb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_control_6fb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a(mut self, val: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xn_xformoptransform_51a_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xn__xformOptransform_51a".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computeextents(mut self, val: bool) -> Self {
        self.params.insert(
            "computeextents".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeextents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computeextents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doublesided(mut self, val: bool) -> Self {
        self.params.insert(
            "doubleSided".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doublesided_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doubleSided".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSphere {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sphere"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum LopSplitpointinstancersSplitmethod {
    Prototypes = 0,
    PointInstances = 1,
    Attribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSplitpointinstancersPiprunemethod {
    Hide = 0,
    Delete = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSplitpointinstancersProtoprunemethod {
    Hide = 0,
    Deactivate = 1,
}

#[derive(Debug, Clone)]
pub struct LopSplitpointinstancers {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSplitpointinstancers {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_splitmethod(mut self, val: LopSplitpointinstancersSplitmethod) -> Self {
        self.params.insert(
            "splitmethod".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_splitmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splitmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_piprunemethod(mut self, val: LopSplitpointinstancersPiprunemethod) -> Self {
        self.params.insert(
            "piprunemethod".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_piprunemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "piprunemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_protoprunemethod(mut self, val: LopSplitpointinstancersProtoprunemethod) -> Self {
        self.params.insert(
            "protoprunemethod".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_protoprunemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "protoprunemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_instances(mut self, val: &str) -> Self {
        self.params.insert(
            "instances".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_instances_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instances".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instancers(mut self, val: &str) -> Self {
        self.params.insert(
            "instancers".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_instancers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instancers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prototypes(mut self, val: &str) -> Self {
        self.params.insert(
            "prototypes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prototypes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prototypes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribute(mut self, val: &str) -> Self {
        self.params.insert(
            "attribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_duplicatename(mut self, val: &str) -> Self {
        self.params.insert(
            "duplicatename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_duplicatename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "duplicatename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_modifysource(mut self, val: &str) -> Self {
        self.params.insert(
            "modifysource".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_modifysource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modifysource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reftype(mut self, val: &str) -> Self {
        self.params.insert(
            "reftype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reftype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reftype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_perprototype(mut self, val: bool) -> Self {
        self.params.insert(
            "perprototype".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_perprototype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "perprototype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSplitpointinstancers {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "splitpointinstancers"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum LopSplitprimitiveSplitmode {
    GeometrySubset = 0,
    Primvar = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSplitprimitiveCopycontents {
    AddSopLayerAsNewSublayer = 0,
    MergeSopLayerIntoExistingActiveLayer = 1,
    CopySopLayerIntoNewActiveLayer = 2,
}

#[derive(Debug, Clone)]
pub struct LopSplitprimitive {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSplitprimitive {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_importtime(mut self, val: f32) -> Self {
        self.params.insert(
            "importtime".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_importtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setmissingwidths(mut self, val: f32) -> Self {
        self.params.insert(
            "setmissingwidths".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setmissingwidths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_splitmode(mut self, val: LopSplitprimitiveSplitmode) -> Self {
        self.params.insert(
            "splitmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_splitmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splitmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copycontents(mut self, val: LopSplitprimitiveCopycontents) -> Self {
        self.params.insert(
            "copycontents".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_copycontents_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copycontents".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_primitives(mut self, val: &str) -> Self {
        self.params.insert(
            "primitives".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primitives_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primitives".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_familyname(mut self, val: &str) -> Self {
        self.params.insert(
            "familyname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_familyname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "familyname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primvarname(mut self, val: &str) -> Self {
        self.params.insert(
            "primvarname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primvarname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primvarname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nurbscurvehandling(mut self, val: &str) -> Self {
        self.params.insert(
            "nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nurbscurvehandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nurbssurfhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nurbssurfhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kindschema(mut self, val: &str) -> Self {
        self.params.insert(
            "kindschema".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kindschema_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kindschema".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subdgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "subdgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subdgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_authortimesamples(mut self, val: &str) -> Self {
        self.params.insert(
            "authortimesamples".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_authortimesamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "authortimesamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topologyhandling(mut self, val: &str) -> Self {
        self.params.insert(
            "topologyhandling".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topologyhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topologyhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indexattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "indexattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "constantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalarconstantattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boolattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "boolattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boolattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_staticattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "staticattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "staticattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partitionattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "partitionattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partitionattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subsetgroups(mut self, val: &str) -> Self {
        self.params.insert(
            "subsetgroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subsetgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "customattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enable_nurbscurvehandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_nurbscurvehandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_nurbscurvehandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_nurbssurfhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_nurbssurfhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_nurbssurfhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_kindschema(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_kindschema".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_kindschema_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_kindschema".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_polygonsassubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_polygonsassubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_polygonsassubd(mut self, val: bool) -> Self {
        self.params.insert(
            "polygonsassubd".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_polygonsassubd_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polygonsassubd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_subdgroup(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_subdgroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_subdgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_subdgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_reversepolygons".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_reversepolygons".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reversepolygons(mut self, val: bool) -> Self {
        self.params.insert(
            "reversepolygons".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reversepolygons_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reversepolygons".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_setmissingwidths(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_setmissingwidths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_setmissingwidths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_setmissingwidths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_setdefaultprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_setdefaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setdefaultprim(mut self, val: bool) -> Self {
        self.params.insert(
            "setdefaultprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setdefaultprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setdefaultprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_topologyhandling(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_topologyhandling".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_topologyhandling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_topologyhandling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_indexattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_indexattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_indexattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_indexattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_constantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_constantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_constantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_constantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_scalarconstantattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_scalarconstantattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_scalarconstantattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_boolattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_boolattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_boolattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_boolattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_staticattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_staticattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_staticattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_staticattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_partitionattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_partitionattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_partitionattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_partitionattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefixpartitionsubsets(mut self, val: bool) -> Self {
        self.params.insert(
            "prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefixpartitionsubsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefixpartitionsubsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_subsetgroups(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_subsetgroups".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_subsetgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_subsetgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_customattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_customattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_customattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_customattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_translateuvtost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_translateuvtost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_translateuvtost(mut self, val: bool) -> Self {
        self.params.insert(
            "translateuvtost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_translateuvtost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "translateuvtost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSplitprimitive {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "splitprimitive"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct LopSplitscene {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSplitscene {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_common_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "common_primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_common_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "common_primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_flatteninput(mut self, val: bool) -> Self {
        self.params.insert(
            "flatteninput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flatteninput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flatteninput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSplitscene {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "splitscene"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub enum LopStagemanagerXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopStagemanagerRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopStagemanagerXord1 {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopStagemanagerRord1 {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopStagemanager {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopStagemanager {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_reload(mut self) -> Self {
        self.params
            .insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_apply(mut self) -> Self {
        self.params
            .insert("apply".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_reset(mut self) -> Self {
        self.params
            .insert("reset".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_removeunusedtransforms(mut self) -> Self {
        self.params.insert(
            "removeunusedtransforms".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Data parameters ---
    pub fn with_delta(mut self, val: &str) -> Self {
        self.params.insert(
            "delta".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_delta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "delta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_scale_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("scale{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("scale{}", index1),
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

    // --- Float3 parameters ---
    pub fn with_t_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("t{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_t_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("t{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("r{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("r{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("s{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("s{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shear_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("shear{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("shear{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("p{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("p{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("pr{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pr{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shear(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_xform_inst(mut self, val: i32) -> Self {
        self.params.insert(
            "xform_inst".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_xform_inst_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xform_inst".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_xord_inst(mut self, index1: usize, val: LopStagemanagerXord) -> Self {
        self.params.insert(
            format!("xOrd{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("xOrd{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord_inst(mut self, index1: usize, val: LopStagemanagerRord) -> Self {
        self.params.insert(
            format!("rOrd{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("rOrd{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: LopStagemanagerXord1) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: LopStagemanagerRord1) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_newprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "newprimtype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_newprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newprimtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_change_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("change{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_change_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("change{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kind_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("kind{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kind_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("kind{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("primpath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primpath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_destprimpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("destprimpath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_destprimpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("destprimpath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reffilepath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("reffilepath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_reffilepath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("reffilepath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refprimpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("refprimpath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_refprimpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("refprimpath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instanceable_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("instanceable{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_instanceable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("instanceable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantset_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.params.insert(
            format!("variantset{}_{}", index1, index2),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_variantset_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantset{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantname_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.params.insert(
            format!("variantname{}_{}", index1, index2),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_variantname_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantname{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_physprimpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "physprimpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_physprimpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "physprimpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bypassprimpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "bypassprimpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bypassprimpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bypassprimpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xformdescription(mut self, val: &str) -> Self {
        self.params.insert(
            "xformdescription".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xformdescription_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformdescription".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_flatteninputlayers(mut self, val: bool) -> Self {
        self.params.insert(
            "flatteninputlayers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flatteninputlayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flatteninputlayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aspayload_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("aspayload{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aspayload_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("aspayload{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setxform_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("setxform{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setxform_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("setxform{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visible_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("visible{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visible_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("visible{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_active_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("active{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_active_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("active{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doinstanceable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("doinstanceable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doinstanceable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("doinstanceable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localxform(mut self, val: bool) -> Self {
        self.params.insert(
            "localxform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_localxform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localxform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopStagemanager {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "stagemanager"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct LopStoreparametervalues {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopStoreparametervalues {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_float_2_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("float__2_{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_float_2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("float__2_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_float2_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(
            format!("float2__{}", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_float2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("float2__{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_float3_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("float3__{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_float3_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("float3__{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_float4_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("float4__{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_float4_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("float4__{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_int_2_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("int__2_{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_int_2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("int__2_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_int2_inst(mut self, index1: usize, val: [i32; 2]) -> Self {
        self.params.insert(
            format!("int2__{}", index1),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_int2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("int2__{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_int3_inst(mut self, index1: usize, val: [i32; 3]) -> Self {
        self.params.insert(
            format!("int3__{}", index1),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_int3_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("int3__{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int4 parameters ---
    pub fn with_int4_inst(mut self, index1: usize, val: [i32; 4]) -> Self {
        self.params.insert(
            format!("int4__{}", index1),
            crate::core::types::ParamValue::Int4(val),
        );
        self
    }
    pub fn with_int4_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("int4__{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("type{}", index1),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("type{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_valuestring_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("valuestring{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_valuestring_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("valuestring{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bool_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("bool_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bool_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bool_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopStoreparametervalues {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "storeparametervalues"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct LopSublayer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl LopSublayer {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }

    // --- Button parameters ---
    pub fn trigger_reload(mut self) -> Self {
        self.params
            .insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_timeoffset_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("timeoffset{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timeoffset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("timeoffset{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timescale_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("timescale{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("timescale{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_positionindex(mut self, val: i32) -> Self {
        self.params.insert(
            "positionindex".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_positionindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positionindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_setstagemetadata(mut self, val: &str) -> Self {
        self.params.insert(
            "setstagemetadata".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_setstagemetadata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setstagemetadata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_findsublayers(mut self, val: &str) -> Self {
        self.params.insert(
            "findsublayers".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_findsublayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "findsublayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sublayertype(mut self, val: &str) -> Self {
        self.params.insert(
            "sublayertype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sublayertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sublayertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_handlemissingfiles(mut self, val: &str) -> Self {
        self.params.insert(
            "handlemissingfiles".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_handlemissingfiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handlemissingfiles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_positiontype(mut self, val: &str) -> Self {
        self.params.insert(
            "positiontype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_positiontype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positiontype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filepath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("filepath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filepath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filepath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_loadpayloads(mut self, val: bool) -> Self {
        self.params.insert(
            "loadpayloads".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadpayloads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loadpayloads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_editrootlayer(mut self, val: bool) -> Self {
        self.params.insert(
            "editrootlayer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_editrootlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "editrootlayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removefoundsublayers(mut self, val: bool) -> Self {
        self.params.insert(
            "removefoundsublayers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removefoundsublayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "removefoundsublayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mute_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("mute{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mute_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("mute{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable(mut self, val: bool) -> Self {
        self.params.insert(
            "enable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSublayer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sublayer"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
pub struct LopSubnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSubnet {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Input 2"
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Input 2" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Input 3"
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Input 3" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Input 4"
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Input 4" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }
}

impl crate::core::types::HoudiniNode for LopSubnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "subnet"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopSubnetInnerExt {
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopSubnetInnerExt for crate::core::graph::InnerGraph<'a, LopSubnet> {
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("output0")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSurfaceconstraintSourcetype {
    Primitives = 0,
    PointInstancer = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSurfaceconstraintTargetsource {
    FirstInput = 0,
    SecondInput = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSurfaceconstraintTargettype {
    Primitive = 0,
    PointInstancer = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSurfaceconstraintMode {
    /// UV (Sticky)
    UvSticky = 0,
    /// Primitive + UV
    PrimitivePlusUv = 1,
    PointGroup = 2,
    PrimitiveGroup = 3,
    ClosestDistance = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSurfaceconstraintLookatmode {
    None = 0,
    DirectionAttributeFromGeometry = 1,
    UDirection = 2,
    VDirection = 3,
    FromPrimitive = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSurfaceconstraintLookatsource {
    FirstInput = 0,
    SecondInput = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSurfaceconstraintLookupmode {
    UpVectorAttributeFromGeometry = 0,
    UDirection = 1,
    VDirection = 2,
    XAxis = 3,
    YAxis = 4,
    ZAxis = 5,
    FromPrimitive = 6,
    Custom = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSurfaceconstraintUpvectorsource {
    FirstInput = 0,
    SecondInput = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSurfaceconstraintLookataxis {
    /// X-
    XMinus = 0,
    /// Y-
    YMinus = 1,
    /// Z-
    ZMinus = 2,
    /// X+
    XPlus = 3,
    /// Y+
    YPlus = 4,
    /// Z+
    ZPlus = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSurfaceconstraintLookupaxisx {
    /// Y-
    YMinus = 0,
    /// Z-
    ZMinus = 1,
    /// Y+
    YPlus = 2,
    /// Z+
    ZPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSurfaceconstraintLookupaxisy {
    /// X-
    XMinus = 0,
    /// Z-
    ZMinus = 1,
    /// X+
    XPlus = 2,
    /// Z+
    ZPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSurfaceconstraintLookupaxisz {
    /// X-
    XMinus = 0,
    /// Y-
    YMinus = 1,
    /// X+
    XPlus = 2,
    /// Y+
    YPlus = 3,
}

#[derive(Debug, Clone)]
pub struct LopSurfaceconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopSurfaceconstraint {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Stage Containing Target Primitives (optional)"
    pub fn set_input_stage_containing_target_primitives_optio<
        N: crate::core::types::HoudiniNode,
    >(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Stage Containing Target Primitives (optional)" and specifies the output index of the target node.
    pub fn set_input_stage_containing_target_primitives_optio_from<
        N: crate::core::types::HoudiniNode,
    >(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_importtime(mut self, val: f32) -> Self {
        self.params.insert(
            "importtime".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_importtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uv(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "uv".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_uv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upvector(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "upvector".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upvector_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvector".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcetype(mut self, val: LopSurfaceconstraintSourcetype) -> Self {
        self.params.insert(
            "sourcetype".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_sourcetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetsource(mut self, val: LopSurfaceconstraintTargetsource) -> Self {
        self.params.insert(
            "targetsource".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_targetsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targettype(mut self, val: LopSurfaceconstraintTargettype) -> Self {
        self.params.insert(
            "targettype".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_targettype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targettype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatmode(mut self, val: LopSurfaceconstraintLookatmode) -> Self {
        self.params.insert(
            "lookatmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_lookatmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatsource(mut self, val: LopSurfaceconstraintLookatsource) -> Self {
        self.params.insert(
            "lookatsource".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_lookatsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupmode(mut self, val: LopSurfaceconstraintLookupmode) -> Self {
        self.params.insert(
            "lookupmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_lookupmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upvectorsource(mut self, val: LopSurfaceconstraintUpvectorsource) -> Self {
        self.params.insert(
            "upvectorsource".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_upvectorsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvectorsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_mode(mut self, val: LopSurfaceconstraintMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookataxis(mut self, val: LopSurfaceconstraintLookataxis) -> Self {
        self.params.insert(
            "lookataxis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookataxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookataxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisx(mut self, val: LopSurfaceconstraintLookupaxisx) -> Self {
        self.params.insert(
            "lookupaxisx".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupaxisx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisy(mut self, val: LopSurfaceconstraintLookupaxisy) -> Self {
        self.params.insert(
            "lookupaxisy".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupaxisy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisz(mut self, val: LopSurfaceconstraintLookupaxisz) -> Self {
        self.params.insert(
            "lookupaxisz".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupaxisz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source(mut self, val: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_source_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourceinstances(mut self, val: &str) -> Self {
        self.params.insert(
            "sourceinstances".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourceinstances_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceinstances".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_target(mut self, val: &str) -> Self {
        self.params.insert(
            "target".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_target_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "target".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetinstance(mut self, val: &str) -> Self {
        self.params.insert(
            "targetinstance".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetinstance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetinstance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_uvattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "uvattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "pattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_positionsnippet(mut self, val: &str) -> Self {
        self.params.insert(
            "positionsnippet".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_positionsnippet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positionsnippet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatxform(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatxform".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookatxform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatxform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upvectorxform(mut self, val: &str) -> Self {
        self.params.insert(
            "upvectorxform".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_upvectorxform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upvectorxform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dirattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "dirattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dirattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dirattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "upattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_upattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rollsnippet(mut self, val: &str) -> Self {
        self.params.insert(
            "rollsnippet".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rollsnippet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rollsnippet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hidetarget(mut self, val: bool) -> Self {
        self.params.insert(
            "hidetarget".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hidetarget_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hidetarget".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepoffset(mut self, val: bool) -> Self {
        self.params.insert(
            "keepoffset".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_position(mut self, val: bool) -> Self {
        self.params.insert(
            "position".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotation(mut self, val: bool) -> Self {
        self.params.insert(
            "rotation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale(mut self, val: bool) -> Self {
        self.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
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
    pub fn with_shear(mut self, val: bool) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subdi(mut self, val: bool) -> Self {
        self.params.insert(
            "subdi".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_subdi_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subdi".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepositionsnippet(mut self, val: bool) -> Self {
        self.params.insert(
            "usepositionsnippet".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepositionsnippet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepositionsnippet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_userollsnippet(mut self, val: bool) -> Self {
        self.params.insert(
            "userollsnippet".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userollsnippet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "userollsnippet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSurfaceconstraint {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "surfaceconstraint"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopSurfaceconstraintInnerExt {
    fn to_sop(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn to_sop_2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn add_to_creatornodes(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn apply_xform(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn apply_xform_instances(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_begin1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn foreach_end1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn objnet(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn output(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn prune(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn sopnet(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn storeglobals(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_hide_targets(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_instances(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_no_target(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn warning(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopSurfaceconstraintInnerExt for crate::core::graph::InnerGraph<'a, LopSurfaceconstraint> {
    fn to_sop(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("TO_SOP")
    }
    fn to_sop_2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("TO_SOP_2")
    }
    fn add_to_creatornodes(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("add_to_creatorNodes")
    }
    fn apply_xform(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("apply_xform")
    }
    fn apply_xform_instances(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("apply_xform_instances")
    }
    fn foreach_begin1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_begin1")
    }
    fn foreach_end1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("foreach_end1")
    }
    fn objnet(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("objnet")
    }
    fn output(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("output")
    }
    fn prune(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("prune")
    }
    fn sopnet(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("sopnet")
    }
    fn storeglobals(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("storeglobals")
    }
    fn switch_hide_targets(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch_hide_targets")
    }
    fn switch_instances(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch_instances")
    }
    fn switch_no_target(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("switch_no_target")
    }
    fn warning(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("warning")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopSwitchChooseinputbyname {
    ByIndex = 0,
    ByName = 1,
    ByPatternMatching = 2,
}

#[derive(Debug, Clone)]
pub struct LopSwitch {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl LopSwitch {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }

    // --- Int parameters ---
    pub fn with_input(mut self, val: i32) -> Self {
        self.params.insert(
            "input".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_input_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "input".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fallback(mut self, val: i32) -> Self {
        self.params.insert(
            "fallback".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_fallback_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fallback".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_chooseinputbyname(mut self, val: LopSwitchChooseinputbyname) -> Self {
        self.params.insert(
            "chooseinputbyname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_chooseinputbyname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "chooseinputbyname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_selectinputname(mut self, val: &str) -> Self {
        self.params.insert(
            "selectinputname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_selectinputname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "selectinputname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_selectinputvalue(mut self, val: &str) -> Self {
        self.params.insert(
            "selectinputvalue".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_selectinputvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "selectinputvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_badinput(mut self, val: &str) -> Self {
        self.params.insert(
            "badinput".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_badinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "badinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_selectfallbackname(mut self, val: &str) -> Self {
        self.params.insert(
            "selectfallbackname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_selectfallbackname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "selectfallbackname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputname(mut self, val: &str) -> Self {
        self.params.insert(
            "inputname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopSwitch {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "switch"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
