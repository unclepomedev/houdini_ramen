#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopHeatdistortGlobalelementsizetype {
    /// Per-Component Controls
    PerMinusComponentControls = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopHeatdistortElementsizetype {
    /// Per-Component Controls
    PerMinusComponentControls = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopHeatdistortDetailelementsizetype {
    /// Per-Component Controls
    PerMinusComponentControls = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopHeatdistortSourceblursizetype {
    /// Per-Component Control
    PerMinusComponentControl = 0,
}

#[derive(Debug, Clone)]
pub struct CopHeatdistort {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopHeatdistort {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_mask(mut self, val: f32) -> Self {
        self.params.insert("mask".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_globaldistortscale(mut self, val: f32) -> Self {
        self.params.insert("globaldistortscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_globaldistortscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("globaldistortscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_globalelementsize(mut self, val: f32) -> Self {
        self.params.insert("globalelementsize".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_globalelementsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("globalelementsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_angle(mut self, val: f32) -> Self {
        self.params.insert("angle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert("angle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_elementsize(mut self, val: f32) -> Self {
        self.params.insert("elementsize".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_elementsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("elementsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_detailscale(mut self, val: f32) -> Self {
        self.params.insert("detailscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_detailscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("detailscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_detailelementsize(mut self, val: f32) -> Self {
        self.params.insert("detailelementsize".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_detailelementsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("detailelementsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cutoff(mut self, val: f32) -> Self {
        self.params.insert("cutoff".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_cutoff_expr(mut self, expr: &str) -> Self {
        self.params.insert("cutoff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rough(mut self, val: f32) -> Self {
        self.params.insert("rough".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rough_expr(mut self, expr: &str) -> Self {
        self.params.insert("rough".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sourceblursize(mut self, val: f32) -> Self {
        self.params.insert("sourceblursize".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_sourceblursize_expr(mut self, expr: &str) -> Self {
        self.params.insert("sourceblursize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cascale(mut self, val: f32) -> Self {
        self.params.insert("cascale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_cascale_expr(mut self, expr: &str) -> Self {
        self.params.insert("cascale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_carshift(mut self, val: f32) -> Self {
        self.params.insert("carshift".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_carshift_expr(mut self, expr: &str) -> Self {
        self.params.insert("carshift".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cagshift(mut self, val: f32) -> Self {
        self.params.insert("cagshift".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_cagshift_expr(mut self, expr: &str) -> Self {
        self.params.insert("cagshift".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cabshift(mut self, val: f32) -> Self {
        self.params.insert("cabshift".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_cabshift_expr(mut self, expr: &str) -> Self {
        self.params.insert("cabshift".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_timeoffset(mut self, val: f32) -> Self {
        self.params.insert("timeoffset".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_timeoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert("timeoffset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pulselength(mut self, val: f32) -> Self {
        self.params.insert("pulselength".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pulselength_expr(mut self, expr: &str) -> Self {
        self.params.insert("pulselength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_diranimangle(mut self, val: f32) -> Self {
        self.params.insert("diranimangle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_diranimangle_expr(mut self, expr: &str) -> Self {
        self.params.insert("diranimangle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_diranimspeed(mut self, val: f32) -> Self {
        self.params.insert("diranimspeed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_diranimspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert("diranimspeed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_globalelementscale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("globalelementscale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_globalelementscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("globalelementscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_elementscale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("elementscale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_elementscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("elementscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_detailelementscale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("detailelementscale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_detailelementscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("detailelementscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sourceblurscale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("sourceblurscale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_sourceblurscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("sourceblurscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_globalelementsizetype(mut self, val: CopHeatdistortGlobalelementsizetype) -> Self {
        self.params.insert("globalelementsizetype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_globalelementsizetype_expr(mut self, expr: &str) -> Self {
        self.params.insert("globalelementsizetype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_elementsizetype(mut self, val: CopHeatdistortElementsizetype) -> Self {
        self.params.insert("elementsizetype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_elementsizetype_expr(mut self, expr: &str) -> Self {
        self.params.insert("elementsizetype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_detailelementsizetype(mut self, val: CopHeatdistortDetailelementsizetype) -> Self {
        self.params.insert("detailelementsizetype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_detailelementsizetype_expr(mut self, expr: &str) -> Self {
        self.params.insert("detailelementsizetype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sourceblursizetype(mut self, val: CopHeatdistortSourceblursizetype) -> Self {
        self.params.insert("sourceblursizetype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sourceblursizetype_expr(mut self, expr: &str) -> Self {
        self.params.insert("sourceblursizetype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert("signature".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert("signature".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_usecfl(mut self, val: bool) -> Self {
        self.params.insert("usecfl".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usecfl_expr(mut self, expr: &str) -> Self {
        self.params.insert("usecfl".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_streakblur(mut self, val: bool) -> Self {
        self.params.insert("streakblur".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_streakblur_expr(mut self, expr: &str) -> Self {
        self.params.insert("streakblur".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scalebytimeinc(mut self, val: bool) -> Self {
        self.params.insert("scalebytimeinc".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_scalebytimeinc_expr(mut self, expr: &str) -> Self {
        self.params.insert("scalebytimeinc".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dodistort(mut self, val: bool) -> Self {
        self.params.insert("dodistort".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dodistort_expr(mut self, expr: &str) -> Self {
        self.params.insert("dodistort".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dodetaildistort(mut self, val: bool) -> Self {
        self.params.insert("dodetaildistort".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dodetaildistort_expr(mut self, expr: &str) -> Self {
        self.params.insert("dodetaildistort".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dosourceblur(mut self, val: bool) -> Self {
        self.params.insert("dosourceblur".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dosourceblur_expr(mut self, expr: &str) -> Self {
        self.params.insert("dosourceblur".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_doca(mut self, val: bool) -> Self {
        self.params.insert("doca".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_doca_expr(mut self, expr: &str) -> Self {
        self.params.insert("doca".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dopulselength(mut self, val: bool) -> Self {
        self.params.insert("dopulselength".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dopulselength_expr(mut self, expr: &str) -> Self {
        self.params.insert("dopulselength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dodiranim(mut self, val: bool) -> Self {
        self.params.insert("dodiranim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dodiranim_expr(mut self, expr: &str) -> Self {
        self.params.insert("dodiranim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for CopHeatdistort {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heatdistort"
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
pub enum CopHeatdistortbylayerSourceblursizetype {
    /// Per-Component Control
    PerMinusComponentControl = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopHeatdistortbylayerNoiseblursizetype {
    /// Per-Component Control
    PerMinusComponentControl = 0,
}

#[derive(Debug, Clone)]
pub struct CopHeatdistortbylayer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopHeatdistortbylayer {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_mask(mut self, val: f32) -> Self {
        self.params.insert("mask".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_angle(mut self, val: f32) -> Self {
        self.params.insert("angle".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_angle_expr(mut self, expr: &str) -> Self {
        self.params.insert("angle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sourceblursize(mut self, val: f32) -> Self {
        self.params.insert("sourceblursize".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_sourceblursize_expr(mut self, expr: &str) -> Self {
        self.params.insert("sourceblursize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_noiseblursize(mut self, val: f32) -> Self {
        self.params.insert("noiseblursize".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_noiseblursize_expr(mut self, expr: &str) -> Self {
        self.params.insert("noiseblursize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cascale(mut self, val: f32) -> Self {
        self.params.insert("cascale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_cascale_expr(mut self, expr: &str) -> Self {
        self.params.insert("cascale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_carshift(mut self, val: f32) -> Self {
        self.params.insert("carshift".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_carshift_expr(mut self, expr: &str) -> Self {
        self.params.insert("carshift".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cagshift(mut self, val: f32) -> Self {
        self.params.insert("cagshift".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_cagshift_expr(mut self, expr: &str) -> Self {
        self.params.insert("cagshift".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cabshift(mut self, val: f32) -> Self {
        self.params.insert("cabshift".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_cabshift_expr(mut self, expr: &str) -> Self {
        self.params.insert("cabshift".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_sourceblurscale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("sourceblurscale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_sourceblurscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("sourceblurscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_noiseblurscale(mut self, val: [f32; 2]) -> Self {
        self.params.insert("noiseblurscale".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_noiseblurscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("noiseblurscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_sourceblursizetype(mut self, val: CopHeatdistortbylayerSourceblursizetype) -> Self {
        self.params.insert("sourceblursizetype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sourceblursizetype_expr(mut self, expr: &str) -> Self {
        self.params.insert("sourceblursizetype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_noiseblursizetype(mut self, val: CopHeatdistortbylayerNoiseblursizetype) -> Self {
        self.params.insert("noiseblursizetype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_noiseblursizetype_expr(mut self, expr: &str) -> Self {
        self.params.insert("noiseblursizetype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert("signature".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert("signature".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_usecfl(mut self, val: bool) -> Self {
        self.params.insert("usecfl".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usecfl_expr(mut self, expr: &str) -> Self {
        self.params.insert("usecfl".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_streakblur(mut self, val: bool) -> Self {
        self.params.insert("streakblur".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_streakblur_expr(mut self, expr: &str) -> Self {
        self.params.insert("streakblur".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scalebytimeinc(mut self, val: bool) -> Self {
        self.params.insert("scalebytimeinc".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_scalebytimeinc_expr(mut self, expr: &str) -> Self {
        self.params.insert("scalebytimeinc".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dosourceblur(mut self, val: bool) -> Self {
        self.params.insert("dosourceblur".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dosourceblur_expr(mut self, expr: &str) -> Self {
        self.params.insert("dosourceblur".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_donoiseblur(mut self, val: bool) -> Self {
        self.params.insert("donoiseblur".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_donoiseblur_expr(mut self, expr: &str) -> Self {
        self.params.insert("donoiseblur".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_doca(mut self, val: bool) -> Self {
        self.params.insert("doca".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_doca_expr(mut self, expr: &str) -> Self {
        self.params.insert("doca".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for CopHeatdistortbylayer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heatdistortbylayer"
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
pub struct CopHeightfieldVisualize {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopHeightfieldVisualize {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_heightscale(mut self, val: f32) -> Self {
        self.params.insert("heightscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_heightscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightoffset(mut self, val: f32) -> Self {
        self.params.insert("heightoffset".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_heightoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightoffset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for CopHeightfieldVisualize {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heightfield_visualize"
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
pub struct CopHeighttoambientocclusion {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopHeighttoambientocclusion {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_heightscale(mut self, val: f32) -> Self {
        self.params.insert("heightscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_heightscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_viewradius(mut self, val: f32) -> Self {
        self.params.insert("viewradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_viewradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("viewradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stepscale(mut self, val: f32) -> Self {
        self.params.insert("stepscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stepscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("stepscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_axiscount(mut self, val: i32) -> Self {
        self.params.insert("axiscount".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_axiscount_expr(mut self, expr: &str) -> Self {
        self.params.insert("axiscount".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Ramp parameters ---
    pub fn with_remap(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert("remap".to_string(), crate::core::types::ParamValue::Ramp(val));
        self
    }
    pub fn with_remap_expr(mut self, expr: &str) -> Self {
        self.params.insert("remap".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_dohemisphere(mut self, val: bool) -> Self {
        self.params.insert("dohemisphere".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dohemisphere_expr(mut self, expr: &str) -> Self {
        self.params.insert("dohemisphere".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for CopHeighttoambientocclusion {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heighttoambientocclusion"
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
pub enum CopHeighttonormalNormaltype {
    /// Signed (-1 to 1)
    SignedMinus1To1 = 0,
    /// Offset (0 to 1)
    Offset0To1 = 1,
}

#[derive(Debug, Clone)]
pub struct CopHeighttonormal {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopHeighttonormal {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_kernel(mut self, val: i32) -> Self {
        self.params.insert("kernel".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_kernel_expr(mut self, expr: &str) -> Self {
        self.params.insert("kernel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_normaltype(mut self, val: CopHeighttonormalNormaltype) -> Self {
        self.params.insert("normaltype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_normaltype_expr(mut self, expr: &str) -> Self {
        self.params.insert("normaltype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_readoutside(mut self, val: bool) -> Self {
        self.params.insert("readoutside".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_readoutside_expr(mut self, expr: &str) -> Self {
        self.params.insert("readoutside".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for CopHeighttonormal {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heighttonormal"
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
pub enum CopHeighttoshadowType {
    Disk = 0,
    Sphere = 1,
    Directional = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopHeighttoshadowUse {
    /// Azimuth/Altitude/Distance
    AzimuthAltitudeDistance = 0,
    Position = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopHeighttoshadowDirectionalUse {
    /// Azimuth/Altitude
    AzimuthAltitude = 0,
    DirectionVector = 1,
}

#[derive(Debug, Clone)]
pub struct CopHeighttoshadow {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopHeighttoshadow {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_radius(mut self, val: f32) -> Self {
        self.params.insert("radius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert("radius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_angularsize(mut self, val: f32) -> Self {
        self.params.insert("angularsize".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_angularsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("angularsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_heightmapscale(mut self, val: f32) -> Self {
        self.params.insert("heightmapscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_heightmapscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("heightmapscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_azimuth(mut self, val: f32) -> Self {
        self.params.insert("azimuth".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_azimuth_expr(mut self, expr: &str) -> Self {
        self.params.insert("azimuth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_altitude(mut self, val: f32) -> Self {
        self.params.insert("altitude".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_altitude_expr(mut self, expr: &str) -> Self {
        self.params.insert("altitude".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_distance(mut self, val: f32) -> Self {
        self.params.insert("distance".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_distance_expr(mut self, expr: &str) -> Self {
        self.params.insert("distance".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_directional_azimuth(mut self, val: f32) -> Self {
        self.params.insert("directional_azimuth".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_directional_azimuth_expr(mut self, expr: &str) -> Self {
        self.params.insert("directional_azimuth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_directional_altitude(mut self, val: f32) -> Self {
        self.params.insert("directional_altitude".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_directional_altitude_expr(mut self, expr: &str) -> Self {
        self.params.insert("directional_altitude".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_viewradius(mut self, val: f32) -> Self {
        self.params.insert("viewradius".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_viewradius_expr(mut self, expr: &str) -> Self {
        self.params.insert("viewradius".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stepscale(mut self, val: f32) -> Self {
        self.params.insert("stepscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_stepscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("stepscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_position(mut self, val: [f32; 3]) -> Self {
        self.params.insert("position".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_position_expr(mut self, expr: &str) -> Self {
        self.params.insert("position".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lightnormal(mut self, val: [f32; 3]) -> Self {
        self.params.insert("lightnormal".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_lightnormal_expr(mut self, expr: &str) -> Self {
        self.params.insert("lightnormal".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_directional_direction(mut self, val: [f32; 3]) -> Self {
        self.params.insert("directional_direction".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_directional_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert("directional_direction".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: CopHeighttoshadowType) -> Self {
        self.params.insert("type".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert("type".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use(mut self, val: CopHeighttoshadowUse) -> Self {
        self.params.insert("use".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_use_expr(mut self, expr: &str) -> Self {
        self.params.insert("use".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_directional_use(mut self, val: CopHeighttoshadowDirectionalUse) -> Self {
        self.params.insert("directional_use".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_directional_use_expr(mut self, expr: &str) -> Self {
        self.params.insert("directional_use".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_singlesided(mut self, val: bool) -> Self {
        self.params.insert("singlesided".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_singlesided_expr(mut self, expr: &str) -> Self {
        self.params.insert("singlesided".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for CopHeighttoshadow {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "heighttoshadow"
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
pub struct CopHextile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopHextile {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: ""
    pub fn set_input_input_5<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "" and specifies the output index of the target node.
    pub fn set_input_input_5_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: ""
    pub fn set_input_input_6<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "" and specifies the output index of the target node.
    pub fn set_input_input_6_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: ""
    pub fn set_input_input_7<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "" and specifies the output index of the target node.
    pub fn set_input_input_7_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: ""
    pub fn set_input_input_8<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "" and specifies the output index of the target node.
    pub fn set_input_input_8_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: ""
    pub fn set_input_input_9<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "" and specifies the output index of the target node.
    pub fn set_input_input_9_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(9, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scalerand(mut self, val: f32) -> Self {
        self.params.insert("scalerand".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scalerand_expr(mut self, expr: &str) -> Self {
        self.params.insert("scalerand".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rot(mut self, val: f32) -> Self {
        self.params.insert("rot".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rot_expr(mut self, expr: &str) -> Self {
        self.params.insert("rot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rotrand(mut self, val: f32) -> Self {
        self.params.insert("rotrand".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rotrand_expr(mut self, expr: &str) -> Self {
        self.params.insert("rotrand".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_contrast(mut self, val: f32) -> Self {
        self.params.insert("contrast".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_contrast_expr(mut self, expr: &str) -> Self {
        self.params.insert("contrast".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_contrast_falloff(mut self, val: f32) -> Self {
        self.params.insert("contrast_falloff".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_contrast_falloff_expr(mut self, expr: &str) -> Self {
        self.params.insert("contrast_falloff".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_weightexp(mut self, val: f32) -> Self {
        self.params.insert("weightexp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_weightexp_expr(mut self, expr: &str) -> Self {
        self.params.insert("weightexp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_size(mut self, val: [f32; 2]) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for CopHextile {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hextile"
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
pub enum CopHistogramMode {
    ColoredBars = 0,
    SeparateBars = 1,
    Graph = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopHistogramOutside {
    Discard = 0,
    Clamp = 1,
}

#[derive(Debug, Clone)]
pub struct CopHistogram {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopHistogram {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_buckets(mut self, val: f32) -> Self {
        self.params.insert("buckets".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_buckets_expr(mut self, expr: &str) -> Self {
        self.params.insert("buckets".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_min(mut self, val: f32) -> Self {
        self.params.insert("min".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_min_expr(mut self, expr: &str) -> Self {
        self.params.insert("min".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_max(mut self, val: f32) -> Self {
        self.params.insert("max".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_max_expr(mut self, expr: &str) -> Self {
        self.params.insert("max".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_res(mut self, val: [f32; 2]) -> Self {
        self.params.insert("res".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_res_expr(mut self, expr: &str) -> Self {
        self.params.insert("res".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_channelscope(mut self, val: i32) -> Self {
        self.params.insert("channelscope".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_channelscope_expr(mut self, expr: &str) -> Self {
        self.params.insert("channelscope".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_mode(mut self, val: CopHistogramMode) -> Self {
        self.params.insert("mode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert("mode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outside(mut self, val: CopHistogramOutside) -> Self {
        self.params.insert("outside".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_outside_expr(mut self, expr: &str) -> Self {
        self.params.insert("outside".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert("signature".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert("signature".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_setres(mut self, val: bool) -> Self {
        self.params.insert("setres".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_setres_expr(mut self, expr: &str) -> Self {
        self.params.insert("setres".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for CopHistogram {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "histogram"
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
pub enum CopHsvOp {
    /// Hue/Sat/Val Adjust
    HueSatValAdjust = 0,
    ConvertRgbToHsv = 1,
    ConvertHsvToRgb = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CopHsvHueshiftmode {
    /// Min/Max
    MinMax = 0,
    Width = 1,
}

#[derive(Debug, Clone)]
pub struct CopHsv {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopHsv {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: ""
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: ""
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: ""
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: ""
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(4, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_mask(mut self, val: f32) -> Self {
        self.params.insert("mask".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.params.insert("mask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hueshift(mut self, val: f32) -> Self {
        self.params.insert("hueshift".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_hueshift_expr(mut self, expr: &str) -> Self {
        self.params.insert("hueshift".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hueshiftmin(mut self, val: f32) -> Self {
        self.params.insert("hueshiftmin".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_hueshiftmin_expr(mut self, expr: &str) -> Self {
        self.params.insert("hueshiftmin".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hueshiftmax(mut self, val: f32) -> Self {
        self.params.insert("hueshiftmax".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_hueshiftmax_expr(mut self, expr: &str) -> Self {
        self.params.insert("hueshiftmax".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hueshiftwidth(mut self, val: f32) -> Self {
        self.params.insert("hueshiftwidth".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_hueshiftwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert("hueshiftwidth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_satscale(mut self, val: f32) -> Self {
        self.params.insert("satscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_satscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("satscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_satshift(mut self, val: f32) -> Self {
        self.params.insert("satshift".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_satshift_expr(mut self, expr: &str) -> Self {
        self.params.insert("satshift".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_valscale(mut self, val: f32) -> Self {
        self.params.insert("valscale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_valscale_expr(mut self, expr: &str) -> Self {
        self.params.insert("valscale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_valshift(mut self, val: f32) -> Self {
        self.params.insert("valshift".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_valshift_expr(mut self, expr: &str) -> Self {
        self.params.insert("valshift".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_op(mut self, val: CopHsvOp) -> Self {
        self.params.insert("op".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_op_expr(mut self, expr: &str) -> Self {
        self.params.insert("op".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_hueshiftmode(mut self, val: CopHsvHueshiftmode) -> Self {
        self.params.insert("hueshiftmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_hueshiftmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("hueshiftmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_signature(mut self, val: &str) -> Self {
        self.params.insert("signature".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_signature_expr(mut self, expr: &str) -> Self {
        self.params.insert("signature".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_keepluma(mut self, val: bool) -> Self {
        self.params.insert("keepluma".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keepluma_expr(mut self, expr: &str) -> Self {
        self.params.insert("keepluma".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ispremult(mut self, val: bool) -> Self {
        self.params.insert("ispremult".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_ispremult_expr(mut self, expr: &str) -> Self {
        self.params.insert("ispremult".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for CopHsv {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hsv"
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
pub enum CopHyperbolictileMappingtype {
    Conformal = 0,
    EllipticGrid = 1,
    Squircle = 2,
    /// Equal-area
    EqualMinusArea = 3,
}

#[derive(Debug, Clone)]
pub struct CopHyperbolictile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl CopHyperbolictile {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: ""
    pub fn set_input_input_0<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "" and specifies the output index of the target node.
    pub fn set_input_input_0_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_flattening(mut self, val: f32) -> Self {
        self.params.insert("flattening".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_flattening_expr(mut self, expr: &str) -> Self {
        self.params.insert("flattening".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_size(mut self, val: f32) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert("size".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tilefitting(mut self, val: f32) -> Self {
        self.params.insert("tilefitting".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tilefitting_expr(mut self, expr: &str) -> Self {
        self.params.insert("tilefitting".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_polygon(mut self, val: i32) -> Self {
        self.params.insert("polygon".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_polygon_expr(mut self, expr: &str) -> Self {
        self.params.insert("polygon".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_mappingtype(mut self, val: CopHyperbolictileMappingtype) -> Self {
        self.params.insert("mappingtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_mappingtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("mappingtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_rectanglize(mut self, val: bool) -> Self {
        self.params.insert("rectanglize".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_rectanglize_expr(mut self, expr: &str) -> Self {
        self.params.insert("rectanglize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_stretchtofit(mut self, val: bool) -> Self {
        self.params.insert("stretchtofit".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_stretchtofit_expr(mut self, expr: &str) -> Self {
        self.params.insert("stretchtofit".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_diskmask(mut self, val: bool) -> Self {
        self.params.insert("diskmask".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_diskmask_expr(mut self, expr: &str) -> Self {
        self.params.insert("diskmask".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for CopHyperbolictile {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hyperbolictile"
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
