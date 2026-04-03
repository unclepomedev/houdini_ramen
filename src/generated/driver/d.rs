#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverDembonesSkinningconverterTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone)]
pub struct DriverDembonesSkinningconverter {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverDembonesSkinningconverter {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }


    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert("execute".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert("renderdialog".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_tolerance(mut self, val: f32) -> Self {
        self.params.insert("tolerance".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_tolerance_expr(mut self, expr: &str) -> Self {
        self.params.insert("tolerance".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_transaffine(mut self, val: f32) -> Self {
        self.params.insert("transAffine".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_transaffine_expr(mut self, expr: &str) -> Self {
        self.params.insert("transAffine".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_transaffinenorm(mut self, val: f32) -> Self {
        self.params.insert("transAffineNorm".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_transaffinenorm_expr(mut self, expr: &str) -> Self {
        self.params.insert("transAffineNorm".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_weightssmooth(mut self, val: f32) -> Self {
        self.params.insert("weightsSmooth".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_weightssmooth_expr(mut self, expr: &str) -> Self {
        self.params.insert("weightsSmooth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_weightssmoothstep(mut self, val: f32) -> Self {
        self.params.insert("weightsSmoothStep".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_weightssmoothstep_expr(mut self, expr: &str) -> Self {
        self.params.insert("weightsSmoothStep".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_nbones(mut self, val: i32) -> Self {
        self.params.insert("nBones".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_nbones_expr(mut self, expr: &str) -> Self {
        self.params.insert("nBones".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_niters(mut self, val: i32) -> Self {
        self.params.insert("nIters".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_niters_expr(mut self, expr: &str) -> Self {
        self.params.insert("nIters".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_patience(mut self, val: i32) -> Self {
        self.params.insert("patience".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_patience_expr(mut self, expr: &str) -> Self {
        self.params.insert("patience".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ninititers(mut self, val: i32) -> Self {
        self.params.insert("nInitIters".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_ninititers_expr(mut self, expr: &str) -> Self {
        self.params.insert("nInitIters".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ntransiters(mut self, val: i32) -> Self {
        self.params.insert("nTransIters".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_ntransiters_expr(mut self, expr: &str) -> Self {
        self.params.insert("nTransIters".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_nweightsiters(mut self, val: i32) -> Self {
        self.params.insert("nWeightsIters".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_nweightsiters_expr(mut self, expr: &str) -> Self {
        self.params.insert("nWeightsIters".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_nnz(mut self, val: i32) -> Self {
        self.params.insert("nnz".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_nnz_expr(mut self, expr: &str) -> Self {
        self.params.insert("nnz".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int2 parameters ---
    pub fn with_f(mut self, val: [i32; 2]) -> Self {
        self.params.insert("f".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert("f".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverDembonesSkinningconverterTrange) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soutputfile(mut self, val: &str) -> Self {
        self.params.insert("sOutputFile".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_soutputfile_expr(mut self, expr: &str) -> Self {
        self.params.insert("sOutputFile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sanimatedcache(mut self, val: &str) -> Self {
        self.params.insert("sAnimatedCache".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sanimatedcache_expr(mut self, expr: &str) -> Self {
        self.params.insert("sAnimatedCache".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sbindposefbx(mut self, val: &str) -> Self {
        self.params.insert("sBindPoseFBX".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sbindposefbx_expr(mut self, expr: &str) -> Self {
        self.params.insert("sBindPoseFBX".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.params.insert("mkpath".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("mkpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_createroot(mut self, val: bool) -> Self {
        self.params.insert("createroot".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_createroot_expr(mut self, expr: &str) -> Self {
        self.params.insert("createroot".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DriverDembonesSkinningconverter {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "dembones_skinningconverter"
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
pub enum DriverDopTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverDopCompresssims {
    NoCompression = 0,
    Blosc = 1,
    Gzip = 2,
}

#[derive(Debug, Clone)]
pub struct DriverDop {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverDop {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }


    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert("execute".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_executebackground(mut self) -> Self {
        self.params.insert("executebackground".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert("renderdialog".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float3 parameters ---
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert("f".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert("f".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverDopTrange) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_compresssims(mut self, val: DriverDopCompresssims) -> Self {
        self.params.insert("compresssims".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_compresssims_expr(mut self, expr: &str) -> Self {
        self.params.insert("compresssims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_doppath(mut self, val: &str) -> Self {
        self.params.insert("doppath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_doppath_expr(mut self, expr: &str) -> Self {
        self.params.insert("doppath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dopoutput(mut self, val: &str) -> Self {
        self.params.insert("dopoutput".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_dopoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert("dopoutput".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert("prerender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert("prerender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert("lprerender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert("lprerender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert("preframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("preframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert("lpreframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("lpreframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert("postframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("postframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert("lpostframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("lpostframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert("postrender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert("postrender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert("lpostrender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert("lpostrender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_usesimframes(mut self, val: bool) -> Self {
        self.params.insert("usesimframes".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usesimframes_expr(mut self, expr: &str) -> Self {
        self.params.insert("usesimframes".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.params.insert("mkpath".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("mkpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_initsim(mut self, val: bool) -> Self {
        self.params.insert("initsim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_initsim_expr(mut self, expr: &str) -> Self {
        self.params.insert("initsim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_alfprogress(mut self, val: bool) -> Self {
        self.params.insert("alfprogress".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_alfprogress_expr(mut self, expr: &str) -> Self {
        self.params.insert("alfprogress".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tprerender(mut self, val: bool) -> Self {
        self.params.insert("tprerender".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert("tprerender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tpreframe(mut self, val: bool) -> Self {
        self.params.insert("tpreframe".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("tpreframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tpostframe(mut self, val: bool) -> Self {
        self.params.insert("tpostframe".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("tpostframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tpostrender(mut self, val: bool) -> Self {
        self.params.insert("tpostrender".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert("tpostrender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DriverDop {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "dop"
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
pub enum DriverDopnetCompresssims {
    NoCompression = 0,
    Blosc = 1,
    Gzip = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverDopnetXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverDopnetRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverDopnetPreXform {
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
pub enum DriverDopnetUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct DriverDopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DriverDopnet {
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



    // --- Button parameters ---
    pub fn trigger_resimulate(mut self) -> Self {
        self.params.insert("resimulate".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_timestep(mut self, val: f32) -> Self {
        self.params.insert("timestep".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_timestep_expr(mut self, expr: &str) -> Self {
        self.params.insert("timestep".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_timescale(mut self, val: f32) -> Self {
        self.params.insert("timescale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert("timescale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert("r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_substep(mut self, val: i32) -> Self {
        self.params.insert("substep".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_substep_expr(mut self, expr: &str) -> Self {
        self.params.insert("substep".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_startframe(mut self, val: i32) -> Self {
        self.params.insert("startframe".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_startframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("startframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxfeedback(mut self, val: i32) -> Self {
        self.params.insert("maxfeedback".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_maxfeedback_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxfeedback".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cachemaxsize(mut self, val: i32) -> Self {
        self.params.insert("cachemaxsize".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_cachemaxsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("cachemaxsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_continuouscook_tick(mut self, val: i32) -> Self {
        self.params.insert("continuouscook_tick".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_continuouscook_tick_expr(mut self, expr: &str) -> Self {
        self.params.insert("continuouscook_tick".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_explicitcachensteps(mut self, val: i32) -> Self {
        self.params.insert("explicitcachensteps".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_explicitcachensteps_expr(mut self, expr: &str) -> Self {
        self.params.insert("explicitcachensteps".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_explicitcachecheckpointspacing(mut self, val: i32) -> Self {
        self.params.insert("explicitcachecheckpointspacing".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_explicitcachecheckpointspacing_expr(mut self, expr: &str) -> Self {
        self.params.insert("explicitcachecheckpointspacing".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_compresssims(mut self, val: DriverDopnetCompresssims) -> Self {
        self.params.insert("compresssims".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_compresssims_expr(mut self, expr: &str) -> Self {
        self.params.insert("compresssims".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: DriverDopnetXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: DriverDopnetRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pre_xform(mut self, val: DriverDopnetPreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: DriverDopnetUparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_playfilesname(mut self, val: &str) -> Self {
        self.params.insert("playfilesname".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_playfilesname_expr(mut self, expr: &str) -> Self {
        self.params.insert("playfilesname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_initialstate(mut self, val: &str) -> Self {
        self.params.insert("initialstate".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_initialstate_expr(mut self, expr: &str) -> Self {
        self.params.insert("initialstate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_explicitcachename(mut self, val: &str) -> Self {
        self.params.insert("explicitcachename".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_explicitcachename_expr(mut self, expr: &str) -> Self {
        self.params.insert("explicitcachename".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displayfilter(mut self, val: &str) -> Self {
        self.params.insert("displayfilter".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_displayfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert("displayfilter".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_isplayer(mut self, val: bool) -> Self {
        self.params.insert("isplayer".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_isplayer_expr(mut self, expr: &str) -> Self {
        self.params.insert("isplayer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_autoresim(mut self, val: bool) -> Self {
        self.params.insert("autoresim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_autoresim_expr(mut self, expr: &str) -> Self {
        self.params.insert("autoresim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_datahints(mut self, val: bool) -> Self {
        self.params.insert("datahints".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_datahints_expr(mut self, expr: &str) -> Self {
        self.params.insert("datahints".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_interpolate(mut self, val: bool) -> Self {
        self.params.insert("interpolate".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_interpolate_expr(mut self, expr: &str) -> Self {
        self.params.insert("interpolate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cacheenabled(mut self, val: bool) -> Self {
        self.params.insert("cacheenabled".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_cacheenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert("cacheenabled".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cachetodisk(mut self, val: bool) -> Self {
        self.params.insert("cachetodisk".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_cachetodisk_expr(mut self, expr: &str) -> Self {
        self.params.insert("cachetodisk".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cachetodisknoninteractive(mut self, val: bool) -> Self {
        self.params.insert("cachetodisknoninteractive".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_cachetodisknoninteractive_expr(mut self, expr: &str) -> Self {
        self.params.insert("cachetodisknoninteractive".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cachesubsteps(mut self, val: bool) -> Self {
        self.params.insert("cachesubsteps".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_cachesubsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert("cachesubsteps".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_timeless(mut self, val: bool) -> Self {
        self.params.insert("timeless".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_timeless_expr(mut self, expr: &str) -> Self {
        self.params.insert("timeless".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_continuouscook(mut self, val: bool) -> Self {
        self.params.insert("continuouscook".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_continuouscook_expr(mut self, expr: &str) -> Self {
        self.params.insert("continuouscook".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_explicitcache(mut self, val: bool) -> Self {
        self.params.insert("explicitcache".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_explicitcache_expr(mut self, expr: &str) -> Self {
        self.params.insert("explicitcache".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DriverDopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "dopnet"
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
pub enum DriverDsmmergeTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriverDsmmergeDsmCompositing {
    SameAsInputs = 0,
    ForceUncomposited = 1,
    ForceComposited = 2,
}

#[derive(Debug, Clone)]
pub struct DriverDsmmerge {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DriverDsmmerge {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }


    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert("execute".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert("renderdialog".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_renderpreview(mut self) -> Self {
        self.params.insert("renderpreview".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_executebackground(mut self) -> Self {
        self.params.insert("executebackground".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_dsm_olimit(mut self, val: f32) -> Self {
        self.params.insert("dsm_olimit".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_dsm_olimit_expr(mut self, expr: &str) -> Self {
        self.params.insert("dsm_olimit".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert("f".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert("f".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_soho_outputmode(mut self, val: i32) -> Self {
        self.params.insert("soho_outputmode".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_soho_outputmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_outputmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: DriverDsmmergeTrange) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dsm_compositing(mut self, val: DriverDsmmergeDsmCompositing) -> Self {
        self.params.insert("dsm_compositing".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_dsm_compositing_expr(mut self, expr: &str) -> Self {
        self.params.insert("dsm_compositing".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_soho_program(mut self, val: &str) -> Self {
        self.params.insert("soho_program".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_soho_program_expr(mut self, expr: &str) -> Self {
        self.params.insert("soho_program".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dsm_output(mut self, val: &str) -> Self {
        self.params.insert("dsm_output".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_dsm_output_expr(mut self, expr: &str) -> Self {
        self.params.insert("dsm_output".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dsm_source_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("dsm_source{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_dsm_source_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("dsm_source{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert("prerender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert("prerender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert("lprerender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert("lprerender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert("preframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("preframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert("lpreframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("lpreframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert("postframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("postframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert("lpostframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("lpostframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert("postrender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert("postrender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert("lpostrender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert("lpostrender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_dsm_bypass_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("dsm_bypass{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_dsm_bypass_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("dsm_bypass{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tprerender(mut self, val: bool) -> Self {
        self.params.insert("tprerender".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert("tprerender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tpreframe(mut self, val: bool) -> Self {
        self.params.insert("tpreframe".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("tpreframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tpostframe(mut self, val: bool) -> Self {
        self.params.insert("tpostframe".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("tpostframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tpostrender(mut self, val: bool) -> Self {
        self.params.insert("tpostrender".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert("tpostrender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DriverDsmmerge {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "dsmmerge"
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
