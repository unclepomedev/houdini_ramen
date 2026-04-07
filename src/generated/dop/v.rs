#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopTwod {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopVoxelplane {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldVoxelplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopUniformvoxels {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopUniformdiv {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopDiv {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopDivsize {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopSize {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopT {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopVoxelsample {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldVoxelsample {
    Center = 0,
    Faces = 1,
    Edges = 2,
    Corner = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopClosedends {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopClosexpos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopClosexneg {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopCloseypos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopCloseyneg {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopClosezpos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopClosezneg {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopBorder {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldBorder {
    Constant = 0,
    Repeat = 1,
    Streak = 2,
    Extrapolated = 3,
    Mirror = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopDirection {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopTol {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopUsefp16 {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopSlice {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopSlicediv {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopSliceoverlapneg {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopSliceoverlappos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldParmopPositionpath {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopVectorfield {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopVectorfield {
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

    // --- Float parameters ---
    pub fn with_divsize(mut self, val: f32) -> Self {
        self.params.insert(
            "divsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tol(mut self, val: f32) -> Self {
        self.params.insert(
            "tol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tol".to_string(),
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
    pub fn with_direction(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direction".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_uniformdiv(mut self, val: i32) -> Self {
        self.params.insert(
            "uniformdiv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uniformdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slice(mut self, val: i32) -> Self {
        self.params.insert(
            "slice".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_slice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_div(mut self, val: [i32; 3]) -> Self {
        self.params
            .insert("div".to_string(), crate::core::types::ParamValue::Int3(val));
        self
    }
    pub fn with_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slicediv(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "slicediv".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_slicediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slicediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sliceoverlapneg(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "sliceoverlapneg".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_sliceoverlapneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sliceoverlapneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sliceoverlappos(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "sliceoverlappos".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_sliceoverlappos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sliceoverlappos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_twod(mut self, val: DopVectorfieldParmopTwod) -> Self {
        self.params.insert(
            "parmop_twod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_twod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_twod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_voxelplane(mut self, val: DopVectorfieldParmopVoxelplane) -> Self {
        self.params.insert(
            "parmop_voxelplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_voxelplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_voxelplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_voxelplane(mut self, val: DopVectorfieldVoxelplane) -> Self {
        self.params.insert(
            "voxelplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_voxelplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voxelplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_uniformvoxels(mut self, val: DopVectorfieldParmopUniformvoxels) -> Self {
        self.params.insert(
            "parmop_uniformvoxels".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_uniformvoxels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_uniformvoxels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformvoxels(mut self, val: DopVectorfieldUniformvoxels) -> Self {
        self.params.insert(
            "uniformvoxels".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uniformvoxels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformvoxels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_uniformdiv(mut self, val: DopVectorfieldParmopUniformdiv) -> Self {
        self.params.insert(
            "parmop_uniformdiv".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_uniformdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_uniformdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_div(mut self, val: DopVectorfieldParmopDiv) -> Self {
        self.params.insert(
            "parmop_div".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_divsize(mut self, val: DopVectorfieldParmopDivsize) -> Self {
        self.params.insert(
            "parmop_divsize".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_divsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_divsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_size(mut self, val: DopVectorfieldParmopSize) -> Self {
        self.params.insert(
            "parmop_size".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_t(mut self, val: DopVectorfieldParmopT) -> Self {
        self.params.insert(
            "parmop_t".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_voxelsample(mut self, val: DopVectorfieldParmopVoxelsample) -> Self {
        self.params.insert(
            "parmop_voxelsample".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_voxelsample_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_voxelsample".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_voxelsample(mut self, val: DopVectorfieldVoxelsample) -> Self {
        self.params.insert(
            "voxelsample".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_voxelsample_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voxelsample".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_closedends(mut self, val: DopVectorfieldParmopClosedends) -> Self {
        self.params.insert(
            "parmop_closedends".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_closedends_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_closedends".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_closexpos(mut self, val: DopVectorfieldParmopClosexpos) -> Self {
        self.params.insert(
            "parmop_closexpos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_closexpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_closexpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_closexneg(mut self, val: DopVectorfieldParmopClosexneg) -> Self {
        self.params.insert(
            "parmop_closexneg".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_closexneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_closexneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_closeypos(mut self, val: DopVectorfieldParmopCloseypos) -> Self {
        self.params.insert(
            "parmop_closeypos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_closeypos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_closeypos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_closeyneg(mut self, val: DopVectorfieldParmopCloseyneg) -> Self {
        self.params.insert(
            "parmop_closeyneg".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_closeyneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_closeyneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_closezpos(mut self, val: DopVectorfieldParmopClosezpos) -> Self {
        self.params.insert(
            "parmop_closezpos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_closezpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_closezpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_closezneg(mut self, val: DopVectorfieldParmopClosezneg) -> Self {
        self.params.insert(
            "parmop_closezneg".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_closezneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_closezneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_border(mut self, val: DopVectorfieldParmopBorder) -> Self {
        self.params.insert(
            "parmop_border".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_border_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_border".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_border(mut self, val: DopVectorfieldBorder) -> Self {
        self.params.insert(
            "border".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_border_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "border".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_direction(mut self, val: DopVectorfieldParmopDirection) -> Self {
        self.params.insert(
            "parmop_direction".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_direction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_tol(mut self, val: DopVectorfieldParmopTol) -> Self {
        self.params.insert(
            "parmop_tol".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_tol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_tol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_usefp16(mut self, val: DopVectorfieldParmopUsefp16) -> Self {
        self.params.insert(
            "parmop_usefp16".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_usefp16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_usefp16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_slice(mut self, val: DopVectorfieldParmopSlice) -> Self {
        self.params.insert(
            "parmop_slice".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_slice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_slice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_slicediv(mut self, val: DopVectorfieldParmopSlicediv) -> Self {
        self.params.insert(
            "parmop_slicediv".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_slicediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_slicediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_sliceoverlapneg(mut self, val: DopVectorfieldParmopSliceoverlapneg) -> Self {
        self.params.insert(
            "parmop_sliceoverlapneg".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_sliceoverlapneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_sliceoverlapneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_sliceoverlappos(mut self, val: DopVectorfieldParmopSliceoverlappos) -> Self {
        self.params.insert(
            "parmop_sliceoverlappos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_sliceoverlappos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_sliceoverlappos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_positionpath(mut self, val: DopVectorfieldParmopPositionpath) -> Self {
        self.params.insert(
            "parmop_positionpath".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_positionpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_positionpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopVectorfieldDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopVectorfieldSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_positionpath(mut self, val: &str) -> Self {
        self.params.insert(
            "positionpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_positionpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positionpath".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_twod(mut self, val: bool) -> Self {
        self.params.insert(
            "twod".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_twod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closedends(mut self, val: bool) -> Self {
        self.params.insert(
            "closedends".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closedends_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closedends".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closexpos(mut self, val: bool) -> Self {
        self.params.insert(
            "closexpos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closexpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closexpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closexneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closexneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closexneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closexneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closeypos(mut self, val: bool) -> Self {
        self.params.insert(
            "closeypos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closeypos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closeypos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closeyneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closeyneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closeyneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closeyneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closezpos(mut self, val: bool) -> Self {
        self.params.insert(
            "closezpos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closezpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closezpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closezneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closezneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closezneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closezneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usefp16(mut self, val: bool) -> Self {
        self.params.insert(
            "usefp16".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usefp16_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usefp16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVectorfield {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vectorfield"
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
pub enum DopVectorfieldvisualizationGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldvisualizationGuidevistype {
    None = 0,
    Distance = 1,
    Speed = 2,
    Origin = 3,
    Direction = 4,
    Value = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldvisualizationGuidevismode {
    /// Infra-Red
    InfraMinusRed = 0,
    WhiteToRed = 1,
    Grayscale = 2,
    Blackbody = 3,
    /// Two-Tone
    TwoMinusTone = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVectorfieldvisualizationSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopVectorfieldvisualization {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopVectorfieldvisualization {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_guidescale(mut self, val: f32) -> Self {
        self.params.insert(
            "guidescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guidescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidestreamerlen(mut self, val: f32) -> Self {
        self.params.insert(
            "guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guidestreamerlen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidestreamerminspeed(mut self, val: f32) -> Self {
        self.params.insert(
            "guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guidestreamerminspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidevisscale(mut self, val: f32) -> Self {
        self.params.insert(
            "guidevisscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guidevisscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidevisscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
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
    pub fn with_guideplanepos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guideplanepos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guideplanepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideplanepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_guidediv(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "guidediv".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_guidediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_guideplane(mut self, val: DopVectorfieldvisualizationGuideplane) -> Self {
        self.params.insert(
            "guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidevistype(mut self, val: DopVectorfieldvisualizationGuidevistype) -> Self {
        self.params.insert(
            "guidevistype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guidevistype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidevistype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidevismode(mut self, val: DopVectorfieldvisualizationGuidevismode) -> Self {
        self.params.insert(
            "guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopVectorfieldvisualizationSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideusebox(mut self, val: bool) -> Self {
        self.params.insert(
            "guideusebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guideusebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideusebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideuseboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guideuseboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideusesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "guideusesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guideusesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideusesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideoverridediv(mut self, val: bool) -> Self {
        self.params.insert(
            "guideoverridediv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guideoverridediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideoverridediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidebarbs(mut self, val: bool) -> Self {
        self.params.insert(
            "guidebarbs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guidebarbs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidebarbs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidepercomp(mut self, val: bool) -> Self {
        self.params.insert(
            "guidepercomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guidepercomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidepercomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideusestreamers(mut self, val: bool) -> Self {
        self.params.insert(
            "guideusestreamers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guideusestreamers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideusestreamers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVectorfieldvisualization {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vectorfieldvisualization"
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
pub enum DopVelimpulseforceParmopDeltavel {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVelimpulseforceParmopHandlepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVelimpulseforceParmopSamplemode {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVelimpulseforceSamplemode {
    Default = 0,
    Point = 1,
    Circle = 2,
    Sphere = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVelimpulseforceDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVelimpulseforceSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopVelimpulseforce {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopVelimpulseforce {
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

    // --- Float3 parameters ---
    pub fn with_deltavel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "deltavel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_deltavel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deltavel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_handlepos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "handlepos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_handlepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handlepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_deltavel(mut self, val: DopVelimpulseforceParmopDeltavel) -> Self {
        self.params.insert(
            "parmop_deltavel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_deltavel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_deltavel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_handlepos(mut self, val: DopVelimpulseforceParmopHandlepos) -> Self {
        self.params.insert(
            "parmop_handlepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_handlepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_handlepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_samplemode(mut self, val: DopVelimpulseforceParmopSamplemode) -> Self {
        self.params.insert(
            "parmop_samplemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_samplemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_samplemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samplemode(mut self, val: DopVelimpulseforceSamplemode) -> Self {
        self.params.insert(
            "samplemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_samplemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopVelimpulseforceDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopVelimpulseforceSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVelimpulseforce {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "velimpulseforce"
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
pub enum DopVellumconstraintpropertyStiffnessexp {
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
pub enum DopVellumconstraintpropertyCompressstiffnessexp {
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
pub enum DopVellumconstraintpropertyBreaktype {
    None = 0,
    StretchStress = 1,
    BendStress = 2,
    StretchDistance = 3,
    StretchRatio = 4,
    BendAngle = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintpropertyBindinputmenu1 {
    None = 0,
    Sop = 1,
    DopData = 2,
    Myself = 3,
    FirstContextGeometry = 4,
    SecondContextGeometry = 5,
    ThirdContextGeometry = 6,
    FourthContextGeometry = 7,
    /// Myself (No Reads of Outputs)
    MyselfNoReadsOfOutputs = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintpropertyBindinputmenu2 {
    None = 0,
    Sop = 1,
    DopData = 2,
    Myself = 3,
    FirstContextGeometry = 4,
    SecondContextGeometry = 5,
    ThirdContextGeometry = 6,
    FourthContextGeometry = 7,
    /// Myself (No Reads of Outputs)
    MyselfNoReadsOfOutputs = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintpropertyBindinputmenu3 {
    None = 0,
    Sop = 1,
    DopData = 2,
    Myself = 3,
    FirstContextGeometry = 4,
    SecondContextGeometry = 5,
    ThirdContextGeometry = 6,
    FourthContextGeometry = 7,
    /// Myself (No Reads of Outputs)
    MyselfNoReadsOfOutputs = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintpropertyBindinputmenu4 {
    None = 0,
    Sop = 1,
    DopData = 2,
    Myself = 3,
    FirstContextGeometry = 4,
    SecondContextGeometry = 5,
    ThirdContextGeometry = 6,
    FourthContextGeometry = 7,
    /// Myself (No Reads of Outputs)
    MyselfNoReadsOfOutputs = 8,
}

#[derive(Debug, Clone)]
pub struct DopVellumconstraintproperty {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopVellumconstraintproperty {
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

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_stiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "stiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compressstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "compressstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_compressstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compressstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stiffnessdropoff(mut self, val: f32) -> Self {
        self.params.insert(
            "stiffnessdropoff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stiffnessdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stiffnessdropoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velblend(mut self, val: f32) -> Self {
        self.params.insert(
            "velblend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "dampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restlength(mut self, val: f32) -> Self {
        self.params.insert(
            "restlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restscale(mut self, val: f32) -> Self {
        self.params.insert(
            "restscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidingrate(mut self, val: f32) -> Self {
        self.params.insert(
            "slidingrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_slidingrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slidingrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_plasticthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "plasticthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_plasticthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "plasticthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_plasticrate(mut self, val: f32) -> Self {
        self.params.insert(
            "plasticrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_plasticrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "plasticrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_plastichardening(mut self, val: f32) -> Self {
        self.params.insert(
            "plastichardening".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_plastichardening_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "plastichardening".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_breakthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "breakthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_breakthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "breakthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_restvector(mut self, val: [f32; 4]) -> Self {
        self.params.insert(
            "restvector".to_string(),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_restvector_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restvector".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activate(mut self, val: i32) -> Self {
        self.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remove(mut self, val: i32) -> Self {
        self.params.insert(
            "remove".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_remove_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remove".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_broken(mut self, val: i32) -> Self {
        self.params.insert(
            "broken".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_broken_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "broken".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_stiffnessexp(mut self, val: DopVellumconstraintpropertyStiffnessexp) -> Self {
        self.params.insert(
            "stiffnessexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stiffnessexp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stiffnessexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compressstiffnessexp(
        mut self,
        val: DopVellumconstraintpropertyCompressstiffnessexp,
    ) -> Self {
        self.params.insert(
            "compressstiffnessexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_compressstiffnessexp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compressstiffnessexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_breaktype(mut self, val: DopVellumconstraintpropertyBreaktype) -> Self {
        self.params.insert(
            "breaktype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_breaktype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "breaktype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindinputmenu1(mut self, val: DopVellumconstraintpropertyBindinputmenu1) -> Self {
        self.params.insert(
            "bindinputmenu1".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindinputmenu1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindinputmenu1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindinputmenu2(mut self, val: DopVellumconstraintpropertyBindinputmenu2) -> Self {
        self.params.insert(
            "bindinputmenu2".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindinputmenu2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindinputmenu2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindinputmenu3(mut self, val: DopVellumconstraintpropertyBindinputmenu3) -> Self {
        self.params.insert(
            "bindinputmenu3".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindinputmenu3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindinputmenu3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindinputmenu4(mut self, val: DopVellumconstraintpropertyBindinputmenu4) -> Self {
        self.params.insert(
            "bindinputmenu4".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindinputmenu4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindinputmenu4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_bindgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindgeo(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgeo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_cwdpath(mut self, val: &str) -> Self {
        self.params.insert(
            "vex_cwdpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_cwdpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_cwdpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput1(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgeoinput1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgeoinput1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_binddopinput1(mut self, val: &str) -> Self {
        self.params.insert(
            "binddopinput1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_binddopinput1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "binddopinput1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput2(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgeoinput2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgeoinput2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_binddopinput2(mut self, val: &str) -> Self {
        self.params.insert(
            "binddopinput2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_binddopinput2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "binddopinput2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput3(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgeoinput3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgeoinput3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_binddopinput3(mut self, val: &str) -> Self {
        self.params.insert(
            "binddopinput3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_binddopinput3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "binddopinput3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput4(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgeoinput4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgeoinput4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_binddopinput4(mut self, val: &str) -> Self {
        self.params.insert(
            "binddopinput4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_binddopinput4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "binddopinput4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localexpression(mut self, val: &str) -> Self {
        self.params.insert(
            "localexpression".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_localexpression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localexpression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usegroup(mut self, val: bool) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dostiffness(mut self, val: bool) -> Self {
        self.params.insert(
            "dostiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dostiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dostiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docompressstiffness(mut self, val: bool) -> Self {
        self.params.insert(
            "docompressstiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docompressstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docompressstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dostiffnessdropoff(mut self, val: bool) -> Self {
        self.params.insert(
            "dostiffnessdropoff".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dostiffnessdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dostiffnessdropoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dovelblend(mut self, val: bool) -> Self {
        self.params.insert(
            "dovelblend".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dovelblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dovelblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodampingratio(mut self, val: bool) -> Self {
        self.params.insert(
            "dodampingratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dorestlength(mut self, val: bool) -> Self {
        self.params.insert(
            "dorestlength".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorestlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dorestlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dorestscale(mut self, val: bool) -> Self {
        self.params.insert(
            "dorestscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorestscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dorestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dorestvector(mut self, val: bool) -> Self {
        self.params.insert(
            "dorestvector".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorestvector_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dorestvector".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doslidingrate(mut self, val: bool) -> Self {
        self.params.insert(
            "doslidingrate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doslidingrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doslidingrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doplasticthreshold(mut self, val: bool) -> Self {
        self.params.insert(
            "doplasticthreshold".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doplasticthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doplasticthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doplasticrate(mut self, val: bool) -> Self {
        self.params.insert(
            "doplasticrate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doplasticrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doplasticrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doplastichardening(mut self, val: bool) -> Self {
        self.params.insert(
            "doplastichardening".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doplastichardening_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doplastichardening".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dobreakthreshold(mut self, val: bool) -> Self {
        self.params.insert(
            "dobreakthreshold".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dobreakthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dobreakthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dobreaktype(mut self, val: bool) -> Self {
        self.params.insert(
            "dobreaktype".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dobreaktype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dobreaktype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doremove(mut self, val: bool) -> Self {
        self.params.insert(
            "doremove".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doremove_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doremove".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dobroken(mut self, val: bool) -> Self {
        self.params.insert(
            "dobroken".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dobroken_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dobroken".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uselocal(mut self, val: bool) -> Self {
        self.params.insert(
            "uselocal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselocal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uselocal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVellumconstraintproperty {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vellumconstraintproperty"
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
pub enum DopVellumconstraintsCreatemode {
    OnCreationFrame = 0,
    EachFrame = 1,
    EachSubstep = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsConstrainttype {
    None = 0,
    DistanceAlongEdges = 1,
    BendAcrossTriangles = 2,
    Cloth = 3,
    Hair = 4,
    String = 5,
    PinToTarget = 6,
    AttachToGeometry = 7,
    StitchPoints = 8,
    Pressure = 9,
    TetrahedralVolume = 10,
    WeldPoints = 11,
    Glue = 12,
    Struts = 13,
    TetrahedralFiber = 14,
    TriangleStretch = 15,
    TetrahedralStretch = 16,
    ShapeMatch = 17,
    SurfaceStruts = 18,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsStretchtype {
    DistanceAlongEdges = 0,
    TriangleStretch = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsBendtype {
    Angle = 0,
    Distance = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsLinear {
    /// Non-Linear ARAP
    NonMinusLinearArap = 0,
    LinearArap = 1,
    /// Scale-Invariant ARAP
    ScaleMinusInvariantArap = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsGrouptype {
    Primitives = 0,
    Points = 1,
    Edges = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsTriangulation {
    None = 0,
    Regular = 1,
    Alternating = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsDomass {
    Unchanged = 0,
    SetUniform = 1,
    CalculateUniform = 2,
    CalculateVarying = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsScaledensitymode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsDothickness {
    Unchanged = 0,
    SetUniform = 1,
    CalculateUniform = 2,
    CalculateVarying = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsScalethicknessmode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsPiecemode {
    FromAttribute = 0,
    FromConnectivity = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsTargetgrouptype {
    Primitives = 0,
    Points = 1,
    Edges = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsPintype {
    Permanent = 0,
    Stopped = 1,
    Soft = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsPinrotation {
    None = 0,
    SameAsPosition = 1,
    Soft = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsSlidingscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsGlueSearchpref {
    Nearest = 0,
    Farthest = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsStretchstiffnessexp {
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
pub enum DopVellumconstraintsStretchstiffnessscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsStretchdampingscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsCompressstiffnessexp {
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
pub enum DopVellumconstraintsCompressstiffnessscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsTangentstiffnessexp {
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
pub enum DopVellumconstraintsStretchstiffnessdropoffdir {
    Increasing = 0,
    Decreasing = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsStretchvelblendscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsStretchplasticthresholdscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsStretchplasticratescalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsStretchplastichardeningscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsBendstiffnessexp {
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
pub enum DopVellumconstraintsBendstiffnessscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsBenddampingscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsBendstiffnessdropoffdir {
    Increasing = 0,
    Decreasing = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsBreakscalemode {
    NoScaling = 0,
    ScaleByAttribute = 1,
    ScaleByValue = 2,
    ScaleByBoth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsBreaktypeall {
    None = 0,
    StretchStress = 1,
    BendStress = 2,
    StretchDistance = 3,
    StretchRatio = 4,
    BendAngle = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsBreaktypestretch {
    None = 0,
    StretchStress = 1,
    StretchDistance = 2,
    StretchRatio = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumconstraintsAttribpromote {
    Maximum = 0,
    Minimum = 1,
    Average = 2,
    Multiply = 3,
    UseSource = 4,
    UseTarget = 5,
}

#[derive(Debug, Clone)]
pub struct DopVellumconstraints {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopVellumconstraints {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_createframe(mut self, val: f32) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_createframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mass(mut self, val: f32) -> Self {
        self.params.insert(
            "mass".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thickness(mut self, val: f32) -> Self {
        self.params.insert(
            "thickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thicknessscale(mut self, val: f32) -> Self {
        self.params.insert(
            "thicknessscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thicknessscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknessscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dragnormal(mut self, val: f32) -> Self {
        self.params.insert(
            "dragnormal".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dragnormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dragnormal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dragtangent(mut self, val: f32) -> Self {
        self.params.insert(
            "dragtangent".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dragtangent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dragtangent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxdist(mut self, val: f32) -> Self {
        self.params.insert(
            "maxdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidingrate(mut self, val: f32) -> Self {
        self.params.insert(
            "slidingrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_slidingrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slidingrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glue_minradius(mut self, val: f32) -> Self {
        self.params.insert(
            "glue_minradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glue_minradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glue_minradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glue_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "glue_radius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glue_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glue_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glue_detach_chance(mut self, val: f32) -> Self {
        self.params.insert(
            "glue_detach_chance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glue_detach_chance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glue_detach_chance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glue_point_chance(mut self, val: f32) -> Self {
        self.params.insert(
            "glue_point_chance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glue_point_chance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glue_point_chance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glue_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "glue_seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glue_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glue_seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strut_maxlen(mut self, val: f32) -> Self {
        self.params.insert(
            "strut_maxlen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strut_maxlen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strut_maxlen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strut_jitter(mut self, val: f32) -> Self {
        self.params.insert(
            "strut_jitter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strut_jitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strut_jitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strut_point_chance(mut self, val: f32) -> Self {
        self.params.insert(
            "strut_point_chance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strut_point_chance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strut_point_chance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strut_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "strut_seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strut_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strut_seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strut_rayoff(mut self, val: f32) -> Self {
        self.params.insert(
            "strut_rayoff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strut_rayoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strut_rayoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchstiffnessscale(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchstiffnessscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchstiffnessscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchstiffnessscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchdampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchdampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchdampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchdampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchdampingscale(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchdampingscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchdampingscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchdampingscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchrestscale(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchrestscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchrestscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchrestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compressstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "compressstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_compressstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compressstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compressstiffnessscale(mut self, val: f32) -> Self {
        self.params.insert(
            "compressstiffnessscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_compressstiffnessscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compressstiffnessscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tangentstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "tangentstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tangentstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tangentstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchstiffnessdropoff(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchstiffnessdropoff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchstiffnessdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchstiffnessdropoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchstiffnessdropoffmin(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchstiffnessdropoffmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchstiffnessdropoffmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchstiffnessdropoffmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchwarpscale(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchwarpscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchwarpscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchwarpscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchweftscale(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchweftscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchweftscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchweftscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchshearscale(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchshearscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchshearscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchshearscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchvelblend(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchvelblend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchvelblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchvelblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchplasticthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchplasticthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchplasticthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchplasticthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchplasticthresholdscale(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchplasticthresholdscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchplasticthresholdscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchplasticthresholdscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchplasticrate(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchplasticrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchplasticrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchplasticrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchplasticratescale(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchplasticratescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchplasticratescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchplasticratescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchplastichardening(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchplastichardening".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchplastichardening_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchplastichardening".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchplastichardeningscale(mut self, val: f32) -> Self {
        self.params.insert(
            "stretchplastichardeningscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stretchplastichardeningscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchplastichardeningscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "bendstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bendstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendstiffnessscale(mut self, val: f32) -> Self {
        self.params.insert(
            "bendstiffnessscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bendstiffnessscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendstiffnessscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_benddampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "benddampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_benddampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "benddampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_benddampingscale(mut self, val: f32) -> Self {
        self.params.insert(
            "benddampingscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_benddampingscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "benddampingscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendrestscale(mut self, val: f32) -> Self {
        self.params.insert(
            "bendrestscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bendrestscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendrestscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendstiffnessdropoff(mut self, val: f32) -> Self {
        self.params.insert(
            "bendstiffnessdropoff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bendstiffnessdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendstiffnessdropoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendstiffnessdropoffmin(mut self, val: f32) -> Self {
        self.params.insert(
            "bendstiffnessdropoffmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bendstiffnessdropoffmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendstiffnessdropoffmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxbranchangle(mut self, val: f32) -> Self {
        self.params.insert(
            "maxbranchangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxbranchangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxbranchangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendplasticthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "bendplasticthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bendplasticthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendplasticthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendplasticrate(mut self, val: f32) -> Self {
        self.params.insert(
            "bendplasticrate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bendplasticrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendplasticrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendplastichardening(mut self, val: f32) -> Self {
        self.params.insert(
            "bendplastichardening".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bendplastichardening_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendplastichardening".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_breakthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "breakthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_breakthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "breakthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_breakscale(mut self, val: f32) -> Self {
        self.params.insert(
            "breakscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_breakscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "breakscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activate(mut self, val: i32) -> Self {
        self.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_neighbordist(mut self, val: i32) -> Self {
        self.params.insert(
            "neighbordist".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_neighbordist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "neighbordist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_layer(mut self, val: i32) -> Self {
        self.params.insert(
            "layer".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_layer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "layer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glue_numpt(mut self, val: i32) -> Self {
        self.params.insert(
            "glue_numpt".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_glue_numpt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glue_numpt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glue_constraintsperpt(mut self, val: i32) -> Self {
        self.params.insert(
            "glue_constraintsperpt".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_glue_constraintsperpt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glue_constraintsperpt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strut_constraintsperpt(mut self, val: i32) -> Self {
        self.params.insert(
            "strut_constraintsperpt".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_strut_constraintsperpt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strut_constraintsperpt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_createmode(mut self, val: DopVellumconstraintsCreatemode) -> Self {
        self.params.insert(
            "createmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainttype(mut self, val: DopVellumconstraintsConstrainttype) -> Self {
        self.params.insert(
            "constrainttype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_constrainttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainttype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchtype(mut self, val: DopVellumconstraintsStretchtype) -> Self {
        self.params.insert(
            "stretchtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stretchtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendtype(mut self, val: DopVellumconstraintsBendtype) -> Self {
        self.params.insert(
            "bendtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bendtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_linear(mut self, val: DopVellumconstraintsLinear) -> Self {
        self.params.insert(
            "linear".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_linear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "linear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_grouptype(mut self, val: DopVellumconstraintsGrouptype) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_triangulation(mut self, val: DopVellumconstraintsTriangulation) -> Self {
        self.params.insert(
            "triangulation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_triangulation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triangulation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domass(mut self, val: DopVellumconstraintsDomass) -> Self {
        self.params.insert(
            "domass".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_domass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scaledensitymode(mut self, val: DopVellumconstraintsScaledensitymode) -> Self {
        self.params.insert(
            "scaledensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_scaledensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scaledensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dothickness(mut self, val: DopVellumconstraintsDothickness) -> Self {
        self.params.insert(
            "dothickness".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dothickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dothickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalethicknessmode(mut self, val: DopVellumconstraintsScalethicknessmode) -> Self {
        self.params.insert(
            "scalethicknessmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_scalethicknessmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalethicknessmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_piecemode(mut self, val: DopVellumconstraintsPiecemode) -> Self {
        self.params.insert(
            "piecemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_piecemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "piecemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetgrouptype(mut self, val: DopVellumconstraintsTargetgrouptype) -> Self {
        self.params.insert(
            "targetgrouptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_targetgrouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetgrouptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pintype(mut self, val: DopVellumconstraintsPintype) -> Self {
        self.params.insert(
            "pintype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pintype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pintype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pinrotation(mut self, val: DopVellumconstraintsPinrotation) -> Self {
        self.params.insert(
            "pinrotation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pinrotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pinrotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidingscalemode(mut self, val: DopVellumconstraintsSlidingscalemode) -> Self {
        self.params.insert(
            "slidingscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_slidingscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slidingscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glue_searchpref(mut self, val: DopVellumconstraintsGlueSearchpref) -> Self {
        self.params.insert(
            "glue_searchpref".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_glue_searchpref_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glue_searchpref".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchstiffnessexp(
        mut self,
        val: DopVellumconstraintsStretchstiffnessexp,
    ) -> Self {
        self.params.insert(
            "stretchstiffnessexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stretchstiffnessexp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchstiffnessexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchstiffnessscalemode(
        mut self,
        val: DopVellumconstraintsStretchstiffnessscalemode,
    ) -> Self {
        self.params.insert(
            "stretchstiffnessscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stretchstiffnessscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchstiffnessscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchdampingscalemode(
        mut self,
        val: DopVellumconstraintsStretchdampingscalemode,
    ) -> Self {
        self.params.insert(
            "stretchdampingscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stretchdampingscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchdampingscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compressstiffnessexp(
        mut self,
        val: DopVellumconstraintsCompressstiffnessexp,
    ) -> Self {
        self.params.insert(
            "compressstiffnessexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_compressstiffnessexp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compressstiffnessexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compressstiffnessscalemode(
        mut self,
        val: DopVellumconstraintsCompressstiffnessscalemode,
    ) -> Self {
        self.params.insert(
            "compressstiffnessscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_compressstiffnessscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compressstiffnessscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tangentstiffnessexp(
        mut self,
        val: DopVellumconstraintsTangentstiffnessexp,
    ) -> Self {
        self.params.insert(
            "tangentstiffnessexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tangentstiffnessexp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tangentstiffnessexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchstiffnessdropoffdir(
        mut self,
        val: DopVellumconstraintsStretchstiffnessdropoffdir,
    ) -> Self {
        self.params.insert(
            "stretchstiffnessdropoffdir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stretchstiffnessdropoffdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchstiffnessdropoffdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchvelblendscalemode(
        mut self,
        val: DopVellumconstraintsStretchvelblendscalemode,
    ) -> Self {
        self.params.insert(
            "stretchvelblendscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stretchvelblendscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchvelblendscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchplasticthresholdscalemode(
        mut self,
        val: DopVellumconstraintsStretchplasticthresholdscalemode,
    ) -> Self {
        self.params.insert(
            "stretchplasticthresholdscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stretchplasticthresholdscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchplasticthresholdscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchplasticratescalemode(
        mut self,
        val: DopVellumconstraintsStretchplasticratescalemode,
    ) -> Self {
        self.params.insert(
            "stretchplasticratescalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stretchplasticratescalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchplasticratescalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchplastichardeningscalemode(
        mut self,
        val: DopVellumconstraintsStretchplastichardeningscalemode,
    ) -> Self {
        self.params.insert(
            "stretchplastichardeningscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stretchplastichardeningscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchplastichardeningscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendstiffnessexp(mut self, val: DopVellumconstraintsBendstiffnessexp) -> Self {
        self.params.insert(
            "bendstiffnessexp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bendstiffnessexp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendstiffnessexp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendstiffnessscalemode(
        mut self,
        val: DopVellumconstraintsBendstiffnessscalemode,
    ) -> Self {
        self.params.insert(
            "bendstiffnessscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bendstiffnessscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendstiffnessscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_benddampingscalemode(
        mut self,
        val: DopVellumconstraintsBenddampingscalemode,
    ) -> Self {
        self.params.insert(
            "benddampingscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_benddampingscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "benddampingscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendstiffnessdropoffdir(
        mut self,
        val: DopVellumconstraintsBendstiffnessdropoffdir,
    ) -> Self {
        self.params.insert(
            "bendstiffnessdropoffdir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bendstiffnessdropoffdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendstiffnessdropoffdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_breakscalemode(mut self, val: DopVellumconstraintsBreakscalemode) -> Self {
        self.params.insert(
            "breakscalemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_breakscalemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "breakscalemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_breaktypeall(mut self, val: DopVellumconstraintsBreaktypeall) -> Self {
        self.params.insert(
            "breaktypeall".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_breaktypeall_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "breaktypeall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_breaktypestretch(mut self, val: DopVellumconstraintsBreaktypestretch) -> Self {
        self.params.insert(
            "breaktypestretch".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_breaktypestretch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "breaktypestretch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribpromote(mut self, val: DopVellumconstraintsAttribpromote) -> Self {
        self.params.insert(
            "attribpromote".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_attribpromote_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribpromote".to_string(),
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
    pub fn with_scaledensityattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "scaledensityattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scaledensityattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scaledensityattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalethicknessattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "scalethicknessattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scalethicknessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalethicknessattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pieceattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pieceattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pieceattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pieceattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "targetgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetpath(mut self, val: &str) -> Self {
        self.params.insert(
            "targetpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pingroup(mut self, val: &str) -> Self {
        self.params.insert(
            "pingroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pingroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pingroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidingattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "slidingattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_slidingattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slidingattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glue_clusterattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "glue_clusterattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_glue_clusterattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glue_clusterattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strut_dirattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "strut_dirattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_strut_dirattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strut_dirattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchstiffnessattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "stretchstiffnessattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stretchstiffnessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchstiffnessattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchdampingattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "stretchdampingattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stretchdampingattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchdampingattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compressstiffnessattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "compressstiffnessattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_compressstiffnessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compressstiffnessattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_materialuv(mut self, val: &str) -> Self {
        self.params.insert(
            "materialuv".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_materialuv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "materialuv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchvelblendattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "stretchvelblendattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stretchvelblendattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchvelblendattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchplasticthresholdattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "stretchplasticthresholdattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stretchplasticthresholdattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchplasticthresholdattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchplasticrateattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "stretchplasticrateattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stretchplasticrateattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchplasticrateattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchplastichardeningattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "stretchplastichardeningattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stretchplastichardeningattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchplastichardeningattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchgrp(mut self, val: &str) -> Self {
        self.params.insert(
            "stretchgrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stretchgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendstiffnessattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "bendstiffnessattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bendstiffnessattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendstiffnessattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_benddampingattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "benddampingattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_benddampingattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "benddampingattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendplasticthresholdattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "bendplasticthresholdattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bendplasticthresholdattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendplasticthresholdattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendplasticrateattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "bendplasticrateattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bendplasticrateattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendplasticrateattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendplastichardeningattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "bendplastichardeningattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bendplastichardeningattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendplastichardeningattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendgrp(mut self, val: &str) -> Self {
        self.params.insert(
            "bendgrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bendgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_breakscaleattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "breakscaleattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_breakscaleattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "breakscaleattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tag(mut self, val: &str) -> Self {
        self.params.insert(
            "tag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geometrydata(mut self, val: &str) -> Self {
        self.params.insert(
            "geometrydata".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geometrydata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geometrydata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraintdata(mut self, val: &str) -> Self {
        self.params.insert(
            "constraintdata".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraintdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraintdata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_preservevol(mut self, val: bool) -> Self {
        self.params.insert(
            "preservevol".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preservevol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preservevol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dolayer(mut self, val: bool) -> Self {
        self.params.insert(
            "dolayer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dolayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dolayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchanimation(mut self, val: bool) -> Self {
        self.params.insert(
            "matchanimation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_matchanimation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchanimation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useclosestpt(mut self, val: bool) -> Self {
        self.params.insert(
            "useclosestpt".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useclosestpt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useclosestpt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useclosestprim(mut self, val: bool) -> Self {
        self.params.insert(
            "useclosestprim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useclosestprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useclosestprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxdistcheck(mut self, val: bool) -> Self {
        self.params.insert(
            "maxdistcheck".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_maxdistcheck_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxdistcheck".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosliding(mut self, val: bool) -> Self {
        self.params.insert(
            "dosliding".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosliding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dosliding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_glue_usecluster(mut self, val: bool) -> Self {
        self.params.insert(
            "glue_usecluster".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_glue_usecluster_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glue_usecluster".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strut_usedirattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "strut_usedirattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_strut_usedirattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strut_usedirattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strut_invertnormal(mut self, val: bool) -> Self {
        self.params.insert(
            "strut_invertnormal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_strut_invertnormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strut_invertnormal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strut_testnormals(mut self, val: bool) -> Self {
        self.params.insert(
            "strut_testnormals".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_strut_testnormals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strut_testnormals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docompress(mut self, val: bool) -> Self {
        self.params.insert(
            "docompress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docompress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docompress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dotangent(mut self, val: bool) -> Self {
        self.params.insert(
            "dotangent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dotangent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dotangent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dostretchstiffnessdropoff(mut self, val: bool) -> Self {
        self.params.insert(
            "dostretchstiffnessdropoff".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dostretchstiffnessdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dostretchstiffnessdropoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dostretchstiffnessdropoffmin(mut self, val: bool) -> Self {
        self.params.insert(
            "dostretchstiffnessdropoffmin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dostretchstiffnessdropoffmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dostretchstiffnessdropoffmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchanisotropy(mut self, val: bool) -> Self {
        self.params.insert(
            "stretchanisotropy".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stretchanisotropy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchanisotropy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dostretchvelblend(mut self, val: bool) -> Self {
        self.params.insert(
            "dostretchvelblend".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dostretchvelblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dostretchvelblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchplasticity(mut self, val: bool) -> Self {
        self.params.insert(
            "stretchplasticity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stretchplasticity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchplasticity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stretchplasticthresholdratio(mut self, val: bool) -> Self {
        self.params.insert(
            "stretchplasticthresholdratio".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stretchplasticthresholdratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stretchplasticthresholdratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dostretchgrp(mut self, val: bool) -> Self {
        self.params.insert(
            "dostretchgrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dostretchgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dostretchgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepuniquestretch(mut self, val: bool) -> Self {
        self.params.insert(
            "keepuniquestretch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepuniquestretch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepuniquestretch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendweld(mut self, val: bool) -> Self {
        self.params.insert(
            "bendweld".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bendweld_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendweld".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendcopystiffness(mut self, val: bool) -> Self {
        self.params.insert(
            "bendcopystiffness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bendcopystiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendcopystiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dobendstiffnessdropoff(mut self, val: bool) -> Self {
        self.params.insert(
            "dobendstiffnessdropoff".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dobendstiffnessdropoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dobendstiffnessdropoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dobendstiffnessdropoffmin(mut self, val: bool) -> Self {
        self.params.insert(
            "dobendstiffnessdropoffmin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dobendstiffnessdropoffmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dobendstiffnessdropoffmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domaxbranchangle(mut self, val: bool) -> Self {
        self.params.insert(
            "domaxbranchangle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domaxbranchangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domaxbranchangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bendplasticity(mut self, val: bool) -> Self {
        self.params.insert(
            "bendplasticity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bendplasticity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bendplasticity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dobendgrp(mut self, val: bool) -> Self {
        self.params.insert(
            "dobendgrp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dobendgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dobendgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepuniquebend(mut self, val: bool) -> Self {
        self.params.insert(
            "keepuniquebend".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepuniquebend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepuniquebend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dobreaking(mut self, val: bool) -> Self {
        self.params.insert(
            "dobreaking".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dobreaking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dobreaking".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVellumconstraints {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vellumconstraints"
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
pub enum DopVellumobjectVisMode {
    None = 0,
    StretchStress = 1,
    BendStress = 2,
    StretchDistance = 3,
    StretchRatio = 4,
    BendAngle = 5,
    StretchPlasticFlow = 6,
    BendPlasticFlow = 7,
    VolumeStress = 8,
    VolumeDistance = 9,
    VolumeRatio = 10,
}

#[derive(Debug, Clone)]
pub struct DopVellumobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopVellumobject {
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
    pub fn with_guide_colvisscale(mut self, val: f32) -> Self {
        self.params.insert(
            "guide_colvisscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guide_colvisscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_colvisscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_constraintvisradius(mut self, val: f32) -> Self {
        self.params.insert(
            "guide_constraintvisradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guide_constraintvisradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_constraintvisradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_maxstretchstress(mut self, val: f32) -> Self {
        self.params.insert(
            "vis_maxstretchstress".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vis_maxstretchstress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_maxstretchstress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_maxbendstress(mut self, val: f32) -> Self {
        self.params.insert(
            "vis_maxbendstress".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vis_maxbendstress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_maxbendstress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_maxstretchdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "vis_maxstretchdistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vis_maxstretchdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_maxstretchdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_maxstretchratio(mut self, val: f32) -> Self {
        self.params.insert(
            "vis_maxstretchratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vis_maxstretchratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_maxstretchratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_maxbendangle(mut self, val: f32) -> Self {
        self.params.insert(
            "vis_maxbendangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vis_maxbendangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_maxbendangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_maxstretchplasticflow(mut self, val: f32) -> Self {
        self.params.insert(
            "vis_maxstretchplasticflow".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vis_maxstretchplasticflow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_maxstretchplasticflow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_maxbendplasticflow(mut self, val: f32) -> Self {
        self.params.insert(
            "vis_maxbendplasticflow".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vis_maxbendplasticflow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_maxbendplasticflow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_maxvolumestress(mut self, val: f32) -> Self {
        self.params.insert(
            "vis_maxvolumestress".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vis_maxvolumestress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_maxvolumestress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_maxvolumedistance(mut self, val: f32) -> Self {
        self.params.insert(
            "vis_maxvolumedistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vis_maxvolumedistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_maxvolumedistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_maxvolumeratio(mut self, val: f32) -> Self {
        self.params.insert(
            "vis_maxvolumeratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vis_maxvolumeratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_maxvolumeratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_guide_thicknesscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_thicknesscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_thicknesscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_thicknesscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_failedself(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_failedself".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_failedself_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_failedself".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_failedexternal(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_failedexternal".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_failedexternal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_failedexternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_weldcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_weldcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_weldcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_weldcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_pincolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_pincolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_pincolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_pincolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_stitchcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_stitchcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_stitchcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_stitchcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_attachcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_attachcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_attachcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_attachcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_createframe(mut self, val: i32) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_createframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_vis_mode(mut self, val: DopVellumobjectVisMode) -> Self {
        self.params.insert(
            "vis_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vis_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_objname(mut self, val: &str) -> Self {
        self.params.insert(
            "objname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displaysoppath(mut self, val: &str) -> Self {
        self.params.insert(
            "displaysoppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_displaysoppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displaysoppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraintsoppath(mut self, val: &str) -> Self {
        self.params.insert(
            "constraintsoppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraintsoppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraintsoppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_constraintgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "guide_constraintgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guide_constraintgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_constraintgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usesimframe(mut self, val: bool) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesimframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvefirstframe(mut self, val: bool) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvefirstframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allowcaching(mut self, val: bool) -> Self {
        self.params.insert(
            "allowcaching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowcaching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowcaching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displaygeo(mut self, val: bool) -> Self {
        self.params.insert(
            "displaygeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displaygeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displaygeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_displayconstraints(mut self, val: bool) -> Self {
        self.params.insert(
            "displayconstraints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayconstraints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayconstraints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_wireframe(mut self, val: bool) -> Self {
        self.params.insert(
            "guide_wireframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_wireframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_wireframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_showthickness(mut self, val: bool) -> Self {
        self.params.insert(
            "guide_showthickness".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_showthickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_showthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_showthicknessextrude(mut self, val: bool) -> Self {
        self.params.insert(
            "guide_showthicknessextrude".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_showthicknessextrude_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_showthicknessextrude".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_showfailedself(mut self, val: bool) -> Self {
        self.params.insert(
            "guide_showfailedself".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_showfailedself_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_showfailedself".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_showfailedexternal(mut self, val: bool) -> Self {
        self.params.insert(
            "guide_showfailedexternal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_showfailedexternal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_showfailedexternal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_showwelds(mut self, val: bool) -> Self {
        self.params.insert(
            "guide_showwelds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_showwelds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_showwelds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_usegroup(mut self, val: bool) -> Self {
        self.params.insert(
            "guide_usegroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_usegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_usegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_showpin(mut self, val: bool) -> Self {
        self.params.insert(
            "guide_showpin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_showpin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_showpin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_showstitch(mut self, val: bool) -> Self {
        self.params.insert(
            "guide_showstitch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_showstitch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_showstitch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guide_showattach(mut self, val: bool) -> Self {
        self.params.insert(
            "guide_showattach".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_showattach_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_showattach".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vis_displaygeo(mut self, val: bool) -> Self {
        self.params.insert(
            "vis_displaygeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vis_displaygeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vis_displaygeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVellumobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vellumobject"
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
pub enum DopVellumrestblendUpdatemode {
    SingleFrame = 0,
    EachFrame = 1,
    EachSubstep = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumrestblendRestsource {
    SimulationGeometry = 0,
    Sop = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumrestblendBlendmode {
    Blend = 0,
    Distance = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumrestblendMaskmode {
    None = 0,
    SetFromAttribute = 1,
    ScaleFromAttribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumrestblendAttribpromote {
    Maximum = 0,
    Minimum = 1,
    Average = 2,
    UseSource = 3,
    UseTarget = 4,
}

#[derive(Debug, Clone)]
pub struct DopVellumrestblend {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopVellumrestblend {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_activationframe(mut self, val: f32) -> Self {
        self.params.insert(
            "activationframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activationframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distance(mut self, val: f32) -> Self {
        self.params.insert(
            "distance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timescale(mut self, val: f32) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activate(mut self, val: i32) -> Self {
        self.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_updatemode(mut self, val: DopVellumrestblendUpdatemode) -> Self {
        self.params.insert(
            "updatemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_updatemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updatemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restsource(mut self, val: DopVellumrestblendRestsource) -> Self {
        self.params.insert(
            "restsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_restsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendmode(mut self, val: DopVellumrestblendBlendmode) -> Self {
        self.params.insert(
            "blendmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blendmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskmode(mut self, val: DopVellumrestblendMaskmode) -> Self {
        self.params.insert(
            "maskmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maskmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribpromote(mut self, val: DopVellumrestblendAttribpromote) -> Self {
        self.params.insert(
            "attribpromote".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_attribpromote_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribpromote".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_partgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_congroup(mut self, val: &str) -> Self {
        self.params.insert(
            "congroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_congroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "congroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restsop(mut self, val: &str) -> Self {
        self.params.insert(
            "restsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restsop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maskattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "maskattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maskattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maskattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geometrydata(mut self, val: &str) -> Self {
        self.params.insert(
            "geometrydata".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geometrydata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geometrydata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraintdata(mut self, val: &str) -> Self {
        self.params.insert(
            "constraintdata".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraintdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraintdata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usegroup(mut self, val: bool) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetimestep(mut self, val: bool) -> Self {
        self.params.insert(
            "usetimestep".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetimestep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetimestep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVellumrestblend {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vellumrestblend"
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
pub enum DopVellumsolverIntegration {
    FirstOrder = 0,
    SecondOrder = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumsolverBreakfrequency {
    Never = 0,
    PerFrame = 1,
    PerSubstep = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumsolverSlidingmethod {
    ClosestPoint = 0,
    TraversePolygons = 1,
    /// Traverse Triangles (Optimized)
    TraverseTrianglesOptimized = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumsolverViscositysolver {
    Explicit = 0,
    Implicit = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumsolverForcefrequency {
    PerFrame = 0,
    PerSubstep = 1,
}

#[derive(Debug, Clone)]
pub struct DopVellumsolver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopVellumsolver {
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

    /// Connects to input 0: "Objects to Solve"
    pub fn set_input_objects_to_solve<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to Solve" and specifies the output index of the target node.
    pub fn set_input_objects_to_solve_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Particle Forces"
    pub fn set_input_particle_forces<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Particle Forces" and specifies the output index of the target node.
    pub fn set_input_particle_forces_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Post-Solve"
    pub fn set_input_post_solve<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Post-Solve" and specifies the output index of the target node.
    pub fn set_input_post_solve_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_timescale(mut self, val: f32) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_veldamping(mut self, val: f32) -> Self {
        self.params.insert(
            "veldamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_veldamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "veldamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_layershock(mut self, val: f32) -> Self {
        self.params.insert(
            "layershock".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_layershock_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "layershock".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_static_threshold(mut self, val: f32) -> Self {
        self.params.insert(
            "static_threshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_static_threshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "static_threshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dynamic_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "dynamic_scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dynamic_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dynamic_scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_friction(mut self, val: f32) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_selffriction(mut self, val: f32) -> Self {
        self.params.insert(
            "selffriction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_selffriction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "selffriction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_static_sdfscale(mut self, val: f32) -> Self {
        self.params.insert(
            "static_sdfscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_static_sdfscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "static_sdfscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dynamic_sdfscale(mut self, val: f32) -> Self {
        self.params.insert(
            "dynamic_sdfscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dynamic_sdfscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dynamic_sdfscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mass(mut self, val: f32) -> Self {
        self.params.insert(
            "mass".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thickness(mut self, val: f32) -> Self {
        self.params.insert(
            "thickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_secondaryfrequency(mut self, val: f32) -> Self {
        self.params.insert(
            "secondaryfrequency".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_secondaryfrequency_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "secondaryfrequency".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disablestretchratio(mut self, val: f32) -> Self {
        self.params.insert(
            "disablestretchratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_disablestretchratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disablestretchratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxaccel(mut self, val: f32) -> Self {
        self.params.insert(
            "maxaccel".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxaccel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxaccel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sleep_velocitythreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "sleep_velocitythreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sleep_velocitythreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sleep_velocitythreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sleep_delay(mut self, val: f32) -> Self {
        self.params.insert(
            "sleep_delay".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sleep_delay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sleep_delay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_grain_searchscale(mut self, val: f32) -> Self {
        self.params.insert(
            "grain_searchscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_grain_searchscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grain_searchscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_grain_weight(mut self, val: f32) -> Self {
        self.params.insert(
            "grain_weight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_grain_weight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grain_weight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repulsionweight(mut self, val: f32) -> Self {
        self.params.insert(
            "repulsionweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_repulsionweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repulsionweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repulsionstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "repulsionstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_repulsionstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repulsionstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attractionweight(mut self, val: f32) -> Self {
        self.params.insert(
            "attractionweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attractionweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attractionweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attractionstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "attractionstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_attractionstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attractionstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_massshockpower(mut self, val: f32) -> Self {
        self.params.insert(
            "massshockpower".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_massshockpower_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "massshockpower".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fluid_searchscale(mut self, val: f32) -> Self {
        self.params.insert(
            "fluid_searchscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fluid_searchscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fluid_searchscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionviscosity(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionviscosity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionviscosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionviscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tol(mut self, val: f32) -> Self {
        self.params.insert(
            "tol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacetension(mut self, val: f32) -> Self {
        self.params.insert(
            "surfacetension".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacetension_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacetension".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adhesion(mut self, val: f32) -> Self {
        self.params.insert(
            "adhesion".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_adhesion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adhesion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_massshockaxis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "massshockaxis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_massshockaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "massshockaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_groundpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "groundpos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_groundpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "groundpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_groundup(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "groundup".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_groundup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "groundup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gravity(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gravity".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gravity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gravity".to_string(),
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
    pub fn with_niter(mut self, val: i32) -> Self {
        self.params.insert(
            "niter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_niter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "niter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smoothiter(mut self, val: i32) -> Self {
        self.params.insert(
            "smoothiter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_smoothiter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smoothiter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisioniter(mut self, val: i32) -> Self {
        self.params.insert(
            "collisioniter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_collisioniter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisioniter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_postcollisioniter(mut self, val: i32) -> Self {
        self.params.insert(
            "postcollisioniter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_postcollisioniter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postcollisioniter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolveallmax(mut self, val: i32) -> Self {
        self.params.insert(
            "resolveallmax".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resolveallmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolveallmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxmultipass(mut self, val: i32) -> Self {
        self.params.insert(
            "maxmultipass".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxmultipass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxmultipass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_grain_maxneighbor(mut self, val: i32) -> Self {
        self.params.insert(
            "grain_maxneighbor".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_grain_maxneighbor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grain_maxneighbor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxviscosityiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "maxviscosityiterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxviscosityiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxviscosityiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sortinterval(mut self, val: i32) -> Self {
        self.params.insert(
            "sortinterval".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sortinterval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sortinterval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdfcollisioniter(mut self, val: i32) -> Self {
        self.params.insert(
            "sdfcollisioniter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sdfcollisioniter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdfcollisioniter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdfresolveallmax(mut self, val: i32) -> Self {
        self.params.insert(
            "sdfresolveallmax".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sdfresolveallmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdfresolveallmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_integration(mut self, val: DopVellumsolverIntegration) -> Self {
        self.params.insert(
            "integration".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_integration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "integration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_breakfrequency(mut self, val: DopVellumsolverBreakfrequency) -> Self {
        self.params.insert(
            "breakfrequency".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_breakfrequency_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "breakfrequency".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slidingmethod(mut self, val: DopVellumsolverSlidingmethod) -> Self {
        self.params.insert(
            "slidingmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_slidingmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slidingmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscositysolver(mut self, val: DopVellumsolverViscositysolver) -> Self {
        self.params.insert(
            "viscositysolver".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viscositysolver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscositysolver".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_forcefrequency(mut self, val: DopVellumsolverForcefrequency) -> Self {
        self.params.insert(
            "forcefrequency".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_forcefrequency_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcefrequency".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_secondarygroup(mut self, val: &str) -> Self {
        self.params.insert(
            "secondarygroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_secondarygroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "secondarygroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdfpath(mut self, val: &str) -> Self {
        self.params.insert(
            "sdfpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sdfpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdfpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kerneloptions(mut self, val: &str) -> Self {
        self.params.insert(
            "kerneloptions".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_kerneloptions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kerneloptions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablecollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "enablecollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablecollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doselfcollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "doselfcollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doselfcollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doselfcollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolveall(mut self, val: bool) -> Self {
        self.params.insert(
            "resolveall".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resolveall_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolveall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalforce(mut self, val: bool) -> Self {
        self.params.insert(
            "externalforce".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_externalforce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalforce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosecondary(mut self, val: bool) -> Self {
        self.params.insert(
            "dosecondary".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosecondary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dosecondary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domultipass(mut self, val: bool) -> Self {
        self.params.insert(
            "domultipass".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domultipass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domultipass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disablefailedcollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "disablefailedcollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_disablefailedcollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disablefailedcollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resetcollided(mut self, val: bool) -> Self {
        self.params.insert(
            "resetcollided".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resetcollided_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resetcollided".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initoverlap(mut self, val: bool) -> Self {
        self.params.insert(
            "initoverlap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_initoverlap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initoverlap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updateoverlap(mut self, val: bool) -> Self {
        self.params.insert(
            "updateoverlap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_updateoverlap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updateoverlap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dotet(mut self, val: bool) -> Self {
        self.params.insert(
            "dotet".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dotet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dotet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domaxaccel(mut self, val: bool) -> Self {
        self.params.insert(
            "domaxaccel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domaxaccel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domaxaccel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_accelfallback(mut self, val: bool) -> Self {
        self.params.insert(
            "accelfallback".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_accelfallback_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accelfallback".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitaccel(mut self, val: bool) -> Self {
        self.params.insert(
            "limitaccel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitaccel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitaccel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitdisplace(mut self, val: bool) -> Self {
        self.params.insert(
            "limitdisplace".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitdisplace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitdisplace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_disablebrokenwelds(mut self, val: bool) -> Self {
        self.params.insert(
            "disablebrokenwelds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_disablebrokenwelds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disablebrokenwelds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normalizestress(mut self, val: bool) -> Self {
        self.params.insert(
            "normalizestress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_normalizestress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalizestress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doautosleep(mut self, val: bool) -> Self {
        self.params.insert(
            "doautosleep".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doautosleep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doautosleep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_grain_uniformradius(mut self, val: bool) -> Self {
        self.params.insert(
            "grain_uniformradius".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_grain_uniformradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grain_uniformradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ignorepiece(mut self, val: bool) -> Self {
        self.params.insert(
            "ignorepiece".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ignorepiece_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ignorepiece".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doconstraintaveraging(mut self, val: bool) -> Self {
        self.params.insert(
            "doconstraintaveraging".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doconstraintaveraging_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doconstraintaveraging".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domassshock(mut self, val: bool) -> Self {
        self.params.insert(
            "domassshock".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_domassshock_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domassshock".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosort(mut self, val: bool) -> Self {
        self.params.insert(
            "dosort".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosort_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dosort".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minimalmode(mut self, val: bool) -> Self {
        self.params.insert(
            "minimalmode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_minimalmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minimalmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minimalnoinit(mut self, val: bool) -> Self {
        self.params.insert(
            "minimalnoinit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_minimalnoinit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minimalnoinit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useground(mut self, val: bool) -> Self {
        self.params.insert(
            "useground".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useground_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useground".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usegravity(mut self, val: bool) -> Self {
        self.params.insert(
            "usegravity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegravity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usegravity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdfcollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "sdfcollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sdfcollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdfcollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdfresolveall(mut self, val: bool) -> Self {
        self.params.insert(
            "sdfresolveall".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sdfresolveall_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdfresolveall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdfaccurategrad(mut self, val: bool) -> Self {
        self.params.insert(
            "sdfaccurategrad".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sdfaccurategrad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdfaccurategrad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_finish(mut self, val: bool) -> Self {
        self.params.insert(
            "finish".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_finish_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "finish".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_recompile(mut self, val: bool) -> Self {
        self.params.insert(
            "recompile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_recompile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "recompile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oclgraphcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "oclgraphcolor".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_oclgraphcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oclgraphcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oclneighborsearch(mut self, val: bool) -> Self {
        self.params.insert(
            "oclneighborsearch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_oclneighborsearch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oclneighborsearch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVellumsolver {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vellumsolver"
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
pub enum DopVellumsourceEmittype {
    OnlyOnce = 0,
    EachFrame = 1,
    EachSubstep = 2,
    InstanceOnPoints = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVellumsourceImportxform {
    None = 0,
    IntoThisObject = 1,
}

#[derive(Debug, Clone)]
pub struct DopVellumsource {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopVellumsource {
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
    pub fn with_activate(mut self, val: f32) -> Self {
        self.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particledensity(mut self, val: f32) -> Self {
        self.params.insert(
            "particledensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particledensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particledensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_emittype(mut self, val: DopVellumsourceEmittype) -> Self {
        self.params.insert(
            "emittype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_emittype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emittype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importxform(mut self, val: DopVellumsourceImportxform) -> Self {
        self.params.insert(
            "importxform".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_importxform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importxform".to_string(),
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
    pub fn with_targetpath(mut self, val: &str) -> Self {
        self.params.insert(
            "targetpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraintpath(mut self, val: &str) -> Self {
        self.params.insert(
            "constraintpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraintpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraintpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instancepath(mut self, val: &str) -> Self {
        self.params.insert(
            "instancepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_instancepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instancepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setpt(mut self, val: &str) -> Self {
        self.params.insert(
            "setpt".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_setpt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setpt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vellumname(mut self, val: &str) -> Self {
        self.params.insert(
            "vellumname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vellumname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vellumname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_streamname(mut self, val: &str) -> Self {
        self.params.insert(
            "streamname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_streamname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "streamname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindgeo(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgeo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindconstraint(mut self, val: &str) -> Self {
        self.params.insert(
            "bindconstraint".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindconstraint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindconstraint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindpatch(mut self, val: &str) -> Self {
        self.params.insert(
            "bindpatch".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindpatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindpatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVellumsource {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vellumsource"
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
pub struct DopVisualizegeometry {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopVisualizegeometry {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- String parameters ---
    pub fn with_bindgeo(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgeo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_clear_visualizers(mut self, val: bool) -> Self {
        self.params.insert(
            "clear_visualizers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clear_visualizers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clear_visualizers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVisualizegeometry {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "visualizegeometry"
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
pub enum DopVolumeParmopMode {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeMode {
    RayIntersect = 0,
    MetaBalls = 1,
    ImplicitBox = 2,
    ImplicitSphere = 3,
    ImplicitPlane = 4,
    Minimum = 5,
    VolumeSample = 6,
    Heightfield = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeParmopUniformvoxels {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeParmopUniformdiv {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeParmopDiv {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeParmopDivsize {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeParmopLaserscan {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeParmopFixsigns {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeParmopForcebounds {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeParmopInvert {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeParmopOffset {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeParmopTol {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeParmopSweepalpha {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeParmopSweepcount {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopVolume {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopVolume {
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

    // --- Float parameters ---
    pub fn with_divsize(mut self, val: f32) -> Self {
        self.params.insert(
            "divsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divsize".to_string(),
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
    pub fn with_tol(mut self, val: f32) -> Self {
        self.params.insert(
            "tol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sweepalpha(mut self, val: f32) -> Self {
        self.params.insert(
            "sweepalpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sweepalpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sweepalpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
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

    // --- Int parameters ---
    pub fn with_uniformdiv(mut self, val: i32) -> Self {
        self.params.insert(
            "uniformdiv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uniformdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sweepcount(mut self, val: i32) -> Self {
        self.params.insert(
            "sweepcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sweepcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sweepcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_div(mut self, val: [i32; 3]) -> Self {
        self.params
            .insert("div".to_string(), crate::core::types::ParamValue::Int3(val));
        self
    }
    pub fn with_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_mode(mut self, val: DopVolumeParmopMode) -> Self {
        self.params.insert(
            "parmop_mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: DopVolumeMode) -> Self {
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
    pub fn with_parmop_uniformvoxels(mut self, val: DopVolumeParmopUniformvoxels) -> Self {
        self.params.insert(
            "parmop_uniformvoxels".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_uniformvoxels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_uniformvoxels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformvoxels(mut self, val: DopVolumeUniformvoxels) -> Self {
        self.params.insert(
            "uniformvoxels".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uniformvoxels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformvoxels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_uniformdiv(mut self, val: DopVolumeParmopUniformdiv) -> Self {
        self.params.insert(
            "parmop_uniformdiv".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_uniformdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_uniformdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_div(mut self, val: DopVolumeParmopDiv) -> Self {
        self.params.insert(
            "parmop_div".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_divsize(mut self, val: DopVolumeParmopDivsize) -> Self {
        self.params.insert(
            "parmop_divsize".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_divsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_divsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_laserscan(mut self, val: DopVolumeParmopLaserscan) -> Self {
        self.params.insert(
            "parmop_laserscan".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_laserscan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_laserscan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_fixsigns(mut self, val: DopVolumeParmopFixsigns) -> Self {
        self.params.insert(
            "parmop_fixsigns".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_fixsigns_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_fixsigns".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_forcebounds(mut self, val: DopVolumeParmopForcebounds) -> Self {
        self.params.insert(
            "parmop_forcebounds".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_forcebounds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_forcebounds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_invert(mut self, val: DopVolumeParmopInvert) -> Self {
        self.params.insert(
            "parmop_invert".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_invert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_invert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_offset(mut self, val: DopVolumeParmopOffset) -> Self {
        self.params.insert(
            "parmop_offset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_tol(mut self, val: DopVolumeParmopTol) -> Self {
        self.params.insert(
            "parmop_tol".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_tol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_tol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_sweepalpha(mut self, val: DopVolumeParmopSweepalpha) -> Self {
        self.params.insert(
            "parmop_sweepalpha".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_sweepalpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_sweepalpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_sweepcount(mut self, val: DopVolumeParmopSweepcount) -> Self {
        self.params.insert(
            "parmop_sweepcount".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_sweepcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_sweepcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopVolumeDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopVolumeSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_laserscan(mut self, val: bool) -> Self {
        self.params.insert(
            "laserscan".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_laserscan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "laserscan".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fixsigns(mut self, val: bool) -> Self {
        self.params.insert(
            "fixsigns".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fixsigns_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fixsigns".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_forcebounds(mut self, val: bool) -> Self {
        self.params.insert(
            "forcebounds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forcebounds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcebounds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_invert(mut self, val: bool) -> Self {
        self.params.insert(
            "invert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "invert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVolume {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "volume"
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
pub enum DopVolumecolliderSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopVolumecollider {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopVolumecollider {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sharedata(mut self, val: DopVolumecolliderSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVolumecollider {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "volumecollider"
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
pub enum DopVolumeinstancesourceInput {
    Sop = 0,
    FirstContextGeometry = 1,
    SecondContextGeometry = 2,
    ThirdContextGeometry = 3,
    FourthContextGeometry = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumeinstancesourceDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone)]
pub struct DopVolumeinstancesource {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopVolumeinstancesource {
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
    pub fn with_referenceframe(mut self, val: i32) -> Self {
        self.params.insert(
            "referenceframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_referenceframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "referenceframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_input(mut self, val: DopVolumeinstancesourceInput) -> Self {
        self.params.insert(
            "input".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_defaultparmop(mut self, val: DopVolumeinstancesourceDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_matchfield(mut self, val: &str) -> Self {
        self.params.insert(
            "matchfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matchfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchfield".to_string(),
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
    pub fn with_instgeo(mut self, val: &str) -> Self {
        self.params.insert(
            "instgeo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_instgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instgeo".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_createmissing(mut self, val: bool) -> Self {
        self.params.insert(
            "createmissing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createmissing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createmissing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resizefields(mut self, val: bool) -> Self {
        self.params.insert(
            "resizefields".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resizefields_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resizefields".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fulltiles(mut self, val: bool) -> Self {
        self.params.insert(
            "fulltiles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fulltiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fulltiles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addaffectors(mut self, val: bool) -> Self {
        self.params.insert(
            "addaffectors".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addaffectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addaffectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solverperobject(mut self, val: bool) -> Self {
        self.params.insert(
            "solverperobject".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solverperobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solverperobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVolumeinstancesource {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "volumeinstancesource"
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
pub enum DopVolumesourceInitialize {
    /// Initialize ↓
    Initialize = 0,
    SourceSmoke = 1,
    Sink = 2,
    Collision = 3,
    Pump = 4,
    Expand = 5,
    SourceFuel = 6,
    SourceFlip = 7,
    SinkFlip = 8,
    SourceOceanLayer = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumesourceInput {
    Sop = 0,
    FirstContextGeometry = 1,
    SecondContextGeometry = 2,
    ThirdContextGeometry = 3,
    FourthContextGeometry = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumesourceXformtype {
    None = 0,
    IntoThisObject = 1,
    IntoSpecifiedObject = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumesourceRank {
    Scalar = 0,
    Vector = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumesourceVoperator {
    Copy = 0,
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    Divide = 4,
    Maximum = 5,
    Minimum = 6,
    Average = 7,
    Pull = 8,
    Blend = 9,
    None = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVolumesourceInstancing {
    AutoDetect = 0,
    Off = 1,
    On = 2,
}

#[derive(Debug, Clone)]
pub struct DopVolumesource {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopVolumesource {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_vscale_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("vscale{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vscale_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("vscale{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_accguidestr_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("accguidestr{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_accguidestr_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("accguidestr{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_decguidestr_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("decguidestr{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_decguidestr_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("decguidestr{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dirguidestr_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("dirguidestr{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dirguidestr_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dirguidestr{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offsetscale(mut self, val: f32) -> Self {
        self.params.insert(
            "offsetscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offsetscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offsetscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offsetseed(mut self, val: f32) -> Self {
        self.params.insert(
            "offsetseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offsetseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offsetseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_life(mut self, val: f32) -> Self {
        self.params.insert(
            "life".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_life_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "life".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lifevar(mut self, val: f32) -> Self {
        self.params.insert(
            "lifevar".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lifevar_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lifevar".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activate_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("activate{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activate_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("activate{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_initialize(mut self, val: DopVolumesourceInitialize) -> Self {
        self.params.insert(
            "initialize".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_initialize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initialize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_input(mut self, val: DopVolumesourceInput) -> Self {
        self.params.insert(
            "input".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_xformtype(mut self, val: DopVolumesourceXformtype) -> Self {
        self.params.insert(
            "xformtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xformtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rank_inst(mut self, index1: usize, val: DopVolumesourceRank) -> Self {
        self.params.insert(
            format!("rank{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rank_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("rank{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_voperator_inst(mut self, index1: usize, val: DopVolumesourceVoperator) -> Self {
        self.params.insert(
            format!("voperator{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_voperator_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("voperator{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instancing(mut self, val: DopVolumesourceInstancing) -> Self {
        self.params.insert(
            "instancing".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_instancing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instancing".to_string(),
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
    pub fn with_xformpath(mut self, val: &str) -> Self {
        self.params.insert(
            "xformpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_xformpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xformpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchfield(mut self, val: &str) -> Self {
        self.params.insert(
            "matchfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matchfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volume_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("volume{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_volume_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("volume{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weightvolume_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("weightvolume{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_weightvolume_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("weightvolume{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vfield_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("vfield{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vfield_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("vfield{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vweightfield_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("vweightfield{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vweightfield_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("vweightfield{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sopmask_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("sopmask{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sopmask_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("sopmask{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mask_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("mask{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mask_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("mask{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcegroup(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_killinsidesop(mut self, val: &str) -> Self {
        self.params.insert(
            "killinsidesop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_killinsidesop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "killinsidesop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_killinsidedop(mut self, val: &str) -> Self {
        self.params.insert(
            "killinsidedop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_killinsidedop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "killinsidedop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_streamname(mut self, val: &str) -> Self {
        self.params.insert(
            "streamname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_streamname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "streamname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_createmissing(mut self, val: bool) -> Self {
        self.params.insert(
            "createmissing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createmissing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createmissing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resizefields(mut self, val: bool) -> Self {
        self.params.insert(
            "resizefields".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resizefields_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resizefields".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fulltiles(mut self, val: bool) -> Self {
        self.params.insert(
            "fulltiles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fulltiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fulltiles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clamped_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("clamped{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clamped_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clamped{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sopmabsolute_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("sopmabsolute{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sopmabsolute_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("sopmabsolute{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sopsdf_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("sopsdf{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sopsdf_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("sopsdf{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mabsolute_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("mabsolute{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mabsolute_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("mabsolute{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_msdf_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("msdf{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_msdf_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("msdf{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vnormalize_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("vnormalize{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vnormalize_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("vnormalize{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usenorm_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("usenorm{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usenorm_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("usenorm{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nonneg_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("nonneg{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_nonneg_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("nonneg{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledirblend_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enabledirblend{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledirblend_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enabledirblend{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourceparticles(mut self, val: bool) -> Self {
        self.params.insert(
            "sourceparticles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sourceparticles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceparticles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dolife(mut self, val: bool) -> Self {
        self.params.insert(
            "dolife".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dolife_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dolife".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_killinside(mut self, val: bool) -> Self {
        self.params.insert(
            "killinside".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_killinside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "killinside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVolumesource {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "volumesource"
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
pub enum DopVopforceVexsrc {
    Myself = 0,
    Shop = 1,
    Script = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVopforceSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopVopforce {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopVopforce {
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
    pub fn trigger_vexclear(mut self) -> Self {
        self.params.insert(
            "vexclear".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vop_forcecompile(mut self) -> Self {
        self.params.insert(
            "vop_forcecompile".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_vexsrc(mut self, val: DopVopforceVexsrc) -> Self {
        self.params.insert(
            "vexsrc".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vexsrc_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vexsrc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopVopforceSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_vexshoppath(mut self, val: &str) -> Self {
        self.params.insert(
            "vexshoppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vexshoppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vexshoppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vexscript(mut self, val: &str) -> Self {
        self.params.insert(
            "vexscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vexscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vexscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vop_compiler(mut self, val: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vop_compiler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vop_compiler".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVopforce {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopforce"
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
pub trait DopVopforceInnerExt {
    fn vopforceglobal1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn vopforceoutput1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> DopVopforceInnerExt for crate::core::graph::InnerGraph<'a> {
    fn vopforceglobal1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("vopforceglobal1")
    }
    fn vopforceoutput1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("vopforceoutput1")
    }
}

#[derive(Debug, Clone)]
pub struct DopVopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopVopnet {
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

impl crate::core::types::HoudiniNode for DopVopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vopnet"
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
pub enum DopVoronoifractureconfigureobjectSurfacecluster {
    UseFractureSettings = 0,
    Disabled = 1,
    SinglePiece = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureconfigureobjectInteriorcluster {
    UseFractureSettings = 0,
    Disabled = 1,
    SinglePiece = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureconfigureobjectExteriorscatter {
    AtImpact = 0,
    ExteriorVolume = 1,
    Both = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureconfigureobjectExteriorcluster {
    UseFractureSettings = 0,
    Disabled = 1,
    SinglePiece = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureconfigureobjectDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone)]
pub struct DopVoronoifractureconfigureobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopVoronoifractureconfigureobject {
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

    /// Connects to input 0: "Objects to be Processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be Processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Fracture Noise"
    pub fn set_input_fracture_noise<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Fracture Noise" and specifies the output index of the target node.
    pub fn set_input_fracture_noise_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_minvolume(mut self, val: f32) -> Self {
        self.params.insert(
            "minvolume".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refracturedelay(mut self, val: f32) -> Self {
        self.params.insert(
            "refracturedelay".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refracturedelay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refracturedelay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minmagnetvol(mut self, val: f32) -> Self {
        self.params.insert(
            "minmagnetvol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minmagnetvol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minmagnetvol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nptsperarea(mut self, val: f32) -> Self {
        self.params.insert(
            "nptsperarea".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_nptsperarea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nptsperarea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceptdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "surfaceptdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfaceptdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfaceptdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "surfaceoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfaceoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfaceoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacescale(mut self, val: f32) -> Self {
        self.params.insert(
            "surfacescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiorptdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "interiorptdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_interiorptdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interiorptdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exteriorptdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "exteriorptdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exteriorptdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exteriorptdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exterioroffset(mut self, val: f32) -> Self {
        self.params.insert(
            "exterioroffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exterioroffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exterioroffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cuspangle(mut self, val: f32) -> Self {
        self.params.insert(
            "cuspangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cuspangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cuspangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cuspouterangle(mut self, val: f32) -> Self {
        self.params.insert(
            "cuspouterangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cuspouterangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cuspouterangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cutplaneoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "cutplaneoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cutplaneoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cutplaneoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_detachseed(mut self, val: f32) -> Self {
        self.params.insert(
            "detachseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_detachseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "detachseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_detachratio(mut self, val: f32) -> Self {
        self.params.insert(
            "detachratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_detachratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "detachratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_detailsize(mut self, val: f32) -> Self {
        self.params.insert(
            "detailsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_detailsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "detailsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intnoiseamp(mut self, val: f32) -> Self {
        self.params.insert(
            "intnoiseamp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_intnoiseamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intnoiseamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthnoisescalebias(mut self, val: f32) -> Self {
        self.params.insert(
            "depthnoisescalebias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depthnoisescalebias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthnoisescalebias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velbias(mut self, val: f32) -> Self {
        self.params.insert(
            "velbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impulsedist(mut self, val: f32) -> Self {
        self.params.insert(
            "impulsedist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_impulsedist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impulsedist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impulsescaleradial(mut self, val: f32) -> Self {
        self.params.insert(
            "impulsescaleradial".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_impulsescaleradial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impulsescaleradial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impulsescalenormal(mut self, val: f32) -> Self {
        self.params.insert(
            "impulsescalenormal".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_impulsescalenormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impulsescalenormal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_minimpact(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "minimpact".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_minimpact_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minimpact".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_radiusscale(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "radiusscale".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_radiusscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radiusscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_impactradius(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "impactradius".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_impactradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impactradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clustersize(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "clustersize".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_clustersize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clustersize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clusteroffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "clusteroffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_clusteroffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clusteroffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clusterjitter(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "clusterjitter".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_clusterjitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clusterjitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intnoisefreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "intnoisefreq".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_intnoisefreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intnoisefreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intnoiseoffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "intnoiseoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_intnoiseoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intnoiseoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_enablefracturing(mut self, val: i32) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_enablefracturing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxfractures(mut self, val: i32) -> Self {
        self.params.insert(
            "maxfractures".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxfractures_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxfractures".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_npts(mut self, val: i32) -> Self {
        self.params
            .insert("npts".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_npts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "npts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intnoiseturb(mut self, val: i32) -> Self {
        self.params.insert(
            "intnoiseturb".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_intnoiseturb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intnoiseturb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_surfacecluster(
        mut self,
        val: DopVoronoifractureconfigureobjectSurfacecluster,
    ) -> Self {
        self.params.insert(
            "surfacecluster".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surfacecluster_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacecluster".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiorcluster(
        mut self,
        val: DopVoronoifractureconfigureobjectInteriorcluster,
    ) -> Self {
        self.params.insert(
            "interiorcluster".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_interiorcluster_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interiorcluster".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exteriorscatter(
        mut self,
        val: DopVoronoifractureconfigureobjectExteriorscatter,
    ) -> Self {
        self.params.insert(
            "exteriorscatter".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_exteriorscatter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exteriorscatter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exteriorcluster(
        mut self,
        val: DopVoronoifractureconfigureobjectExteriorcluster,
    ) -> Self {
        self.params.insert(
            "exteriorcluster".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_exteriorcluster_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exteriorcluster".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(
        mut self,
        val: DopVoronoifractureconfigureobjectDefaultparmop,
    ) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
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
    pub fn with_impactgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "impactgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_impactgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impactgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_newpiecegroup(mut self, val: &str) -> Self {
        self.params.insert(
            "newpiecegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_newpiecegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newpiecegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usemagnetgeo(mut self, val: bool) -> Self {
        self.params.insert(
            "usemagnetgeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemagnetgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemagnetgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptsperarea(mut self, val: bool) -> Self {
        self.params.insert(
            "ptsperarea".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ptsperarea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptsperarea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptsperimpact(mut self, val: bool) -> Self {
        self.params.insert(
            "ptsperimpact".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ptsperimpact_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptsperimpact".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showfracturepts(mut self, val: bool) -> Self {
        self.params.insert(
            "showfracturepts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showfracturepts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showfracturepts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cuspnormals(mut self, val: bool) -> Self {
        self.params.insert(
            "cuspnormals".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cuspnormals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cuspnormals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cuspouternormals(mut self, val: bool) -> Self {
        self.params.insert(
            "cuspouternormals".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cuspouternormals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cuspouternormals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cluster(mut self, val: bool) -> Self {
        self.params.insert(
            "cluster".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cluster_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cluster".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomdetach(mut self, val: bool) -> Self {
        self.params.insert(
            "randomdetach".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomdetach_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomdetach".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addinteriordetail(mut self, val: bool) -> Self {
        self.params.insert(
            "addinteriordetail".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addinteriordetail_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addinteriordetail".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVoronoifractureconfigureobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "voronoifractureconfigureobject"
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
pub enum DopVoronoifractureparmsParmopImpactgroup {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopMinimpact {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopMinvolume {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopRefracturedelay {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopUsemagnetgeo {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopMinmagnetvol {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopMaxfractures {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopImpactradius {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopRadiusscale {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopPtsperarea {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopNptsperarea {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopNpts {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopPtsperimpact {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopSurfaceptdensity {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopSurfaceoffset {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopSurfacescale {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopSurfacecluster {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsSurfacecluster {
    UseFractureSettings = 0,
    Disabled = 1,
    SinglePiece = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopInteriorptdensity {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopInteriorcluster {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsInteriorcluster {
    UseFractureSettings = 0,
    Disabled = 1,
    SinglePiece = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopExteriorptdensity {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopExteriorscatter {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsExteriorscatter {
    AtImpact = 0,
    ExteriorVolume = 1,
    Both = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopExterioroffset {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopExteriorcluster {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsExteriorcluster {
    UseFractureSettings = 0,
    Disabled = 1,
    SinglePiece = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopCuspnormals {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopCuspangle {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopCuspouternormals {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopCuspouterangle {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopCutplaneoffset {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopCluster {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopClustersize {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopClusteroffset {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopClusterjitter {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopRandomdetach {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopDetachseed {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopDetachratio {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopAddinteriordetail {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopDetailsize {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopIntnoiseamp {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopIntnoisefreq {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopIntnoiseoffset {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopIntnoiseturb {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopDepthnoisescalebias {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopVelbias {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopImpulsedist {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopImpulsescaleradial {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsParmopImpulsescalenormal {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVoronoifractureparmsSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopVoronoifractureparms {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopVoronoifractureparms {
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

    // --- Float parameters ---
    pub fn with_minvolume(mut self, val: f32) -> Self {
        self.params.insert(
            "minvolume".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refracturedelay(mut self, val: f32) -> Self {
        self.params.insert(
            "refracturedelay".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_refracturedelay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refracturedelay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minmagnetvol(mut self, val: f32) -> Self {
        self.params.insert(
            "minmagnetvol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minmagnetvol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minmagnetvol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nptsperarea(mut self, val: f32) -> Self {
        self.params.insert(
            "nptsperarea".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_nptsperarea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nptsperarea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceptdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "surfaceptdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfaceptdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfaceptdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfaceoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "surfaceoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfaceoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfaceoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacescale(mut self, val: f32) -> Self {
        self.params.insert(
            "surfacescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiorptdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "interiorptdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_interiorptdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interiorptdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exteriorptdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "exteriorptdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exteriorptdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exteriorptdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exterioroffset(mut self, val: f32) -> Self {
        self.params.insert(
            "exterioroffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_exterioroffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exterioroffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cuspangle(mut self, val: f32) -> Self {
        self.params.insert(
            "cuspangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cuspangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cuspangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cuspouterangle(mut self, val: f32) -> Self {
        self.params.insert(
            "cuspouterangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cuspouterangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cuspouterangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cutplaneoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "cutplaneoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cutplaneoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cutplaneoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_detachseed(mut self, val: f32) -> Self {
        self.params.insert(
            "detachseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_detachseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "detachseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_detachratio(mut self, val: f32) -> Self {
        self.params.insert(
            "detachratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_detachratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "detachratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_detailsize(mut self, val: f32) -> Self {
        self.params.insert(
            "detailsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_detailsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "detailsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intnoiseamp(mut self, val: f32) -> Self {
        self.params.insert(
            "intnoiseamp".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_intnoiseamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intnoiseamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_depthnoisescalebias(mut self, val: f32) -> Self {
        self.params.insert(
            "depthnoisescalebias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_depthnoisescalebias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "depthnoisescalebias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velbias(mut self, val: f32) -> Self {
        self.params.insert(
            "velbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impulsedist(mut self, val: f32) -> Self {
        self.params.insert(
            "impulsedist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_impulsedist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impulsedist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impulsescaleradial(mut self, val: f32) -> Self {
        self.params.insert(
            "impulsescaleradial".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_impulsescaleradial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impulsescaleradial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_impulsescalenormal(mut self, val: f32) -> Self {
        self.params.insert(
            "impulsescalenormal".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_impulsescalenormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impulsescalenormal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_minimpact(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "minimpact".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_minimpact_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minimpact".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_radiusscale(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "radiusscale".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_radiusscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radiusscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_impactradius(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "impactradius".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_impactradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impactradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clustersize(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "clustersize".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_clustersize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clustersize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clusteroffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "clusteroffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_clusteroffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clusteroffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clusterjitter(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "clusterjitter".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_clusterjitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clusterjitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intnoisefreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "intnoisefreq".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_intnoisefreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intnoisefreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intnoiseoffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "intnoiseoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_intnoiseoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intnoiseoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxfractures(mut self, val: i32) -> Self {
        self.params.insert(
            "maxfractures".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxfractures_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxfractures".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_npts(mut self, val: i32) -> Self {
        self.params
            .insert("npts".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_npts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "npts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intnoiseturb(mut self, val: i32) -> Self {
        self.params.insert(
            "intnoiseturb".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_intnoiseturb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intnoiseturb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_impactgroup(
        mut self,
        val: DopVoronoifractureparmsParmopImpactgroup,
    ) -> Self {
        self.params.insert(
            "parmop_impactgroup".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_impactgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_impactgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_minimpact(mut self, val: DopVoronoifractureparmsParmopMinimpact) -> Self {
        self.params.insert(
            "parmop_minimpact".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_minimpact_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_minimpact".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_minvolume(mut self, val: DopVoronoifractureparmsParmopMinvolume) -> Self {
        self.params.insert(
            "parmop_minvolume".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_minvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_minvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_refracturedelay(
        mut self,
        val: DopVoronoifractureparmsParmopRefracturedelay,
    ) -> Self {
        self.params.insert(
            "parmop_refracturedelay".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_refracturedelay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_refracturedelay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_usemagnetgeo(
        mut self,
        val: DopVoronoifractureparmsParmopUsemagnetgeo,
    ) -> Self {
        self.params.insert(
            "parmop_usemagnetgeo".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_usemagnetgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_usemagnetgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_minmagnetvol(
        mut self,
        val: DopVoronoifractureparmsParmopMinmagnetvol,
    ) -> Self {
        self.params.insert(
            "parmop_minmagnetvol".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_minmagnetvol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_minmagnetvol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_maxfractures(
        mut self,
        val: DopVoronoifractureparmsParmopMaxfractures,
    ) -> Self {
        self.params.insert(
            "parmop_maxfractures".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_maxfractures_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_maxfractures".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_impactradius(
        mut self,
        val: DopVoronoifractureparmsParmopImpactradius,
    ) -> Self {
        self.params.insert(
            "parmop_impactradius".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_impactradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_impactradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_radiusscale(
        mut self,
        val: DopVoronoifractureparmsParmopRadiusscale,
    ) -> Self {
        self.params.insert(
            "parmop_radiusscale".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_radiusscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_radiusscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_ptsperarea(mut self, val: DopVoronoifractureparmsParmopPtsperarea) -> Self {
        self.params.insert(
            "parmop_ptsperarea".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_ptsperarea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_ptsperarea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_nptsperarea(
        mut self,
        val: DopVoronoifractureparmsParmopNptsperarea,
    ) -> Self {
        self.params.insert(
            "parmop_nptsperarea".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_nptsperarea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_nptsperarea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_npts(mut self, val: DopVoronoifractureparmsParmopNpts) -> Self {
        self.params.insert(
            "parmop_npts".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_npts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_npts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_ptsperimpact(
        mut self,
        val: DopVoronoifractureparmsParmopPtsperimpact,
    ) -> Self {
        self.params.insert(
            "parmop_ptsperimpact".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_ptsperimpact_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_ptsperimpact".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_surfaceptdensity(
        mut self,
        val: DopVoronoifractureparmsParmopSurfaceptdensity,
    ) -> Self {
        self.params.insert(
            "parmop_surfaceptdensity".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_surfaceptdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_surfaceptdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_surfaceoffset(
        mut self,
        val: DopVoronoifractureparmsParmopSurfaceoffset,
    ) -> Self {
        self.params.insert(
            "parmop_surfaceoffset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_surfaceoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_surfaceoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_surfacescale(
        mut self,
        val: DopVoronoifractureparmsParmopSurfacescale,
    ) -> Self {
        self.params.insert(
            "parmop_surfacescale".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_surfacescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_surfacescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_surfacecluster(
        mut self,
        val: DopVoronoifractureparmsParmopSurfacecluster,
    ) -> Self {
        self.params.insert(
            "parmop_surfacecluster".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_surfacecluster_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_surfacecluster".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacecluster(mut self, val: DopVoronoifractureparmsSurfacecluster) -> Self {
        self.params.insert(
            "surfacecluster".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surfacecluster_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacecluster".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_interiorptdensity(
        mut self,
        val: DopVoronoifractureparmsParmopInteriorptdensity,
    ) -> Self {
        self.params.insert(
            "parmop_interiorptdensity".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_interiorptdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_interiorptdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_interiorcluster(
        mut self,
        val: DopVoronoifractureparmsParmopInteriorcluster,
    ) -> Self {
        self.params.insert(
            "parmop_interiorcluster".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_interiorcluster_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_interiorcluster".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interiorcluster(mut self, val: DopVoronoifractureparmsInteriorcluster) -> Self {
        self.params.insert(
            "interiorcluster".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_interiorcluster_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interiorcluster".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_exteriorptdensity(
        mut self,
        val: DopVoronoifractureparmsParmopExteriorptdensity,
    ) -> Self {
        self.params.insert(
            "parmop_exteriorptdensity".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_exteriorptdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_exteriorptdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_exteriorscatter(
        mut self,
        val: DopVoronoifractureparmsParmopExteriorscatter,
    ) -> Self {
        self.params.insert(
            "parmop_exteriorscatter".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_exteriorscatter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_exteriorscatter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exteriorscatter(mut self, val: DopVoronoifractureparmsExteriorscatter) -> Self {
        self.params.insert(
            "exteriorscatter".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_exteriorscatter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exteriorscatter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_exterioroffset(
        mut self,
        val: DopVoronoifractureparmsParmopExterioroffset,
    ) -> Self {
        self.params.insert(
            "parmop_exterioroffset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_exterioroffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_exterioroffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_exteriorcluster(
        mut self,
        val: DopVoronoifractureparmsParmopExteriorcluster,
    ) -> Self {
        self.params.insert(
            "parmop_exteriorcluster".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_exteriorcluster_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_exteriorcluster".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exteriorcluster(mut self, val: DopVoronoifractureparmsExteriorcluster) -> Self {
        self.params.insert(
            "exteriorcluster".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_exteriorcluster_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exteriorcluster".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_cuspnormals(
        mut self,
        val: DopVoronoifractureparmsParmopCuspnormals,
    ) -> Self {
        self.params.insert(
            "parmop_cuspnormals".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_cuspnormals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_cuspnormals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_cuspangle(mut self, val: DopVoronoifractureparmsParmopCuspangle) -> Self {
        self.params.insert(
            "parmop_cuspangle".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_cuspangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_cuspangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_cuspouternormals(
        mut self,
        val: DopVoronoifractureparmsParmopCuspouternormals,
    ) -> Self {
        self.params.insert(
            "parmop_cuspouternormals".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_cuspouternormals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_cuspouternormals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_cuspouterangle(
        mut self,
        val: DopVoronoifractureparmsParmopCuspouterangle,
    ) -> Self {
        self.params.insert(
            "parmop_cuspouterangle".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_cuspouterangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_cuspouterangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_cutplaneoffset(
        mut self,
        val: DopVoronoifractureparmsParmopCutplaneoffset,
    ) -> Self {
        self.params.insert(
            "parmop_cutplaneoffset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_cutplaneoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_cutplaneoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_cluster(mut self, val: DopVoronoifractureparmsParmopCluster) -> Self {
        self.params.insert(
            "parmop_cluster".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_cluster_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_cluster".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_clustersize(
        mut self,
        val: DopVoronoifractureparmsParmopClustersize,
    ) -> Self {
        self.params.insert(
            "parmop_clustersize".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_clustersize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_clustersize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_clusteroffset(
        mut self,
        val: DopVoronoifractureparmsParmopClusteroffset,
    ) -> Self {
        self.params.insert(
            "parmop_clusteroffset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_clusteroffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_clusteroffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_clusterjitter(
        mut self,
        val: DopVoronoifractureparmsParmopClusterjitter,
    ) -> Self {
        self.params.insert(
            "parmop_clusterjitter".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_clusterjitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_clusterjitter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_randomdetach(
        mut self,
        val: DopVoronoifractureparmsParmopRandomdetach,
    ) -> Self {
        self.params.insert(
            "parmop_randomdetach".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_randomdetach_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_randomdetach".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_detachseed(mut self, val: DopVoronoifractureparmsParmopDetachseed) -> Self {
        self.params.insert(
            "parmop_detachseed".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_detachseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_detachseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_detachratio(
        mut self,
        val: DopVoronoifractureparmsParmopDetachratio,
    ) -> Self {
        self.params.insert(
            "parmop_detachratio".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_detachratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_detachratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_addinteriordetail(
        mut self,
        val: DopVoronoifractureparmsParmopAddinteriordetail,
    ) -> Self {
        self.params.insert(
            "parmop_addinteriordetail".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_addinteriordetail_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_addinteriordetail".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_detailsize(mut self, val: DopVoronoifractureparmsParmopDetailsize) -> Self {
        self.params.insert(
            "parmop_detailsize".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_detailsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_detailsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_intnoiseamp(
        mut self,
        val: DopVoronoifractureparmsParmopIntnoiseamp,
    ) -> Self {
        self.params.insert(
            "parmop_intnoiseamp".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_intnoiseamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_intnoiseamp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_intnoisefreq(
        mut self,
        val: DopVoronoifractureparmsParmopIntnoisefreq,
    ) -> Self {
        self.params.insert(
            "parmop_intnoisefreq".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_intnoisefreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_intnoisefreq".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_intnoiseoffset(
        mut self,
        val: DopVoronoifractureparmsParmopIntnoiseoffset,
    ) -> Self {
        self.params.insert(
            "parmop_intnoiseoffset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_intnoiseoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_intnoiseoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_intnoiseturb(
        mut self,
        val: DopVoronoifractureparmsParmopIntnoiseturb,
    ) -> Self {
        self.params.insert(
            "parmop_intnoiseturb".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_intnoiseturb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_intnoiseturb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_depthnoisescalebias(
        mut self,
        val: DopVoronoifractureparmsParmopDepthnoisescalebias,
    ) -> Self {
        self.params.insert(
            "parmop_depthnoisescalebias".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_depthnoisescalebias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_depthnoisescalebias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_velbias(mut self, val: DopVoronoifractureparmsParmopVelbias) -> Self {
        self.params.insert(
            "parmop_velbias".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_velbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_velbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_impulsedist(
        mut self,
        val: DopVoronoifractureparmsParmopImpulsedist,
    ) -> Self {
        self.params.insert(
            "parmop_impulsedist".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_impulsedist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_impulsedist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_impulsescaleradial(
        mut self,
        val: DopVoronoifractureparmsParmopImpulsescaleradial,
    ) -> Self {
        self.params.insert(
            "parmop_impulsescaleradial".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_impulsescaleradial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_impulsescaleradial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_impulsescalenormal(
        mut self,
        val: DopVoronoifractureparmsParmopImpulsescalenormal,
    ) -> Self {
        self.params.insert(
            "parmop_impulsescalenormal".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_impulsescalenormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_impulsescalenormal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopVoronoifractureparmsDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopVoronoifractureparmsSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_impactgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "impactgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_impactgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impactgroup".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usemagnetgeo(mut self, val: bool) -> Self {
        self.params.insert(
            "usemagnetgeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemagnetgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemagnetgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptsperarea(mut self, val: bool) -> Self {
        self.params.insert(
            "ptsperarea".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ptsperarea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptsperarea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ptsperimpact(mut self, val: bool) -> Self {
        self.params.insert(
            "ptsperimpact".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ptsperimpact_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptsperimpact".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cuspnormals(mut self, val: bool) -> Self {
        self.params.insert(
            "cuspnormals".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cuspnormals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cuspnormals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cuspouternormals(mut self, val: bool) -> Self {
        self.params.insert(
            "cuspouternormals".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cuspouternormals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cuspouternormals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cluster(mut self, val: bool) -> Self {
        self.params.insert(
            "cluster".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cluster_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cluster".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomdetach(mut self, val: bool) -> Self {
        self.params.insert(
            "randomdetach".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomdetach_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomdetach".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addinteriordetail(mut self, val: bool) -> Self {
        self.params.insert(
            "addinteriordetail".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addinteriordetail_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addinteriordetail".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVoronoifractureparms {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "voronoifractureparms"
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
pub struct DopVoronoifracturesolver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopVoronoifracturesolver {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_minpiecevol(mut self, val: f32) -> Self {
        self.params.insert(
            "minpiecevol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minpiecevol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minpiecevol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clusterfusedist(mut self, val: f32) -> Self {
        self.params.insert(
            "clusterfusedist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clusterfusedist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clusterfusedist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_converttopolylod(mut self, val: f32) -> Self {
        self.params.insert(
            "converttopolylod".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_converttopolylod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "converttopolylod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_seed(mut self, val: i32) -> Self {
        self.params
            .insert("seed".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seed".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_stampcreation(mut self, val: bool) -> Self {
        self.params.insert(
            "stampcreation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stampcreation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stampcreation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_feedback(mut self, val: bool) -> Self {
        self.params.insert(
            "feedback".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_feedback_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "feedback".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fractureignoresresting(mut self, val: bool) -> Self {
        self.params.insert(
            "fractureignoresresting".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fractureignoresresting_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fractureignoresresting".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_converttopoly(mut self, val: bool) -> Self {
        self.params.insert(
            "converttopoly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_converttopoly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "converttopoly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVoronoifracturesolver {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "voronoifracturesolver"
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
pub enum DopVortexforceParmopRadius {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceParmopVel {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceParmopVeltype {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceVeltype {
    TangentialVelocity = 0,
    AngularVelocity = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceParmopDirection {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceParmopDirectiontype {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceDirectiontype {
    UseNextVertex = 0,
    UsePreviousVertex = 1,
    UseBothVertices = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceParmopMaxdistance {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceParmopLiftforce {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceParmopLiftmult {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceParmopFalloff {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceParmopDensity {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceParmopDragconstant {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceParmopSamplemode {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceSamplemode {
    Default = 0,
    Point = 1,
    Circle = 2,
    Sphere = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopVortexforceSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopVortexforce {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopVortexforce {
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

    // --- Float parameters ---
    pub fn with_liftmult(mut self, val: f32) -> Self {
        self.params.insert(
            "liftmult".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_liftmult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "liftmult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloff(mut self, val: f32) -> Self {
        self.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dragconstant(mut self, val: f32) -> Self {
        self.params.insert(
            "dragconstant".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dragconstant_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dragconstant".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_clrcontrol(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "clrcontrol".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_clrcontrol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clrcontrol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clrinterp(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "clrinterp".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_clrinterp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clrinterp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_divisions(mut self, val: i32) -> Self {
        self.params.insert(
            "divisions".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_divisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_radius(mut self, val: DopVortexforceParmopRadius) -> Self {
        self.params.insert(
            "parmop_radius".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_radius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_vel(mut self, val: DopVortexforceParmopVel) -> Self {
        self.params.insert(
            "parmop_vel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_veltype(mut self, val: DopVortexforceParmopVeltype) -> Self {
        self.params.insert(
            "parmop_veltype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_veltype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_veltype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_veltype(mut self, val: DopVortexforceVeltype) -> Self {
        self.params.insert(
            "veltype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_veltype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "veltype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_direction(mut self, val: DopVortexforceParmopDirection) -> Self {
        self.params.insert(
            "parmop_direction".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_direction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_directiontype(mut self, val: DopVortexforceParmopDirectiontype) -> Self {
        self.params.insert(
            "parmop_directiontype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_directiontype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_directiontype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_directiontype(mut self, val: DopVortexforceDirectiontype) -> Self {
        self.params.insert(
            "directiontype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_directiontype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "directiontype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_maxdistance(mut self, val: DopVortexforceParmopMaxdistance) -> Self {
        self.params.insert(
            "parmop_maxdistance".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_maxdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_maxdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_liftforce(mut self, val: DopVortexforceParmopLiftforce) -> Self {
        self.params.insert(
            "parmop_liftforce".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_liftforce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_liftforce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_liftmult(mut self, val: DopVortexforceParmopLiftmult) -> Self {
        self.params.insert(
            "parmop_liftmult".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_liftmult_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_liftmult".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_falloff(mut self, val: DopVortexforceParmopFalloff) -> Self {
        self.params.insert(
            "parmop_falloff".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_falloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_falloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_density(mut self, val: DopVortexforceParmopDensity) -> Self {
        self.params.insert(
            "parmop_density".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_density".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_dragconstant(mut self, val: DopVortexforceParmopDragconstant) -> Self {
        self.params.insert(
            "parmop_dragconstant".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_dragconstant_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_dragconstant".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_samplemode(mut self, val: DopVortexforceParmopSamplemode) -> Self {
        self.params.insert(
            "parmop_samplemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_samplemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_samplemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samplemode(mut self, val: DopVortexforceSamplemode) -> Self {
        self.params.insert(
            "samplemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_samplemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopVortexforceDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopVortexforceSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_radius(mut self, val: &str) -> Self {
        self.params.insert(
            "radius".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_vel(mut self, val: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_direction(mut self, val: &str) -> Self {
        self.params.insert(
            "direction".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxdistance(mut self, val: &str) -> Self {
        self.params.insert(
            "maxdistance".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maxdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_liftforce(mut self, val: &str) -> Self {
        self.params.insert(
            "liftforce".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_liftforce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "liftforce".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showmaxorbits(mut self, val: bool) -> Self {
        self.params.insert(
            "showmaxorbits".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showmaxorbits_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showmaxorbits".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopVortexforce {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "vortexforce"
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
