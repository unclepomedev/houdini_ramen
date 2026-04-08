#[derive(Debug, Clone)]
pub struct VopWatersurf {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopWatersurf {
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
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_surface_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }
    pub fn set_surface_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(1, (out.node_id, out.pin));
        self
    }
    pub fn set_shallow_water_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(2, (out.node_id, out.pin));
        self
    }
    pub fn set_mid_depth_water_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(3, (out.node_id, out.pin));
        self
    }
    pub fn set_deep_water_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(4, (out.node_id, out.pin));
        self
    }
    pub fn set_dirty_water_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(5, (out.node_id, out.pin));
        self
    }
    pub fn set_weed_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(6, (out.node_id, out.pin));
        self
    }
    pub fn set_specular_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(7, (out.node_id, out.pin));
        self
    }
    pub fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(8, (out.node_id, out.pin));
        self
    }
    pub fn set_water_clarity_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(9, (out.node_id, out.pin));
        self
    }
    pub fn set_water_depth_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(10, (out.node_id, out.pin));
        self
    }
    pub fn set_reflection_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(11, (out.node_id, out.pin));
        self
    }
    pub fn set_ocean_floor_brightness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(12, (out.node_id, out.pin));
        self
    }
    pub fn set_depth_noise_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(13, (out.node_id, out.pin));
        self
    }
    pub fn set_weed_noise_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(14, (out.node_id, out.pin));
        self
    }
    pub fn set_weed_amount_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(15, (out.node_id, out.pin));
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
    pub fn with_waterclarity(mut self, val: f32) -> Self {
        self.params.insert(
            "waterclarity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_waterclarity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "waterclarity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_waterdepth(mut self, val: f32) -> Self {
        self.params.insert(
            "waterdepth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_waterdepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "waterdepth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reflscale(mut self, val: f32) -> Self {
        self.params.insert(
            "reflscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reflscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_depthbright(mut self, val: f32) -> Self {
        self.params.insert(
            "depthbright".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depthbright_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthbright".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_weedamount(mut self, val: f32) -> Self {
        self.params.insert(
            "weedamount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weedamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weedamount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shallowcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "shallowcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_shallowcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shallowcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_midcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "midcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_midcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "midcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_deepcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "deepcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_deepcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deepcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dirtycolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dirtycolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dirtycolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dirtycolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_weedcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "weedcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_weedcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weedcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_speccolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "speccolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_speccolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speccolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_depthfreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "depthfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_depthfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_weedfreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "weedfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_weedfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weedfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopWatersurf {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "watersurf"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait VopWatersurfOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Color"
    fn out_clr(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("clr".to_string()),
        }
    }
    /// Output pin: "Reflectivity Amount"
    fn out_reflect(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("reflect".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopWatersurfOutputs for VopWatersurf {}
impl VopWatersurfOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopWatersurf> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopWatersurfInnerExt {
    fn i_dot_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn i_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add_base_reflectivuty(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn adjusted_water_color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn blend_region_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn blend_weeds(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn bottom_color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clean_and_dirty_water(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clean_water(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn dist_to_sea_bottom(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn effective_depth(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn factor_in_floor_brightness(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn final_weed_amount(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn frontface1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn lighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn negate1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn normalize_i(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn normalize_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn normalize_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn plateau_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn raw_reflectivity(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn reflectivity_amount(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn transmit_amount(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn weed_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopWatersurfInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopWatersurf> {
    fn i_dot_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("I_dot_N")
    }
    fn i_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("I_global")
    }
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_global")
    }
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_input")
    }
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_global")
    }
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_input")
    }
    fn add_base_reflectivuty(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add_base_reflectivuty")
    }
    fn adjusted_water_color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("adjusted_water_color")
    }
    fn blend_region_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("blend_region_noise")
    }
    fn blend_weeds(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("blend_weeds")
    }
    fn bottom_color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("bottom_color")
    }
    fn clean_and_dirty_water(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clean_and_dirty_water")
    }
    fn clean_water(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clean_water")
    }
    fn dist_to_sea_bottom(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("dist_to_sea_bottom")
    }
    fn effective_depth(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("effective_depth")
    }
    fn factor_in_floor_brightness(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("factor_in_floor_brightness")
    }
    fn final_weed_amount(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("final_weed_amount")
    }
    fn frontface1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("frontface1")
    }
    fn lighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("lighting1")
    }
    fn negate1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("negate1")
    }
    fn normalize_i(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("normalize_I")
    }
    fn normalize_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("normalize_N")
    }
    fn normalize_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("normalize_noise")
    }
    fn plateau_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("plateau_noise")
    }
    fn raw_reflectivity(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("raw_reflectivity")
    }
    fn reflectivity_amount(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("reflectivity_amount")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn transmit_amount(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("transmit_amount")
    }
    fn weed_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("weed_noise")
    }
}

#[derive(Debug, Clone)]
pub struct VopWaves {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopWaves {
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
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_surface_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }
    pub fn set_surface_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(1, (out.node_id, out.pin));
        self
    }
    pub fn set_wave_up_vector_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(2, (out.node_id, out.pin));
        self
    }
    pub fn set_wave_direction_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(3, (out.node_id, out.pin));
        self
    }
    pub fn set_wave_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(4, (out.node_id, out.pin));
        self
    }
    pub fn set_wave_speed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(5, (out.node_id, out.pin));
        self
    }
    pub fn set_wave_randomness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(6, (out.node_id, out.pin));
        self
    }
    pub fn set_chop_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(7, (out.node_id, out.pin));
        self
    }
    pub fn set_chop_speed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(8, (out.node_id, out.pin));
        self
    }
    pub fn set_chop_amount_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(9, (out.node_id, out.pin));
        self
    }
    pub fn set_displacement_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(10, (out.node_id, out.pin));
        self
    }
    pub fn set_time_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(11, (out.node_id, out.pin));
        self
    }

    pub fn with_wfreq(mut self, val: f32) -> Self {
        self.params.insert(
            "wfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wspeed(mut self, val: f32) -> Self {
        self.params.insert(
            "wspeed".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wspeed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wrandom(mut self, val: f32) -> Self {
        self.params.insert(
            "wrandom".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wrandom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wrandom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_camount(mut self, val: f32) -> Self {
        self.params.insert(
            "camount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_camount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_time(mut self, val: f32) -> Self {
        self.params.insert(
            "time".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_time_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "time".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wup(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "wup".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_wup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wdir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "wdir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_wdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wdir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cfreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cspeed(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "cspeed".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_cspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cspeed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopWaves {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "waves"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait VopWavesOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Displaced Position"
    fn out_dispp(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("dispP".to_string()),
        }
    }
    /// Output pin: "Displaced Normal"
    fn out_dispn(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("dispN".to_string()),
        }
    }
    /// Output pin: "Displacement Amount"
    fn out_amount(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("amount".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopWavesOutputs for VopWaves {}
impl VopWavesOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopWaves> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopWavesInnerExt {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add_all_waves(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn cellular_chopiness(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn chop_noise2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn chop_noise3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn chop_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn chop_offset_further(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn chop_position(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn compute_chopiness(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn compute_primary_wave(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn compute_swells(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn convert_to_vector(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn displacenml1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn final_chop_freq(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn get_wave_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mulconst3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn normalize_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn offset_primary_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn primary_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn primary_time_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn swell_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn swell_noise_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn swell_up_the_waves(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn veccast1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn wave_frequency(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn xspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn yspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn zspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopWavesInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopWaves> {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_global")
    }
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_input")
    }
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_global")
    }
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_input")
    }
    fn add_all_waves(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add_all_waves")
    }
    fn cellular_chopiness(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("cellular_chopiness")
    }
    fn chop_noise2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("chop_noise2")
    }
    fn chop_noise3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("chop_noise3")
    }
    fn chop_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("chop_offset")
    }
    fn chop_offset_further(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("chop_offset_further")
    }
    fn chop_position(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("chop_position")
    }
    fn compute_chopiness(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("compute_chopiness")
    }
    fn compute_primary_wave(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("compute_primary_wave")
    }
    fn compute_swells(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("compute_swells")
    }
    fn convert_to_vector(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("convert_to_vector")
    }
    fn displacenml1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("displacenml1")
    }
    fn final_chop_freq(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("final_chop_freq")
    }
    fn get_wave_space(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("get_wave_space")
    }
    fn mulconst3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("mulconst3")
    }
    fn multiply7(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply7")
    }
    fn normalize_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("normalize_N")
    }
    fn offset_primary_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("offset_primary_noise")
    }
    fn primary_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("primary_noise")
    }
    fn primary_time_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("primary_time_offset")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn swell_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("swell_noise")
    }
    fn swell_noise_offset(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("swell_noise_offset")
    }
    fn swell_up_the_waves(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("swell_up_the_waves")
    }
    fn veccast1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("veccast1")
    }
    fn wave_frequency(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("wave_frequency")
    }
    fn xspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("xspace")
    }
    fn yspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("yspace")
    }
    fn zspace(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("zspace")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopWavevectorPlane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone)]
pub struct VopWavevector {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopWavevector {
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
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_ix_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }
    pub fn set_iy_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(1, (out.node_id, out.pin));
        self
    }
    pub fn set_iz_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(2, (out.node_id, out.pin));
        self
    }
    pub fn set_resx_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(3, (out.node_id, out.pin));
        self
    }
    pub fn set_resy_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(4, (out.node_id, out.pin));
        self
    }
    pub fn set_resz_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(5, (out.node_id, out.pin));
        self
    }
    pub fn set_gridsize_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(6, (out.node_id, out.pin));
        self
    }
    pub fn set_n_2d_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(7, (out.node_id, out.pin));
        self
    }
    pub fn set_plane_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(8, (out.node_id, out.pin));
        self
    }

    pub fn with_ix(mut self, val: i32) -> Self {
        self.params.insert(
            "ix".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ix".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iy(mut self, val: i32) -> Self {
        self.params.insert(
            "iy".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iz(mut self, val: i32) -> Self {
        self.params.insert(
            "iz".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resx(mut self, val: i32) -> Self {
        self.params.insert(
            "resx".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resy(mut self, val: i32) -> Self {
        self.params.insert(
            "resy".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resz(mut self, val: i32) -> Self {
        self.params.insert(
            "resz".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resz_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resz".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_plane(mut self, val: VopWavevectorPlane) -> Self {
        self.params.insert(
            "plane".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_plane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "plane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_twod(mut self, val: bool) -> Self {
        self.params.insert(
            "twod".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_twod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopWavevector {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "wavevector"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait VopWavevectorOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "wavevec"
    fn out_wavevec(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("wavevec".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopWavevectorOutputs for VopWavevector {}
impl VopWavevectorOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopWavevector> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopWavevectorInnerExt {
    fn clear_axis_if_twod(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clear_x(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clear_y(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clear_z(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn const1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn divide_by_gridsize(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn inttovec1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mul_two_pi(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn resx_div_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn resy_div_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn resz_div_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subtract1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subtract4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subtract5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn switch_plane(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopWavevectorInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopWavevector> {
    fn clear_axis_if_twod(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clear_axis_if_twod")
    }
    fn clear_x(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clear_x")
    }
    fn clear_y(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clear_y")
    }
    fn clear_z(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clear_z_")
    }
    fn const1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("const1")
    }
    fn divide_by_gridsize(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("divide_by_gridsize")
    }
    fn inttovec1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("inttovec1")
    }
    fn mul_two_pi(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("mul_two_pi")
    }
    fn null1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("null1")
    }
    fn resx_div_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("resx_div_2")
    }
    fn resy_div_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("resy_div_2")
    }
    fn resz_div_2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("resz_div_2")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn subtract1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subtract1")
    }
    fn subtract4(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subtract4")
    }
    fn subtract5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subtract5")
    }
    fn switch_plane(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("switch_plane")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopWhileCondition {
    /// True ( != 0 )
    True0 = 0,
    /// False ( == 0 )
    False0 = 1,
}

#[derive(Debug, Clone)]
pub struct VopWhile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopWhile {
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
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_condition(mut self, val: VopWhileCondition) -> Self {
        self.params.insert(
            "condition".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_condition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condition".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopWhile {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "while"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait VopWhileOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 0"
    fn out_output0(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output0".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopWhileOutputs for VopWhile {}
impl VopWhileOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopWhile> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopWhileInnerExt {
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopWhileInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopWhile> {
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
}

#[derive(Debug, Clone)]
pub struct VopWirePattern {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopWirePattern {
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
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_uv_coordinates_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }
    pub fn set_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(1, (out.node_id, out.pin));
        self
    }
    pub fn set_wire_width_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(2, (out.node_id, out.pin));
        self
    }

    pub fn with_width(mut self, val: f32) -> Self {
        self.params.insert(
            "width".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "width".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_freq(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopWirePattern {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "wire_pattern"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait VopWirePatternOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Combined Value"
    fn out_amount(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("amount".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopWirePatternOutputs for VopWirePattern {}
impl VopWirePatternOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopWirePattern> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopWirePatternInnerExt {
    fn ifconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn max1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pulsetrain1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn pulsetrain2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn uvcoords1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn vec2tovec1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn vectofloat1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopWirePatternInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopWirePattern> {
    fn ifconnected1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("ifconnected1")
    }
    fn max1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("max1")
    }
    fn multiply3(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply3")
    }
    fn pulsetrain1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pulsetrain1")
    }
    fn pulsetrain2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("pulsetrain2")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn uvcoords1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("uvcoords1")
    }
    fn vec2tovec1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("vec2tovec1")
    }
    fn vectofloat1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("vectofloat1")
    }
}

#[derive(Debug, Clone)]
pub struct VopWireframe {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopWireframe {
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
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_surface_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }
    pub fn set_s_coordinate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(1, (out.node_id, out.pin));
        self
    }
    pub fn set_t_coordinate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(2, (out.node_id, out.pin));
        self
    }
    pub fn set_s_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(3, (out.node_id, out.pin));
        self
    }
    pub fn set_t_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(4, (out.node_id, out.pin));
        self
    }
    pub fn set_wire_width_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(5, (out.node_id, out.pin));
        self
    }
    pub fn set_wire_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(6, (out.node_id, out.pin));
        self
    }

    pub fn with_sfreq(mut self, val: f32) -> Self {
        self.params.insert(
            "sfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tfreq(mut self, val: f32) -> Self {
        self.params.insert(
            "tfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "wwidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wwidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "wcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_wcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopWireframe {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "wireframe"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait VopWireframeOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Surface Color"
    fn out_color(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("color".to_string()),
        }
    }
    /// Output pin: "Opacity"
    fn out_opacity(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("opacity".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopWireframeOutputs for VopWireframe {}
impl VopWireframeOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopWireframe> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopWireframeInnerExt {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn boxes1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn complement2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn floattovec1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn lighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn normalize_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn opacity(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn wire_color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopWireframeInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopWireframe> {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_global")
    }
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_input")
    }
    fn boxes1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("boxes1")
    }
    fn complement2(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("complement2")
    }
    fn floattovec1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("floattovec1")
    }
    fn lighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("lighting1")
    }
    fn normalize_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("normalize_N")
    }
    fn opacity(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("opacity")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn wire_color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("wire_color")
    }
}

#[derive(Debug, Clone)]
pub struct VopWood {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopWood {
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
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_surface_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }
    pub fn set_surface_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(1, (out.node_id, out.pin));
        self
    }
    pub fn set_s_coordinate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(2, (out.node_id, out.pin));
        self
    }
    pub fn set_t_coordinate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(3, (out.node_id, out.pin));
        self
    }
    pub fn set_base_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(4, (out.node_id, out.pin));
        self
    }
    pub fn set_vein_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(5, (out.node_id, out.pin));
        self
    }
    pub fn set_specular_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(6, (out.node_id, out.pin));
        self
    }
    pub fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(7, (out.node_id, out.pin));
        self
    }
    pub fn set_vein_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(8, (out.node_id, out.pin));
        self
    }
    pub fn set_vein_s_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(9, (out.node_id, out.pin));
        self
    }
    pub fn set_vein_t_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(10, (out.node_id, out.pin));
        self
    }
    pub fn set_vein_attenuation_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(11, (out.node_id, out.pin));
        self
    }
    pub fn set_noise_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(12, (out.node_id, out.pin));
        self
    }
    pub fn set_noise_offset_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(13, (out.node_id, out.pin));
        self
    }
    pub fn set_noise_amplitude_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(14, (out.node_id, out.pin));
        self
    }
    pub fn set_groove_depth_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(15, (out.node_id, out.pin));
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
    pub fn with_vscale(mut self, val: f32) -> Self {
        self.params.insert(
            "vscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vsfreq(mut self, val: f32) -> Self {
        self.params.insert(
            "vsfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vsfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vsfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vtfreq(mut self, val: f32) -> Self {
        self.params.insert(
            "vtfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vtfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vtfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vatten(mut self, val: f32) -> Self {
        self.params.insert(
            "vatten".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vatten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vatten".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_namp(mut self, val: f32) -> Self {
        self.params.insert(
            "namp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_namp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "namp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_gdepth(mut self, val: f32) -> Self {
        self.params.insert(
            "gdepth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gdepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gdepth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "basecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_basecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_veincolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "veincolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_veincolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "veincolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_speccolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "speccolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_speccolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speccolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nfreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "nfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_nfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_noffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "noffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_noffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "noffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopWood {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "wood"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait VopWoodOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Color"
    fn out_color(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("color".to_string()),
        }
    }
    /// Output pin: "Vein Amount"
    fn out_amount(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("amount".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopWoodOutputs for VopWood {}
impl VopWoodOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopWood> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopWoodInnerExt {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn bottom(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamp_amount_to_low_range(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn colormix1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn generate_noisy_s_t(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn generate_ripply_veins(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn groove_displacement(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn lighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mid(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn negate1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn normalize1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn normalize_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn s_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn s_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn top(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopWoodInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopWood> {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_global")
    }
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_input")
    }
    fn bottom(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("bottom")
    }
    fn clamp_amount_to_low_range(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamp_amount_to_low_range")
    }
    fn colormix1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("colormix1")
    }
    fn generate_noisy_s_t(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("generate_noisy_s_t")
    }
    fn generate_ripply_veins(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("generate_ripply_veins")
    }
    fn groove_displacement(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("groove_displacement")
    }
    fn lighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("lighting1")
    }
    fn mid(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("mid")
    }
    fn multiply1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply1")
    }
    fn negate1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("negate1")
    }
    fn normalize1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("normalize1")
    }
    fn normalize_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("normalize_N")
    }
    fn s_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("s_global")
    }
    fn s_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("s_input")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn top(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("top")
    }
}

#[derive(Debug, Clone)]
pub struct VopWoodplank {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopWoodplank {
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
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_surface_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }
    pub fn set_surface_normal_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(1, (out.node_id, out.pin));
        self
    }
    pub fn set_s_coordinate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(2, (out.node_id, out.pin));
        self
    }
    pub fn set_t_coordinate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(3, (out.node_id, out.pin));
        self
    }
    pub fn set_dark_wood_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(4, (out.node_id, out.pin));
        self
    }
    pub fn set_light_wood_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(5, (out.node_id, out.pin));
        self
    }
    pub fn set_overall_lightness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(6, (out.node_id, out.pin));
        self
    }
    pub fn set_groove_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(7, (out.node_id, out.pin));
        self
    }
    pub fn set_specular_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(8, (out.node_id, out.pin));
        self
    }
    pub fn set_reflection_color_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(9, (out.node_id, out.pin));
        self
    }
    pub fn set_color_variance_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(10, (out.node_id, out.pin));
        self
    }
    pub fn set_roughness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(11, (out.node_id, out.pin));
        self
    }
    pub fn set_waviness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(12, (out.node_id, out.pin));
        self
    }
    pub fn set_ringiness_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(13, (out.node_id, out.pin));
        self
    }
    pub fn set_graininess_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(14, (out.node_id, out.pin));
        self
    }
    pub fn set_overall_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(15, (out.node_id, out.pin));
        self
    }
    pub fn set_n_3d_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(16, (out.node_id, out.pin));
        self
    }
    pub fn set_bump_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(17, (out.node_id, out.pin));
        self
    }
    pub fn set_scuff_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(18, (out.node_id, out.pin));
        self
    }
    pub fn set_grain_scale_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(19, (out.node_id, out.pin));
        self
    }
    pub fn set_plank_width_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(20, (out.node_id, out.pin));
        self
    }
    pub fn set_grout_width_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(21, (out.node_id, out.pin));
        self
    }

    pub fn with_lightness(mut self, val: f32) -> Self {
        self.params.insert(
            "lightness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lightness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_variance(mut self, val: f32) -> Self {
        self.params.insert(
            "variance".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_variance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variance".to_string(),
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
    pub fn with_wavy(mut self, val: f32) -> Self {
        self.params.insert(
            "wavy".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wavy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wavy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ringy(mut self, val: f32) -> Self {
        self.params.insert(
            "ringy".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ringy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ringy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_grainy(mut self, val: f32) -> Self {
        self.params.insert(
            "grainy".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_grainy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grainy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scale3d(mut self, val: f32) -> Self {
        self.params.insert(
            "scale3d".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale3d_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale3d".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bumpscale(mut self, val: f32) -> Self {
        self.params.insert(
            "bumpscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bumpscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bumpscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scuffscale(mut self, val: f32) -> Self {
        self.params.insert(
            "scuffscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scuffscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scuffscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_grainscale(mut self, val: f32) -> Self {
        self.params.insert(
            "grainscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_grainscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grainscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_plankwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "plankwidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_plankwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "plankwidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_groutwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "groutwidth".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_groutwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "groutwidth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_darkcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "darkcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_darkcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "darkcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lightcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lightcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lightcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_groovecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "groovecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_groovecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "groovecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_speccolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "speccolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_speccolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speccolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reflcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "reflcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_reflcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reflcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopWoodplank {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "woodplank"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait VopWoodplankOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Color"
    fn out_color(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("color".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
    /// Output pin: "Output 4"
    fn out_output4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output4".to_string()),
        }
    }
    /// Output pin: "Output 5"
    fn out_output5(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output5".to_string()),
        }
    }
    /// Output pin: "Output 6"
    fn out_output6(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output6".to_string()),
        }
    }
    /// Output pin: "Output 7"
    fn out_output7(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output7".to_string()),
        }
    }
    /// Output pin: "Output 8"
    fn out_output8(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output8".to_string()),
        }
    }
    /// Output pin: "Output 9"
    fn out_output9(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output9".to_string()),
        }
    }
    /// Output pin: "Output 10"
    fn out_output10(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output10".to_string()),
        }
    }
    /// Output pin: "Output 11"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output11".to_string()),
        }
    }
    /// Output pin: "Output 12"
    fn out_output12(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output12".to_string()),
        }
    }
    /// Output pin: "Output 13"
    fn out_output13(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output13".to_string()),
        }
    }
    /// Output pin: "Output 14"
    fn out_output14(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output14".to_string()),
        }
    }
    /// Output pin: "Output 15"
    fn out_output15(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output15".to_string()),
        }
    }
    /// Output pin: "Output 16"
    fn out_output16(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output16".to_string()),
        }
    }
    /// Output pin: "Output 17"
    fn out_output17(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output17".to_string()),
        }
    }
    /// Output pin: "Output 18"
    fn out_output18(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output18".to_string()),
        }
    }
    /// Output pin: "Output 19"
    fn out_output19(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output19".to_string()),
        }
    }
    /// Output pin: "Output 20"
    fn out_output20(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output20".to_string()),
        }
    }
    /// Output pin: "Output 21"
    fn out_output21(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output21".to_string()),
        }
    }
    /// Output pin: "Output 22"
    fn out_output22(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output22".to_string()),
        }
    }
    /// Output pin: "Output 23"
    fn out_output23(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output23".to_string()),
        }
    }
    /// Output pin: "Output 24"
    fn out_output24(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output24".to_string()),
        }
    }
    /// Output pin: "Output 25"
    fn out_output25(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output25".to_string()),
        }
    }
    /// Output pin: "Output 26"
    fn out_output26(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output26".to_string()),
        }
    }
    /// Output pin: "Output 27"
    fn out_output27(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output27".to_string()),
        }
    }
    /// Output pin: "Output 28"
    fn out_output28(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output28".to_string()),
        }
    }
    /// Output pin: "Output 29"
    fn out_output29(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output29".to_string()),
        }
    }
    /// Output pin: "Output 30"
    fn out_output30(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output30".to_string()),
        }
    }
    /// Output pin: "Output 31"
    fn out_output31(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output31".to_string()),
        }
    }
}

