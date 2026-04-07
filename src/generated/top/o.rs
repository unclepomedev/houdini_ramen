#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopObjnetXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopObjnetRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopObjnetPreXform {
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
pub enum TopObjnetUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct TopObjnet {
    pub base: crate::core::types::NodeBase,
}

impl TopObjnet {
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
    pub fn with_pos(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.base.params.insert(
            "bank".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bank".to_string(),
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
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pr".to_string(),
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
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathorient".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.base.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Int(val),
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

    // --- Menu parameters ---
    pub fn with_xord(mut self, val: TopObjnetXord) -> Self {
        self.base.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord(mut self, val: TopObjnetRord) -> Self {
        self.base.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rOrd".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pre_xform(mut self, val: TopObjnetPreXform) -> Self {
        self.base.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pre_xform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: TopObjnetUparmtype) -> Self {
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

    // --- String parameters ---
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookatpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pathobjpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.base.params.insert(
            "label1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "label1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.base.params.insert(
            "label2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "label2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.base.params.insert(
            "label3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "label3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.base.params.insert(
            "label4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "label4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputobj".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputobj".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.base.params.insert(
            "visibleobjects".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "visibleobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pickscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keeppos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.base.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "childcomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.base.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tdisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.base.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "picking".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.base.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "caching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.base.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "use_dcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopObjnet {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "objnet"
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
pub enum TopOpenimageioPdgCooktype {
    /// In-Process
    InMinusProcess = 0,
    /// Out-of-Process
    OutMinusOfMinusProcess = 1,
    Service = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioPdgUseserviceblock {
    Never = 0,
    IfServiceNameMatches = 1,
    Always = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioBatchmode {
    Off = 0,
    AllItemsInOneBatch = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioCookwhen {
    AllItemsAreReady = 0,
    FirstItemIsReady = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioInputsource {
    UpstreamOutputFiles = 0,
    CustomFilePath = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioInputsort {
    None = 0,
    NaturalFilename = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioOperation {
    None = 0,
    ColorConvert = 1,
    ColorMap = 2,
    Resize = 3,
    Mosaic = 4,
    TextAndShape = 5,
    ColorTransform = 6,
    Custom = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioColorconfigfiletype {
    HoudiniConfiguration = 0,
    CustomFilePath = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioResizetype {
    /// Absolute (Pixels)
    AbsolutePixels = 0,
    /// Relative (Percent)
    RelativePercent = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioSuboperation {
    Text = 0,
    Box = 1,
    Line = 2,
    Point = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioTextpostype {
    /// Absolute (Pixels)
    AbsolutePixels = 0,
    /// Relative (Percent)
    RelativePercent = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioTextxalign {
    Left = 0,
    Right = 1,
    Center = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioTextyalign {
    Base = 0,
    Top = 1,
    Bottom = 2,
    Center = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioBoxpostype {
    /// Absolute (Pixels)
    AbsolutePixels = 0,
    /// Relative (Percent)
    RelativePercent = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioLinepostype {
    /// Absolute (Pixels)
    AbsolutePixels = 0,
    /// Relative (Percent)
    RelativePercent = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioPointpostype {
    /// Absolute (Pixels)
    AbsolutePixels = 0,
    /// Relative (Percent)
    RelativePercent = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioSubimages {
    Default = 0,
    AllSubimages = 1,
    SpecificSubimages = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioSubimagereftype {
    Index = 0,
    Name = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioValuetype {
    NoValue = 0,
    AttributeName = 1,
    CustomValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioOpenimageiobinarytype {
    Hoiiotool = 0,
    CustomFilePath = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopOpenimageioPdgCommandtype {
    UseDefault = 0,
    CustomScript = 1,
    CustomCommand = 2,
}

#[derive(Debug, Clone)]
pub struct TopOpenimageio {
    pub base: crate::core::types::NodeBase,
}

impl TopOpenimageio {
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

    /// Connects to input 0: "input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_manageservices(mut self) -> Self {
        self.base.params.insert(
            "manageservices".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Data parameters ---
    pub fn with_argmodifiers_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("argmodifiers{}", index1),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_argmodifiers_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("argmodifiers{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_resizewidthpercent(mut self, val: f32) -> Self {
        self.base.params.insert(
            "resizewidthpercent".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_resizewidthpercent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resizewidthpercent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resizeheightpercent(mut self, val: f32) -> Self {
        self.base.params.insert(
            "resizeheightpercent".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_resizeheightpercent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resizeheightpercent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_textpospercent_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.base.params.insert(
            format!("textpospercent{}", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_textpospercent_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("textpospercent{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_box1pospercent_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.base.params.insert(
            format!("box1pospercent{}", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_box1pospercent_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("box1pospercent{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_box2pospercent_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.base.params.insert(
            format!("box2pospercent{}", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_box2pospercent_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("box2pospercent{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_linepospercent_inst(mut self, index1: usize, index2: usize, val: [f32; 2]) -> Self {
        self.base.params.insert(
            format!("linepospercent{}_{}", index1, index2),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_linepospercent_inst_expr(
        mut self,
        index1: usize,
        index2: usize,
        expr: &str,
    ) -> Self {
        self.base.params.insert(
            format!("linepospercent{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointpospercent_inst(
        mut self,
        index1: usize,
        index2: usize,
        val: [f32; 2],
    ) -> Self {
        self.base.params.insert(
            format!("pointpospercent{}_{}", index1, index2),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pointpospercent_inst_expr(
        mut self,
        index1: usize,
        index2: usize,
        expr: &str,
    ) -> Self {
        self.base.params.insert(
            format!("pointpospercent{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_textcolor_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.base.params.insert(
            format!("textcolor{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_textcolor_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("textcolor{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boxcolor_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.base.params.insert(
            format!("boxcolor{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_boxcolor_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("boxcolor{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_linecolor_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.base.params.insert(
            format!("linecolor{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_linecolor_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("linecolor{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointcolor_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.base.params.insert(
            format!("pointcolor{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_pointcolor_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("pointcolor{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_resizewidthpixels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "resizewidthpixels".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resizewidthpixels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resizewidthpixels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resizeheightpixels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "resizeheightpixels".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resizeheightpixels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resizeheightpixels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mosaicpadding(mut self, val: i32) -> Self {
        self.base.params.insert(
            "mosaicpadding".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mosaicpadding_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mosaicpadding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_textsize_inst(mut self, index1: usize, val: i32) -> Self {
        self.base.params.insert(
            format!("textsize{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_textsize_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("textsize{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_textshadowsize_inst(mut self, index1: usize, val: i32) -> Self {
        self.base.params.insert(
            format!("textshadowsize{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_textshadowsize_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("textshadowsize{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subimageindex_inst(mut self, index1: usize, val: i32) -> Self {
        self.base.params.insert(
            format!("subimageindex{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_subimageindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("subimageindex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_mosaicnumtiles(mut self, val: [i32; 2]) -> Self {
        self.base.params.insert(
            "mosaicnumtiles".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_mosaicnumtiles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mosaicnumtiles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mosaictilesize(mut self, val: [i32; 2]) -> Self {
        self.base.params.insert(
            "mosaictilesize".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_mosaictilesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mosaictilesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_textpos_inst(mut self, index1: usize, val: [i32; 2]) -> Self {
        self.base.params.insert(
            format!("textpos{}", index1),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_textpos_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("textpos{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_box1pos_inst(mut self, index1: usize, val: [i32; 2]) -> Self {
        self.base.params.insert(
            format!("box1pos{}", index1),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_box1pos_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("box1pos{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_box2pos_inst(mut self, index1: usize, val: [i32; 2]) -> Self {
        self.base.params.insert(
            format!("box2pos{}", index1),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_box2pos_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("box2pos{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_linepos_inst(mut self, index1: usize, index2: usize, val: [i32; 2]) -> Self {
        self.base.params.insert(
            format!("linepos{}_{}", index1, index2),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_linepos_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("linepos{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointpos_inst(mut self, index1: usize, index2: usize, val: [i32; 2]) -> Self {
        self.base.params.insert(
            format!("pointpos{}_{}", index1, index2),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_pointpos_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("pointpos{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cachemode(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_cachemode".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_cachemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_cachemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cooktype(mut self, val: TopOpenimageioPdgCooktype) -> Self {
        self.base.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_useserviceblock(mut self, val: TopOpenimageioPdgUseserviceblock) -> Self {
        self.base.params.insert(
            "pdg_useserviceblock".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_useserviceblock_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_useserviceblock".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_batchmode(mut self, val: TopOpenimageioBatchmode) -> Self {
        self.base.params.insert(
            "batchmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_batchmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "batchmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cookwhen(mut self, val: TopOpenimageioCookwhen) -> Self {
        self.base.params.insert(
            "cookwhen".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cookwhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cookwhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputsource(mut self, val: TopOpenimageioInputsource) -> Self {
        self.base.params.insert(
            "inputsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inputsource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputsort(mut self, val: TopOpenimageioInputsort) -> Self {
        self.base.params.insert(
            "inputsort".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inputsort_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputsort".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_operation(mut self, val: TopOpenimageioOperation) -> Self {
        self.base.params.insert(
            "operation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_operation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "operation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorconfigfiletype(mut self, val: TopOpenimageioColorconfigfiletype) -> Self {
        self.base.params.insert(
            "colorconfigfiletype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_colorconfigfiletype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "colorconfigfiletype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resizetype(mut self, val: TopOpenimageioResizetype) -> Self {
        self.base.params.insert(
            "resizetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resizetype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resizetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_suboperation_inst(
        mut self,
        index1: usize,
        val: TopOpenimageioSuboperation,
    ) -> Self {
        self.base.params.insert(
            format!("suboperation{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_suboperation_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("suboperation{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_textpostype_inst(mut self, index1: usize, val: TopOpenimageioTextpostype) -> Self {
        self.base.params.insert(
            format!("textpostype{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_textpostype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("textpostype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_textxalign_inst(mut self, index1: usize, val: TopOpenimageioTextxalign) -> Self {
        self.base.params.insert(
            format!("textxalign{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_textxalign_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("textxalign{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_textyalign_inst(mut self, index1: usize, val: TopOpenimageioTextyalign) -> Self {
        self.base.params.insert(
            format!("textyalign{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_textyalign_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("textyalign{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boxpostype_inst(mut self, index1: usize, val: TopOpenimageioBoxpostype) -> Self {
        self.base.params.insert(
            format!("boxpostype{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_boxpostype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("boxpostype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_linepostype_inst(mut self, index1: usize, val: TopOpenimageioLinepostype) -> Self {
        self.base.params.insert(
            format!("linepostype{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_linepostype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("linepostype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointpostype_inst(
        mut self,
        index1: usize,
        val: TopOpenimageioPointpostype,
    ) -> Self {
        self.base.params.insert(
            format!("pointpostype{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pointpostype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("pointpostype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subimages(mut self, val: TopOpenimageioSubimages) -> Self {
        self.base.params.insert(
            "subimages".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_subimages_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "subimages".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subimagereftype_inst(
        mut self,
        index1: usize,
        val: TopOpenimageioSubimagereftype,
    ) -> Self {
        self.base.params.insert(
            format!("subimagereftype{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_subimagereftype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("subimagereftype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_valuetype_inst(mut self, index1: usize, val: TopOpenimageioValuetype) -> Self {
        self.base.params.insert(
            format!("valuetype{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_valuetype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("valuetype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_openimageiobinarytype(mut self, val: TopOpenimageioOpenimageiobinarytype) -> Self {
        self.base.params.insert(
            "openimageiobinarytype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_openimageiobinarytype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "openimageiobinarytype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopOpenimageioPdgWorkitemlabel) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopOpenimageioPdgWorkitempriority) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_commandtype(mut self, val: TopOpenimageioPdgCommandtype) -> Self {
        self.base.params.insert(
            "pdg_commandtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_commandtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_commandtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pdg_servicename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_servicename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_servicename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_servicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputfiletag(mut self, val: &str) -> Self {
        self.base.params.insert(
            "inputfiletag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputfiletag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputfiletag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputfilepath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "inputfilepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputfilepath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputfilepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputfilepath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputfilepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputfilepath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputfilepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputfiletag(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputfiletag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputfiletag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputfiletag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputcolorspace(mut self, val: &str) -> Self {
        self.base.params.insert(
            "inputcolorspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputcolorspace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputcolorspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputcolorspace(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputcolorspace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputcolorspace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputcolorspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customcolorconfig(mut self, val: &str) -> Self {
        self.base.params.insert(
            "customcolorconfig".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customcolorconfig_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "customcolorconfig".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colormapname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "colormapname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_colormapname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "colormapname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_textfontfile_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("textfontfile{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_textfontfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("textfontfile{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_text_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("text{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_text_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("text{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "lookname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_lookname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "displayname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_displayname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displayname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viewname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "viewname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_viewname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "viewname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customoperation(mut self, val: &str) -> Self {
        self.base.params.insert(
            "customoperation".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customoperation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "customoperation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_subimagename_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("subimagename{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_subimagename_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("subimagename{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_argumentname_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("argumentname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_argumentname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("argumentname{}", index1),
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
    pub fn with_customvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("customvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("customvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customargs(mut self, val: &str) -> Self {
        self.base.params.insert(
            "customargs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customargs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "customargs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customopenimageiobinary(mut self, val: &str) -> Self {
        self.base.params.insert(
            "customopenimageiobinary".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customopenimageiobinary_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "customopenimageiobinary".to_string(),
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
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_command(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_command".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_command_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_command".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablemosaictilesize(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablemosaictilesize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemosaictilesize_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablemosaictilesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablemosaicpadding(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablemosaicpadding".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemosaicpadding_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablemosaicpadding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletextfont_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("enabletextfont{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletextfont_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("enabletextfont{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boxfilltoggle_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("boxfilltoggle{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_boxfilltoggle_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("boxfilltoggle{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableociolook(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableociolook".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableociolook_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableociolook".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableociodisplay(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableociodisplay".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableociodisplay_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableociodisplay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablemipmap(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablemipmap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemipmap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablemipmap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablecustomargs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablecustomargs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecustomargs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablecustomargs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopOpenimageio {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "openimageio"
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
pub enum TopOpnotifyReloadtype {
    RecookNode = 0,
    PressButton = 1,
}

#[derive(Debug, Clone)]
pub struct TopOpnotify {
    pub base: crate::core::types::NodeBase,
}

impl TopOpnotify {
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

    /// Connects to input 0: "input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reloadtype(mut self, val: TopOpnotifyReloadtype) -> Self {
        self.base.params.insert(
            "reloadtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_reloadtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reloadtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_oppath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "oppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_oppath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "oppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_buttonname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "buttonname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_buttonname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "buttonname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopOpnotify {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "opnotify"
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
pub enum TopOutputPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone)]
pub struct TopOutput {
    pub base: crate::core::types::NodeBase,
}

impl TopOutput {
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

    /// Connects to input 0: "input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_outputidx(mut self, val: i32) -> Self {
        self.base.params.insert(
            "outputidx".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_outputidx_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputidx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemlabel(mut self, val: TopOutputPdgWorkitemlabel) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopOutput {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "output"
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
