#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMaskfromboundsPrimitivetype {
    PointInstances = 0,
    /// Primitives/Native Instances
    PrimitivesNativeInstances = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMaskfromboundsBoundingtype {
    BoundingBox = 0,
    BoundingSphere = 1,
    /// Primitives (Geometry)
    PrimitivesGeometry = 2,
    CameraFrustrum = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMaskfromboundsPrimitivesource {
    FirstInput = 0,
    SecondInput = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMaskfromboundsInterpolation {
    Constant = 0,
    Uniform = 1,
    Vertex = 2,
    FaceVarying = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMaskfromboundsMethod {
    Maximum = 0,
    Minimum = 1,
    Average = 2,
    FirstMatch = 3,
    LastMatch = 4,
    Centroid = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMaskfromboundsWeightmethod {
    Maximum = 0,
    Minimum = 1,
    Average = 2,
    FirstMatch = 3,
    LastMatch = 4,
    Centroid = 5,
}

#[derive(Debug, Clone)]
pub struct LopMaskfrombounds {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopMaskfrombounds {
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

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: "Stage Containing Bounding Primitives (optional)"
    pub fn set_input_stage_containing_bounding_primitives_opt<
        N: crate::core::types::HoudiniNode,
    >(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Stage Containing Bounding Primitives (optional)" and specifies the output index of the target node.
    pub fn set_input_stage_containing_bounding_primitives_opt_from<
        N: crate::core::types::HoudiniNode,
    >(
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

    pub fn set_input_stage_containing_bounding_primitives_opt_by_name<
        N: crate::core::types::HoudiniNode,
    >(
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
    pub fn with_feather(mut self, val: f32) -> Self {
        self.params.insert(
            "feather".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_feather_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "feather".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: f32) -> Self {
        self.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_float(mut self, val: f32) -> Self {
        self.params.insert(
            "float".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_float_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "float".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

    // --- Float2 parameters ---
    pub fn with_clip(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "clip".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_clip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_size(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_center(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "center".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_center_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "center".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotate(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "rotate".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_float3(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "float3".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_float3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "float3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vector(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vector".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vector_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vector".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_float4(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "float4".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_float4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "float4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_primitivetype(mut self, val: LopMaskfromboundsPrimitivetype) -> Self {
        self.params.insert(
            "primitivetype".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_primitivetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primitivetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundingtype(mut self, val: LopMaskfromboundsBoundingtype) -> Self {
        self.params.insert(
            "boundingtype".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_boundingtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundingtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primitivesource(mut self, val: LopMaskfromboundsPrimitivesource) -> Self {
        self.params.insert(
            "primitivesource".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_primitivesource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primitivesource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interpolation(mut self, val: LopMaskfromboundsInterpolation) -> Self {
        self.params.insert(
            "interpolation".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_interpolation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interpolation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_method(mut self, val: LopMaskfromboundsMethod) -> Self {
        self.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_int(mut self, val: i32) -> Self {
        self.params
            .insert("int".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_int_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "int".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weightmethod(mut self, val: LopMaskfromboundsWeightmethod) -> Self {
        self.params.insert(
            "weightmethod".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_weightmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weightmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_int3(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "int3".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_int3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "int3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int4 parameters ---
    pub fn with_int4(mut self, val: [i32; 4]) -> Self {
        self.params.insert(
            "int4".to_string(),
            crate::core::types::ParamValue::Int4(val),
        );
        self
    }
    pub fn with_int4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "int4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: i32) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "type".to_string(),
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
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundingprimitives(mut self, val: &str) -> Self {
        self.params.insert(
            "boundingprimitives".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_boundingprimitives_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundingprimitives".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name(mut self, val: &str) -> Self {
        self.params.insert(
            "name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weightname(mut self, val: &str) -> Self {
        self.params.insert(
            "weightname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_weightname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "weightname".to_string(),
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
    pub fn with_selectionrulename(mut self, val: &str) -> Self {
        self.params.insert(
            "selectionrulename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_selectionrulename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "selectionrulename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useclipping(mut self, val: bool) -> Self {
        self.params.insert(
            "useclipping".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useclipping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useclipping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inverse(mut self, val: bool) -> Self {
        self.params.insert(
            "inverse".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inverse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inverse".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prefilter(mut self, val: bool) -> Self {
        self.params.insert(
            "prefilter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_prefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addprimvar(mut self, val: bool) -> Self {
        self.params.insert(
            "addprimvar".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addprimvar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addprimvar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bool(mut self, val: bool) -> Self {
        self.params.insert(
            "bool".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bool_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bool".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addweights(mut self, val: bool) -> Self {
        self.params.insert(
            "addweights".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addweights_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addweights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addcollection(mut self, val: bool) -> Self {
        self.params.insert(
            "addcollection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addcollection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addcollection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addselectionrule(mut self, val: bool) -> Self {
        self.params.insert(
            "addselectionrule".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addselectionrule_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addselectionrule".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopMaskfrombounds {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "maskfrombounds"
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
pub enum LopMatchsizeJustifytarget {
    OriginAndUnitSize = 0,
    /// Target Primitive(s)
    TargetPrimitiveS = 1,
    LocationAndSize = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMatchsizeTargetsource {
    FirstInput = 0,
    SecondInput = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMatchsizeMode {
    ApplyTransform = 0,
    TransformGeometry = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMatchsizeJustifyX {
    None = 0,
    Min = 1,
    Center = 2,
    Max = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMatchsizeGoalX {
    Same = 0,
    Min = 1,
    Center = 2,
    Max = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMatchsizeJustifyY {
    None = 0,
    Min = 1,
    Center = 2,
    Max = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMatchsizeGoalY {
    Same = 0,
    Min = 1,
    Center = 2,
    Max = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMatchsizeJustifyZ {
    None = 0,
    Min = 1,
    Center = 2,
    Max = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMatchsizeGoalZ {
    Same = 0,
    Min = 1,
    Center = 2,
    Max = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMatchsizeJustify {
    None = 0,
    YMin = 1,
    Center = 2,
    YMax = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMatchsizeScaleAxis {
    X = 0,
    Y = 1,
    Z = 2,
    BestFit = 3,
    Perimeter = 4,
    Area = 5,
    Volume = 6,
}

#[derive(Debug, Clone)]
pub struct LopMatchsize {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopMatchsize {
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

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: "Input Target"
    pub fn set_input_input_target<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Input Target" and specifies the output index of the target node.
    pub fn set_input_input_target_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_target_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_importframe(mut self, val: f32) -> Self {
        self.params.insert(
            "importframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_importframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset_x(mut self, val: f32) -> Self {
        self.params.insert(
            "offset_x".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offset_x_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset_x".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset_y(mut self, val: f32) -> Self {
        self.params.insert(
            "offset_y".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offset_y_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset_y".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset_z(mut self, val: f32) -> Self {
        self.params.insert(
            "offset_z".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offset_z_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset_z".to_string(),
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
    pub fn with_scale_x_mult(mut self, val: f32) -> Self {
        self.params.insert(
            "scale_x_mult".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_x_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale_x_mult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale_y_mult(mut self, val: f32) -> Self {
        self.params.insert(
            "scale_y_mult".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_y_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale_y_mult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale_z_mult(mut self, val: f32) -> Self {
        self.params.insert(
            "scale_z_mult".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_z_mult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale_z_mult".to_string(),
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
    pub fn with_size(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_mode(mut self, val: LopMatchsizeMode) -> Self {
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

    // --- Menu parameters ---
    pub fn with_justifytarget(mut self, val: LopMatchsizeJustifytarget) -> Self {
        self.params.insert(
            "justifytarget".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_justifytarget_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "justifytarget".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetsource(mut self, val: LopMatchsizeTargetsource) -> Self {
        self.params.insert(
            "targetsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_justify_x(mut self, val: LopMatchsizeJustifyX) -> Self {
        self.params.insert(
            "justify_x".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_justify_x_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "justify_x".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goal_x(mut self, val: LopMatchsizeGoalX) -> Self {
        self.params.insert(
            "goal_x".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_goal_x_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goal_x".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_justify_y(mut self, val: LopMatchsizeJustifyY) -> Self {
        self.params.insert(
            "justify_y".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_justify_y_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "justify_y".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goal_y(mut self, val: LopMatchsizeGoalY) -> Self {
        self.params.insert(
            "goal_y".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_goal_y_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goal_y".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_justify_z(mut self, val: LopMatchsizeJustifyZ) -> Self {
        self.params.insert(
            "justify_z".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_justify_z_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "justify_z".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goal_z(mut self, val: LopMatchsizeGoalZ) -> Self {
        self.params.insert(
            "goal_z".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_goal_z_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goal_z".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_justify(mut self, val: LopMatchsizeJustify) -> Self {
        self.params.insert(
            "justify".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_justify_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "justify".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale_axis(mut self, val: LopMatchsizeScaleAxis) -> Self {
        self.params.insert(
            "scale_axis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_scale_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale_axis".to_string(),
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
    pub fn with_transformname(mut self, val: &str) -> Self {
        self.params.insert(
            "transformname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transformname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transformname".to_string(),
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
    pub fn with_usefastbounds(mut self, val: bool) -> Self {
        self.params.insert(
            "usefastbounds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usefastbounds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usefastbounds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchperprim(mut self, val: bool) -> Self {
        self.params.insert(
            "matchperprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_matchperprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchperprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dotranslate(mut self, val: bool) -> Self {
        self.params.insert(
            "dotranslate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dotranslate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dotranslate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doscale(mut self, val: bool) -> Self {
        self.params.insert(
            "doscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformscale(mut self, val: bool) -> Self {
        self.params.insert(
            "uniformscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniformscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale_x(mut self, val: bool) -> Self {
        self.params.insert(
            "scale_x".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_scale_x_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale_x".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale_y(mut self, val: bool) -> Self {
        self.params.insert(
            "scale_y".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_scale_y_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale_y".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale_z(mut self, val: bool) -> Self {
        self.params.insert(
            "scale_z".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_scale_z_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale_z".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopMatchsize {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "matchsize"
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
pub struct LopMateriallibrary {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopMateriallibrary {
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

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Button parameters ---
    pub fn trigger_fillmaterials(mut self) -> Self {
        self.params.insert(
            "fillmaterials".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_matpathprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "matpathprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matpathprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matpathprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matnet(mut self, val: &str) -> Self {
        self.params.insert(
            "matnet".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matnet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matnet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_containerpath(mut self, val: &str) -> Self {
        self.params.insert(
            "containerpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_containerpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "containerpath".to_string(),
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
    pub fn with_genpreviewshaders(mut self, val: bool) -> Self {
        self.params.insert(
            "genpreviewshaders".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_genpreviewshaders_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "genpreviewshaders".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allowparmanim(mut self, val: bool) -> Self {
        self.params.insert(
            "allowparmanim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowparmanim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowparmanim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_referencerendervars(mut self, val: bool) -> Self {
        self.params.insert(
            "referencerendervars".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_referencerendervars_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "referencerendervars".to_string(),
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
    pub fn with_matflag_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("matflag{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_matflag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("matflag{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_assign_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("assign{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_assign_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("assign{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopMateriallibrary {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "materiallibrary"
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
pub enum LopMateriallinkerCreateprims {
    EditExistingPrimitives = 0,
    CreateNewPrimitives = 1,
}

#[derive(Debug, Clone)]
pub struct LopMateriallinker {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopMateriallinker {
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

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Button parameters ---
    pub fn trigger_reload(mut self) -> Self {
        self.params
            .insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Menu parameters ---
    pub fn with_createprims_inst(
        mut self,
        index1: usize,
        val: LopMateriallinkerCreateprims,
    ) -> Self {
        self.params.insert(
            format!("createprims_{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createprims_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("createprims_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_primpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("primpath_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("primpath_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filepath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("filepath_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filepath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filepath_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filerefprimpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("filerefprimpath_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filerefprimpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filerefprimpath_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_id_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("link_id_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_link_id_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_id_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_prim_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("link_prim_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_link_prim_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_prim_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_includes_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("link_includes_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_link_includes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_includes_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_excludes_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("link_excludes_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_link_excludes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_excludes_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_type_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("link_type_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_link_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_type_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uiconfig(mut self, val: &str) -> Self {
        self.params.insert(
            "uiconfig".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uiconfig_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uiconfig".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enabled_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enabled_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabled_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enabled_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_enabled_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("link_enabled_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_link_enabled_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_enabled_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_ispathexpression_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("link_ispathexpression_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_link_ispathexpression_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_ispathexpression_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_link_reversed_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("link_reversed_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_link_reversed_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("link_reversed_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopMateriallinker {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "materiallinker"
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
pub enum LopMaterialvariationPrimitivetype {
    PointInstances = 0,
    /// Primitives/Native Instances
    PrimitivesNativeInstances = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMaterialvariationSourcemode {
    InternalSop = 0,
    ExternalSop = 1,
    Uniform = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMaterialvariationInterpolation {
    Constant = 0,
    Uniform = 1,
    Vertex = 2,
    FaceVarying = 3,
}

#[derive(Debug, Clone)]
pub struct LopMaterialvariation {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopMaterialvariation {
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

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_float_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("float{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_float_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("float{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_double_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("double{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_double_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("double{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_float2_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(
            format!("float2{}", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_float2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("float2{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_half2_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(
            format!("half2{}", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_half2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("half2{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_double2_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(
            format!("double2{}", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_double2_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("double2{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texcoord2f_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(
            format!("texCoord2f{}", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_texcoord2f_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("texCoord2f{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_half3_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("half3{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_half3_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("half3{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_float3_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("float3{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_float3_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("float3{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_double3_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("double3{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_double3_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("double3{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_point3f_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("point3f{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_point3f_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("point3f{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_point3d_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("point3d{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_point3d_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("point3d{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vector3f_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("vector3f{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vector3f_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("vector3f{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vector3d_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("vector3d{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vector3d_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("vector3d{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normal3f_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("normal3f{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_normal3f_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("normal3f{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normal3d_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("normal3d{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_normal3d_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("normal3d{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_color3f_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("color3f{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_color3f_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("color3f{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_color3d_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("color3d{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_color3d_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("color3d{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_texcoord3f_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("texCoord3f{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_texcoord3f_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("texCoord3f{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_color4f_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("color4f{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_color4f_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("color4f{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_color4d_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("color4d{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_color4d_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("color4d{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_half4_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("half4{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_half4_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("half4{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_float4_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("float4{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_float4_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("float4{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_double4_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("double4{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_double4_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("double4{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quath_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("quath{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_quath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("quath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quatf_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("quatf{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_quatf_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("quatf{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quatd_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("quatd{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_quatd_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("quatd{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_primitivetype(mut self, val: LopMaterialvariationPrimitivetype) -> Self {
        self.params.insert(
            "primitivetype".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_primitivetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primitivetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcemode_inst(
        mut self,
        index1: usize,
        val: LopMaterialvariationSourcemode,
    ) -> Self {
        self.params.insert(
            format!("sourcemode{}", index1),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_sourcemode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("sourcemode{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interpolation_inst(
        mut self,
        index1: usize,
        val: LopMaterialvariationInterpolation,
    ) -> Self {
        self.params.insert(
            format!("interpolation{}", index1),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_interpolation_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("interpolation{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_int_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("int{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_int_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("int{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_int64_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("int64{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_int64_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("int64{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_int3_inst(mut self, index1: usize, val: [i32; 3]) -> Self {
        self.params.insert(
            format!("int3{}", index1),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_int3_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("int3{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int4 parameters ---
    pub fn with_int4_inst(mut self, index1: usize, val: [i32; 4]) -> Self {
        self.params.insert(
            format!("int4{}", index1),
            crate::core::types::ParamValue::Int4(val),
        );
        self
    }
    pub fn with_int4_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("int4{}", index1),
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
    pub fn with_soppath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("soppath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("soppath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_string_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("string{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_string_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("string{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_image_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("image{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_image_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("image{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_token_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("token{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_token_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("token{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_asset_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("asset{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_asset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("asset{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_snippet_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("snippet{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_snippet_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("snippet{}", index1),
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
    pub fn with_matchbyid_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("matchbyid{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_matchbyid_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("matchbyid{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalar_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("scalar{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_scalar_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("scalar{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesnippet_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("usesnippet{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesnippet_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("usesnippet{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bool_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("bool{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bool_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bool{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopMaterialvariation {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "materialvariation"
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

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("sopnet/modify")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopMaterialvariationInnerExt {
    fn from_lops(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopMaterialvariationInnerExt for crate::core::graph::InnerGraph<'a, LopMaterialvariation> {
    fn from_lops(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("sopnet/modify/from_lops")
    }
}

#[derive(Debug, Clone)]
pub struct LopMatnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopMatnet {
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

impl crate::core::types::HoudiniNode for LopMatnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "matnet"
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
pub struct LopMerge {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl LopMerge {
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self.next_input_index += 1;
        self
    }

    pub fn add_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self.next_input_index += 1;
        self
    }

    // --- String parameters ---
    pub fn with_mergestyle(mut self, val: &str) -> Self {
        self.params.insert(
            "mergestyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mergestyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mergestyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
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
}

impl crate::core::types::HoudiniNode for LopMerge {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "merge"
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
pub enum LopMergepointinstancersPrunemethod {
    Hide = 0,
    Delete = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMergepointinstancersNewprototypes {
    InPlace = 0,
    Reference = 1,
}

#[derive(Debug, Clone)]
pub struct LopMergepointinstancers {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopMergepointinstancers {
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

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Int parameters ---
    pub fn with_prunemethod(mut self, val: LopMergepointinstancersPrunemethod) -> Self {
        self.params.insert(
            "prunemethod".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_prunemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prunemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_newprototypes(mut self, val: LopMergepointinstancersNewprototypes) -> Self {
        self.params.insert(
            "newprototypes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_newprototypes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newprototypes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
}

impl crate::core::types::HoudiniNode for LopMergepointinstancers {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "mergepointinstancers"
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
pub enum LopMeshCreateprims {
    Edit = 0,
    Create = 1,
    /// Force Edit (Ignore Editable Flag)
    ForceEditIgnoreEditableFlag = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMeshXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMeshRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone)]
pub struct LopMesh {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopMesh {
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

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Float parameters ---
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
    pub fn with_createprims(mut self, val: LopMeshCreateprims) -> Self {
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
    pub fn with_xord(mut self, val: LopMeshXord) -> Self {
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
    pub fn with_rord(mut self, val: LopMeshRord) -> Self {
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
    pub fn with_subdivisionscheme_control(mut self, val: &str) -> Self {
        self.params.insert(
            "subdivisionScheme_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subdivisionscheme_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subdivisionScheme_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subdivisionscheme(mut self, val: &str) -> Self {
        self.params.insert(
            "subdivisionScheme".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subdivisionscheme_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subdivisionScheme".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trianglesubdivisionrule_control(mut self, val: &str) -> Self {
        self.params.insert(
            "triangleSubdivisionRule_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_trianglesubdivisionrule_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triangleSubdivisionRule_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trianglesubdivisionrule(mut self, val: &str) -> Self {
        self.params.insert(
            "triangleSubdivisionRule".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_trianglesubdivisionrule_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triangleSubdivisionRule".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_facevaryinglinearinterpolation_control(mut self, val: &str) -> Self {
        self.params.insert(
            "faceVaryingLinearInterpolation_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_facevaryinglinearinterpolation_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "faceVaryingLinearInterpolation_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_facevaryinglinearinterpolation(mut self, val: &str) -> Self {
        self.params.insert(
            "faceVaryingLinearInterpolation".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_facevaryinglinearinterpolation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "faceVaryingLinearInterpolation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interpolateboundary_control(mut self, val: &str) -> Self {
        self.params.insert(
            "interpolateBoundary_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_interpolateboundary_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interpolateBoundary_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interpolateboundary(mut self, val: &str) -> Self {
        self.params.insert(
            "interpolateBoundary".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_interpolateboundary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interpolateBoundary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientation_control(mut self, val: &str) -> Self {
        self.params.insert(
            "orientation_control".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_orientation_control_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orientation_control".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientation(mut self, val: &str) -> Self {
        self.params.insert(
            "orientation".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_orientation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orientation".to_string(),
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

impl crate::core::types::HoudiniNode for LopMesh {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "mesh"
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
pub struct LopModifypaths {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopModifypaths {
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

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
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
    pub fn with_findprefix_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("findprefix{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_findprefix_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("findprefix{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_replaceprefix_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("replaceprefix{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_replaceprefix_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("replaceprefix{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_findsuffix_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("findsuffix{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_findsuffix_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("findsuffix{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_replacesuffix_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("replacesuffix{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_replacesuffix_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("replacesuffix{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pythoncode_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("pythoncode{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pythoncode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pythoncode{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_allowchained(mut self, val: bool) -> Self {
        self.params.insert(
            "allowchained".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowchained_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowchained".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopModifypaths {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "modifypaths"
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
pub enum LopModifypointinstancesGlobalsourcemode {
    InternalSop = 0,
    ExternalSop = 1,
    UniformValues = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopModifypointinstancesSourcemode {
    InternalSop = 0,
    ExternalSop = 1,
    UniformValues = 2,
    GlobalSetting = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopModifypointinstancesXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopModifypointinstancesRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopModifypointinstancesSourcemode1 {
    InternalSop = 0,
    ExternalSop = 1,
    UniformValues = 2,
    GlobalSetting = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopModifypointinstancesPrunemethod {
    DoNothing = 0,
    Hide = 1,
    Delete = 2,
    InternalSop = 3,
    Deactivate = 4,
}

#[derive(Debug, Clone)]
pub struct LopModifypointinstances {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopModifypointinstances {
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

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Button parameters ---
    pub fn trigger_movecentroidtoorigin(mut self) -> Self {
        self.params.insert(
            "movecentroidtoorigin".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_movepivottocentroid(mut self) -> Self {
        self.params.insert(
            "movepivottocentroid".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_matchpivot(mut self) -> Self {
        self.params.insert(
            "matchpivot".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_populate(mut self) -> Self {
        self.params.insert(
            "populate".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_soppopulate(mut self) -> Self {
        self.params.insert(
            "soppopulate".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_populatefromsop(mut self) -> Self {
        self.params.insert(
            "populatefromsop".to_string(),
            crate::core::types::ParamValue::Button,
        );
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
    pub fn with_float_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("float{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_float_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("float{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_double_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("double{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_double_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("double{}", index1),
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
    pub fn with_half3_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("half3{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_half3_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("half3{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_float3_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("float3{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_float3_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("float3{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_double3_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("double3{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_double3_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("double3{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_point3f_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("point3f{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_point3f_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("point3f{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_point3d_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("point3d{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_point3d_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("point3d{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vector3f_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("vector3f{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vector3f_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("vector3f{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vector3d_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("vector3d{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vector3d_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("vector3d{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normal3f_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("normal3f{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_normal3f_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("normal3f{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normal3d_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("normal3d{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_normal3d_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("normal3d{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_color3f_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("color3f{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_color3f_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("color3f{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_color3d_inst(mut self, index1: usize, val: [f32; 3]) -> Self {
        self.params.insert(
            format!("color3d{}", index1),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_color3d_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("color3d{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_half4_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("half4{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_half4_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("half4{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_float4_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("float4{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_float4_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("float4{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_double4_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("double4{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_double4_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("double4{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quath_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("quath{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_quath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("quath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quatf_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("quatf{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_quatf_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("quatf{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quatd_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("quatd{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_quatd_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("quatd{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_globalsourcemode(mut self, val: LopModifypointinstancesGlobalsourcemode) -> Self {
        self.params.insert(
            "globalsourcemode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_globalsourcemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globalsourcemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcemode(mut self, val: LopModifypointinstancesSourcemode) -> Self {
        self.params.insert(
            "sourcemode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_sourcemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: LopModifypointinstancesXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
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
    pub fn with_rord(mut self, val: LopModifypointinstancesRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
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
    pub fn with_sourcemode_inst(
        mut self,
        index1: usize,
        val: LopModifypointinstancesSourcemode1,
    ) -> Self {
        self.params.insert(
            format!("sourcemode{}", index1),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_sourcemode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("sourcemode{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_int_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("int{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_int_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("int{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_int64_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("int64{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_int64_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("int64{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prunemethod(mut self, val: LopModifypointinstancesPrunemethod) -> Self {
        self.params.insert(
            "prunemethod".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_prunemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prunemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prune(mut self, val: i32) -> Self {
        self.params.insert(
            "prune".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_prune_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prune".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_int3_inst(mut self, index1: usize, val: [i32; 3]) -> Self {
        self.params.insert(
            format!("int3{}", index1),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_int3_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("int3{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int4 parameters ---
    pub fn with_int4_inst(mut self, index1: usize, val: [i32; 4]) -> Self {
        self.params.insert(
            format!("int4{}", index1),
            crate::core::types::ParamValue::Int4(val),
        );
        self
    }
    pub fn with_int4_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("int4{}", index1),
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
    pub fn with_globalsoppath(mut self, val: &str) -> Self {
        self.params.insert(
            "globalsoppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_globalsoppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globalsoppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_globalgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "globalgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_globalgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globalgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_soppath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("soppath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("soppath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("group{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("group{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_string_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("string{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_string_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("string{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_token_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("token{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_token_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("token{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_asset_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("asset{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_asset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("asset{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_snippet_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("snippet{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_snippet_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("snippet{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_edittransform(mut self, val: bool) -> Self {
        self.params.insert(
            "edittransform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_edittransform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "edittransform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchbyid(mut self, val: bool) -> Self {
        self.params.insert(
            "matchbyid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_matchbyid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchbyid".to_string(),
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
    pub fn with_matchbyid_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("matchbyid{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_matchbyid_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("matchbyid{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usev_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("usev{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usev_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("usev{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usew_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("usew{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usew_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("usew{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useaccel_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("useaccel{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useaccel_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("useaccel{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesnippet_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("usesnippet{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesnippet_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("usesnippet{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bool_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("bool{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bool_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("bool{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addduplicates(mut self, val: bool) -> Self {
        self.params.insert(
            "addduplicates".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addduplicates_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addduplicates".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_skipvalidation(mut self, val: bool) -> Self {
        self.params.insert(
            "skipvalidation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_skipvalidation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "skipvalidation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopModifypointinstances {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "modifypointinstances"
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

    fn get_dive_target(&self) -> Option<&'static str> {
        Some("sopnet/modify")
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait LopModifypointinstancesInnerExt {
    fn from_lops(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> LopModifypointinstancesInnerExt
    for crate::core::graph::InnerGraph<'a, LopModifypointinstances>
{
    fn from_lops(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("sopnet/modify/from_lops")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopMotionblurVelapproximation {
    BackwardDifference = 0,
    CentralDifference = 1,
    ForwardDifference = 2,
}

#[derive(Debug, Clone)]
pub struct LopMotionblur {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl LopMotionblur {
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

    /// Connects to input 0: "Input Stage"
    pub fn set_input_input_stage<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Input Stage" and specifies the output index of the target node.
    pub fn set_input_input_stage_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_stage_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Float parameters ---
    pub fn with_scale_velocity(mut self, val: f32) -> Self {
        self.params.insert(
            "scale_velocity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_velocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale_velocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dt(mut self, val: f32) -> Self {
        self.params
            .insert("dt".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_dt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dt".to_string(),
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

    // --- Menu parameters ---
    pub fn with_velapproximation(mut self, val: LopMotionblurVelapproximation) -> Self {
        self.params.insert(
            "velapproximation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_velapproximation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velapproximation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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

    // --- Toggle parameters ---
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
    pub fn with_computemotion(mut self, val: bool) -> Self {
        self.params.insert(
            "computemotion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computemotion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computemotion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_computevelocity(mut self, val: bool) -> Self {
        self.params.insert(
            "computevelocity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computevelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computevelocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resampletransforms(mut self, val: bool) -> Self {
        self.params.insert(
            "resampletransforms".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resampletransforms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resampletransforms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_includeoriginal(mut self, val: bool) -> Self {
        self.params.insert(
            "includeoriginal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_includeoriginal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "includeoriginal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for LopMotionblur {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "motionblur"
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