impl VopWoodplankOutputs for VopWoodplank {}
impl VopWoodplankOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopWoodplank> {}

#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait VopWoodplankInnerExt {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_more_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn p_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add_lightness(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn add_stripe_distortion(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn base_plank_color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn border_width(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn bump_amount(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn clamp_grain_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn compute_graininess(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn facefwd_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn final_plank_color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn float_plank_random(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn grain_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn grainy_stripes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn lighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn make_bumpy(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn make_distorted_stripes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn make_planks(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn mix_in_groove(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply_grain_scale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply_scales(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply_scuff_scale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn multiply_variance(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn normalize_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn one(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn one_over_scale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn randomize_p(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn reflective1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn s_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn s_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn scale_p(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn t_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn t_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn t_width(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn use_it_as_freq(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn veccast1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
    fn vector_plank_random(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> VopWoodplankInnerExt for houdini_ramen_core::graph::InnerGraph<'a, VopWoodplank> {
    fn n_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_global")
    }
    fn n_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("N_input")
    }
    fn p_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_global")
    }
    fn p_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_input")
    }
    fn p_more_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_more_noise")
    }
    fn p_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("P_noise")
    }
    fn add5(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add5")
    }
    fn add_lightness(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add_lightness")
    }
    fn add_stripe_distortion(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("add_stripe_distortion")
    }
    fn base_plank_color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("base_plank_color")
    }
    fn border_width(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("border_width")
    }
    fn bump_amount(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("bump_amount")
    }
    fn clamp_grain_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("clamp_grain_noise")
    }
    fn compute_graininess(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("compute_graininess")
    }
    fn facefwd_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("facefwd_N")
    }
    fn final_plank_color(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("final_plank_color")
    }
    fn float_plank_random(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("float_plank_random")
    }
    fn grain_noise(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("grain_noise")
    }
    fn grainy_stripes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("grainy_stripes")
    }
    fn lighting1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("lighting1")
    }
    fn make_bumpy(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("make_bumpy")
    }
    fn make_distorted_stripes(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("make_distorted_stripes")
    }
    fn make_planks(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("make_planks")
    }
    fn mix_in_groove(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("mix_in_groove")
    }
    fn multiply_grain_scale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply_grain_scale")
    }
    fn multiply_scales(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply_scales")
    }
    fn multiply_scuff_scale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply_scuff_scale")
    }
    fn multiply_variance(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("multiply_variance")
    }
    fn normalize_n(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("normalize_N")
    }
    fn one(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("one")
    }
    fn one_over_scale(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("one_over_scale")
    }
    fn randomize_p(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("randomize_P")
    }
    fn reflective1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("reflective1")
    }
    fn s_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("s_global")
    }
    fn s_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("s_input")
    }
    fn scale_p(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("scale_P")
    }
    fn subinput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("subinput1")
    }
    fn suboutput1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("suboutput1")
    }
    fn t_global(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("t_global")
    }
    fn t_input(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("t_input")
    }
    fn t_width(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("t_width")
    }
    fn use_it_as_freq(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("use_it_as_freq")
    }
    fn veccast1(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("veccast1")
    }
    fn vector_plank_random(&mut self) -> houdini_ramen_core::graph::ExistingNodeRef {
        self.existing_node("vector_plank_random")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VopWorleynoiseFour {
    TwoPoints = 0,
    FourPoints = 1,
}

#[derive(Debug, Clone)]
pub struct VopWorleynoise {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopWorleynoise {
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
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_position_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }
    pub fn set_frequency_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(1, (out.node_id, out.pin));
        self
    }
    pub fn set_offset_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(2, (out.node_id, out.pin));
        self
    }
    pub fn set_metric_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(3, (out.node_id, out.pin));
        self
    }

    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_freq(mut self, val: f32) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_freq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset(mut self, val: f32) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
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
    pub fn with_pos_u(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pos_u".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pos_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos_u".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_freq_u(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "freq_u".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_freq_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq_u".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset_u(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "offset_u".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_offset_u_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset_u".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pos_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pos_v".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pos_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos_v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_freq_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "freq_v".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_freq_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq_v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset_v(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset_v".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset_v_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset_v".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pos_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "pos_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_pos_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_freq_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "freq_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_freq_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freq_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset_v4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "offset_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_offset_v4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset_v4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_four(mut self, val: VopWorleynoiseFour) -> Self {
        self.params.insert(
            "four".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_four_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "four".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
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
    pub fn with_metric(mut self, val: &str) -> Self {
        self.params.insert(
            "metric".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_metric_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "metric".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopWorleynoise {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "worleynoise"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait VopWorleynoiseOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Distance To Point 1 (To Closest Point)"
    fn out_dist1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("dist1".to_string()),
        }
    }
    /// Output pin: "Distance To Point 2"
    fn out_dist2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("dist2".to_string()),
        }
    }
    /// Output pin: "Distance To Point 3"
    fn out_dist3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("dist3".to_string()),
        }
    }
    /// Output pin: "Distance To Point 4"
    fn out_dist4(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("dist4".to_string()),
        }
    }
    /// Output pin: "Seed"
    fn out_seed(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("seed".to_string()),
        }
    }
}

impl VopWorleynoiseOutputs for VopWorleynoise {}
impl VopWorleynoiseOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopWorleynoise> {}

#[derive(Debug, Clone)]
pub struct VopWoven {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl VopWoven {
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
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_s_coordinate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }
    pub fn set_t_coordinate_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(1, (out.node_id, out.pin));
        self
    }
    pub fn set_warp_density_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(2, (out.node_id, out.pin));
        self
    }
    pub fn set_weft_density_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(3, (out.node_id, out.pin));
        self
    }
    pub fn set_yarn_width_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(4, (out.node_id, out.pin));
        self
    }

    pub fn with_warp(mut self, val: f32) -> Self {
        self.params.insert(
            "warp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_warp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "warp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_weft(mut self, val: f32) -> Self {
        self.params.insert(
            "weft".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weft".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_yarn(mut self, val: f32) -> Self {
        self.params.insert(
            "yarn".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_yarn_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "yarn".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for VopWoven {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "woven"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait VopWovenOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Woven Pattern"
    fn out_weave(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("weave".to_string()),
        }
    }
}

impl VopWovenOutputs for VopWoven {}
impl VopWovenOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<VopWoven> {}
