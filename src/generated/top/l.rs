#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLocalschedulerMaxprocsmenu {
    /// Equal to 1/4 of Total CPU Count
    EqualTo14OfTotalCpuCount = 0,
    EqualToCpuCountLessOne = 1,
    CustomSlotCount = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLocalschedulerHythonbin {
    Default = 0,
    Custom = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLocalschedulerPdgWorkitemdatasource {
    TemporaryJsonFile = 0,
    RpcMessage = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLocalschedulerPdgMapmode {
    Global = 0,
    None = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLocalschedulerPdgTransfertype {
    FlatCopy = 0,
    RelativeToRoot = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLocalschedulerTempdirmenu {
    WorkingDirectory = 0,
    HoudiniTemp = 1,
    Custom = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLocalschedulerPdgDeletetempdir {
    Never = 0,
    WhenSchedulerIsDeleted = 1,
    WhenCookCompletes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLocalschedulerPdgRpcignoreerrors {
    Never = 0,
    WhenCookingBatches = 1,
    Always = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLocalschedulerLocalUseminfreemem {
    NoMinimum = 0,
    MbAvailable = 1,
    PercentAvailable = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLocalschedulerLocalHandletimeout {
    MarkAsFailed = 0,
    MarkAsSucceeded = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLocalschedulerLocalHandlememory {
    MarkAsFailed = 0,
    MarkAsSucceeded = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLocalschedulerLocalEchandleby {
    ReportError = 0,
    ReportWarning = 1,
    RetryTask = 2,
    Ignore = 3,
}

#[derive(Debug, Clone)]
pub struct TopLocalscheduler {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopLocalscheduler {
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
    pub fn with_pdg_rpcbatch(mut self, val: f32) -> Self {
        self.params.insert(
            "pdg_rpcbatch".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pdg_rpcbatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcbatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_minfreemem(mut self, val: f32) -> Self {
        self.params.insert(
            "local_minfreemem".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_local_minfreemem_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_minfreemem".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_minfreemempct(mut self, val: f32) -> Self {
        self.params.insert(
            "local_minfreemempct".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_local_minfreemempct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_minfreemempct".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_maxtime(mut self, val: f32) -> Self {
        self.params.insert(
            "local_maxtime".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_local_maxtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_maxtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_maxmemory(mut self, val: f32) -> Self {
        self.params.insert(
            "local_maxmemory".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_local_maxmemory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_maxmemory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxprocs(mut self, val: i32) -> Self {
        self.params.insert(
            "maxprocs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxprocs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxprocs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_maxtasks(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_maxtasks".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_maxtasks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_maxtasks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_socketcallbacks(mut self, val: i32) -> Self {
        self.params.insert(
            "socketcallbacks".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_socketcallbacks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "socketcallbacks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcmaxerrors(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_rpcmaxerrors".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpcmaxerrors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcmaxerrors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpctimeout(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_rpctimeout".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpctimeout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpctimeout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcretries(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_rpcretries".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpcretries_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcretries".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcbackoff(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_rpcbackoff".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpcbackoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcbackoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_cpus_to_use(mut self, val: i32) -> Self {
        self.params.insert(
            "local_CPUs_to_use".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_local_cpus_to_use_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_CPUs_to_use".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_eccustomcode(mut self, val: i32) -> Self {
        self.params.insert(
            "local_eccustomcode".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_local_eccustomcode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_eccustomcode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_maximumretries(mut self, val: i32) -> Self {
        self.params.insert(
            "local_maximumretries".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_local_maximumretries_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_maximumretries".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_houdinimaxthreads(mut self, val: i32) -> Self {
        self.params.insert(
            "local_houdinimaxthreads".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_local_houdinimaxthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_houdinimaxthreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_maxprocsmenu(mut self, val: TopLocalschedulerMaxprocsmenu) -> Self {
        self.params.insert(
            "maxprocsmenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maxprocsmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxprocsmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hythonbin(mut self, val: TopLocalschedulerHythonbin) -> Self {
        self.params.insert(
            "hythonbin".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hythonbin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hythonbin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemdatasource(
        mut self,
        val: TopLocalschedulerPdgWorkitemdatasource,
    ) -> Self {
        self.params.insert(
            "pdg_workitemdatasource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemdatasource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemdatasource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_mapmode(mut self, val: TopLocalschedulerPdgMapmode) -> Self {
        self.params.insert(
            "pdg_mapmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_mapmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_mapmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_transfertype(mut self, val: TopLocalschedulerPdgTransfertype) -> Self {
        self.params.insert(
            "pdg_transfertype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_transfertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_transfertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tempdirmenu(mut self, val: TopLocalschedulerTempdirmenu) -> Self {
        self.params.insert(
            "tempdirmenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tempdirmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tempdirmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_deletetempdir(mut self, val: TopLocalschedulerPdgDeletetempdir) -> Self {
        self.params.insert(
            "pdg_deletetempdir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_deletetempdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_deletetempdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcignoreerrors(mut self, val: TopLocalschedulerPdgRpcignoreerrors) -> Self {
        self.params.insert(
            "pdg_rpcignoreerrors".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_rpcignoreerrors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcignoreerrors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_useminfreemem(mut self, val: TopLocalschedulerLocalUseminfreemem) -> Self {
        self.params.insert(
            "local_useminfreemem".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_local_useminfreemem_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_useminfreemem".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_handletimeout(mut self, val: TopLocalschedulerLocalHandletimeout) -> Self {
        self.params.insert(
            "local_handletimeout".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_local_handletimeout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_handletimeout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_handlememory(mut self, val: TopLocalschedulerLocalHandlememory) -> Self {
        self.params.insert(
            "local_handlememory".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_local_handlememory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_handlememory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_echandleby(mut self, val: TopLocalschedulerLocalEchandleby) -> Self {
        self.params.insert(
            "local_echandleby".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_local_echandleby_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_echandleby".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pdg_workingdir(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_workingdir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_workingdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workingdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_logfilename(mut self, val: &str) -> Self {
        self.params.insert(
            "logfilename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_logfilename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "logfilename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hythonbincustomuniversal(mut self, val: &str) -> Self {
        self.params.insert(
            "hythonbincustomuniversal".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hythonbincustomuniversal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hythonbincustomuniversal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_mapzone(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_mapzone".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_mapzone_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_mapzone".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_transferroot(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_transferroot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_transferroot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_transferroot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tempdircustom(mut self, val: &str) -> Self {
        self.params.insert(
            "tempdircustom".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tempdircustom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tempdircustom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_failcountattr(mut self, val: &str) -> Self {
        self.params.insert(
            "local_failcountattr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_local_failcountattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_failcountattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_envunset(mut self, val: &str) -> Self {
        self.params.insert(
            "local_envunset".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_local_envunset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_envunset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_env_file(mut self, val: &str) -> Self {
        self.params.insert(
            "local_env_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_local_env_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_env_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_envname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("local_envname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_local_envname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("local_envname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_envvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("local_envvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_local_envvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("local_envvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_pdg_usemaxtasks(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_usemaxtasks".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_usemaxtasks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_usemaxtasks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbose(mut self, val: bool) -> Self {
        self.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Toggle(val),
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
    pub fn with_pdg_waitforfailures(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_waitforfailures".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_waitforfailures_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_waitforfailures".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uselogfilename(mut self, val: bool) -> Self {
        self.params.insert(
            "uselogfilename".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselogfilename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uselogfilename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_compressworkitemdata(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_compressworkitemdata".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_compressworkitemdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_compressworkitemdata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_validateoutputs(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_validateoutputs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_validateoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_validateoutputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_checkexpectedoutputs(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_checkexpectedoutputs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_checkexpectedoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_checkexpectedoutputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_usemapzone(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_usemapzone".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_usemapzone_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_usemapzone".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tempdirappendpid(mut self, val: bool) -> Self {
        self.params.insert(
            "tempdirappendpid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tempdirappendpid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tempdirappendpid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcrelease(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_rpcrelease".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_rpcrelease_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcrelease".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_single(mut self, val: bool) -> Self {
        self.params.insert(
            "local_single".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_local_single_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_single".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_is_cpu_number_set(mut self, val: bool) -> Self {
        self.params.insert(
            "local_is_CPU_number_set".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_local_is_cpu_number_set_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_is_CPU_number_set".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_enabletimeout(mut self, val: bool) -> Self {
        self.params.insert(
            "local_enabletimeout".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_local_enabletimeout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_enabletimeout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_enablemaxmemory(mut self, val: bool) -> Self {
        self.params.insert(
            "local_enablemaxmemory".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_local_enablemaxmemory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_enablemaxmemory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_echandleall(mut self, val: bool) -> Self {
        self.params.insert(
            "local_echandleall".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_local_echandleall_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_echandleall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_addfailcountattr(mut self, val: bool) -> Self {
        self.params.insert(
            "local_addfailcountattr".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_local_addfailcountattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_addfailcountattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_usehoudinimaxthreads(mut self, val: bool) -> Self {
        self.params.insert(
            "local_usehoudinimaxthreads".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_local_usehoudinimaxthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_usehoudinimaxthreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_requireswindow(mut self, val: bool) -> Self {
        self.params.insert(
            "local_requireswindow".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_local_requireswindow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_requireswindow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_local_skippackages(mut self, val: bool) -> Self {
        self.params.insert(
            "local_skippackages".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_local_skippackages_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "local_skippackages".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopLocalscheduler {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "localscheduler"
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
pub struct TopLopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopLopnet {
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

    // --- Int parameters ---
    pub fn with_modifiedprimcounttostartnewlayer(mut self, val: i32) -> Self {
        self.params.insert(
            "modifiedprimcounttostartnewlayer".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_modifiedprimcounttostartnewlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modifiedprimcounttostartnewlayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_expansioneffect(mut self, val: &str) -> Self {
        self.params.insert(
            "expansioneffect".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_expansioneffect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expansioneffect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pinnedprims(mut self, val: &str) -> Self {
        self.params.insert(
            "pinnedprims".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pinnedprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pinnedprims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolvercontextassetpath(mut self, val: &str) -> Self {
        self.params.insert(
            "resolvercontextassetpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resolvercontextassetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolvercontextassetpath".to_string(),
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
    pub fn with_insertionpointdescriptor(mut self, val: &str) -> Self {
        self.params.insert(
            "insertionpointdescriptor".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_insertionpointdescriptor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "insertionpointdescriptor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rendergallerysource(mut self, val: &str) -> Self {
        self.params.insert(
            "rendergallerysource".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rendergallerysource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendergallerysource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
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
}

impl crate::core::types::HoudiniNode for TopLopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "lopnet"
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
