#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopDeadlineschedulerPdgWorkitemdatasource {
    TemporaryJsonFile = 0,
    RpcMessage = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopDeadlineschedulerPdgDeletetempdir {
    Never = 0,
    WhenSchedulerIsDeleted = 1,
    WhenCookCompletes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopDeadlineschedulerPdgMapmode {
    Global = 0,
    None = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopDeadlineschedulerPdgTransfertype {
    FlatCopy = 0,
    RelativeToRoot = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopDeadlineschedulerSubmitjobverbosity {
    None = 0,
    ErrorsOnly = 1,
    Full = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopDeadlineschedulerUsedatalayerport {
    Automatic = 0,
    Custom = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopDeadlineschedulerSubmitjobwhenfinished {
    Terminate = 0,
    KeepRunningIfError = 1,
    KeepRunning = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopDeadlineschedulerMqusage {
    Farm = 0,
    Local = 1,
    Connect = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopDeadlineschedulerPdgRpcignoreerrors {
    Never = 0,
    WhenCookingBatches = 1,
    Always = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopDeadlineschedulerHythonbin {
    Default = 0,
    Custom = 1,
}

#[derive(Debug, Clone)]
pub struct TopDeadlinescheduler {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopDeadlinescheduler {
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
    pub fn trigger_submitjob(mut self) -> Self {
        self.params.insert(
            "submitjob".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_deadline_submitcommandtimeout(mut self, val: f32) -> Self {
        self.params.insert(
            "deadline_submitcommandtimeout".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_deadline_submitcommandtimeout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_submitcommandtimeout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

    // --- Int parameters ---
    pub fn with_task_file_timeout(mut self, val: i32) -> Self {
        self.params.insert(
            "task_file_timeout".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_task_file_timeout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "task_file_timeout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_tasktimeout(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_tasktimeout".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_tasktimeout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_tasktimeout".to_string(),
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
    pub fn with_deadline_max_submit_tasks(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_max_submit_tasks".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_max_submit_tasks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_max_submit_tasks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_max_check_tasks(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_max_check_tasks".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_max_check_tasks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_max_check_tasks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_maxsubmitjobs(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_maxsubmitjobs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_maxsubmitjobs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_maxsubmitjobs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_maxcheckjobs(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_maxcheckjobs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_maxcheckjobs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_maxcheckjobs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_maxsubmittasks(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_maxsubmittasks".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_maxsubmittasks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_maxsubmittasks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_maxchecktasks(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_maxchecktasks".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_maxchecktasks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_maxchecktasks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_job_priority(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_job_priority".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_job_priority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_job_priority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_jobpriority(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_jobpriority".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_jobpriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_jobpriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_concurrent_tasks(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_concurrent_tasks".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_concurrent_tasks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_concurrent_tasks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_concurrenttasks(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_concurrenttasks".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_concurrenttasks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_concurrenttasks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_machine_limit(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_machine_limit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_machine_limit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_machine_limit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_machinelimit(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_machinelimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_machinelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_machinelimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_datalayerserverport(mut self, val: i32) -> Self {
        self.params.insert(
            "datalayerserverport".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_datalayerserverport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "datalayerserverport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mqusage(mut self, val: TopDeadlineschedulerMqusage) -> Self {
        self.params.insert(
            "mqusage".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_mqusage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mqusage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_taskcallbackport(mut self, val: i32) -> Self {
        self.params.insert(
            "taskcallbackport".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_taskcallbackport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskcallbackport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mqrelayport(mut self, val: i32) -> Self {
        self.params.insert(
            "mqrelayport".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mqrelayport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mqrelayport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_mqjobpriority(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_mqjobpriority".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_mqjobpriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_mqjobpriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_mqmachinelimit(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_mqmachinelimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_mqmachinelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_mqmachinelimit".to_string(),
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
    pub fn with_deadline_houdinimaxthreads(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_houdinimaxthreads".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_houdinimaxthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_houdinimaxthreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_gpuspertask(mut self, val: i32) -> Self {
        self.params.insert(
            "deadline_gpuspertask".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_deadline_gpuspertask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_gpuspertask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemdatasource(
        mut self,
        val: TopDeadlineschedulerPdgWorkitemdatasource,
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
    pub fn with_pdg_deletetempdir(mut self, val: TopDeadlineschedulerPdgDeletetempdir) -> Self {
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
    pub fn with_pdg_mapmode(mut self, val: TopDeadlineschedulerPdgMapmode) -> Self {
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
    pub fn with_pdg_transfertype(mut self, val: TopDeadlineschedulerPdgTransfertype) -> Self {
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
    pub fn with_submitjobverbosity(mut self, val: TopDeadlineschedulerSubmitjobverbosity) -> Self {
        self.params.insert(
            "submitjobverbosity".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_submitjobverbosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobverbosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedatalayerport(mut self, val: TopDeadlineschedulerUsedatalayerport) -> Self {
        self.params.insert(
            "usedatalayerport".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_usedatalayerport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedatalayerport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobwhenfinished(
        mut self,
        val: TopDeadlineschedulerSubmitjobwhenfinished,
    ) -> Self {
        self.params.insert(
            "submitjobwhenfinished".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_submitjobwhenfinished_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobwhenfinished".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcignoreerrors(mut self, val: TopDeadlineschedulerPdgRpcignoreerrors) -> Self {
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
    pub fn with_hythonbin(mut self, val: TopDeadlineschedulerHythonbin) -> Self {
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

    // --- String parameters ---
    pub fn with_deadline_launch_monitor(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_launch_monitor".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_launch_monitor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_launch_monitor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_launchmonitor(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_launchmonitor".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_launchmonitor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_launchmonitor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_repository(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_repository".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_repository_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_repository".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_connection_type(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_connection_type".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_connection_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_connection_type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_connectiontype(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_connectiontype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_connectiontype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_connectiontype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_plugin(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_plugin".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_plugin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_plugin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_plugin_directory(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_plugin_directory".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_plugin_directory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_plugin_directory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_plugindirectory(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_plugindirectory".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_plugindirectory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_plugindirectory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_remotesharedroot(mut self, val: &str) -> Self {
        self.params.insert(
            "remotesharedroot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_remotesharedroot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remotesharedroot".to_string(),
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
    pub fn with_deadline_job_name(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_job_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_job_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_job_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_jobbatchname(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_jobbatchname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_jobbatchname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_jobbatchname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_jobname(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_jobname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_jobname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_jobname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_job_comment(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_job_comment".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_job_comment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_job_comment".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_jobcomment(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_jobcomment".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_jobcomment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_jobcomment".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_job_dept(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_job_dept".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_job_dept_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_job_dept".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_jobdept(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_jobdept".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_jobdept_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_jobdept".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_job_batch_name(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_job_batch_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_job_batch_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_job_batch_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_job_pool(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_job_pool".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_job_pool_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_job_pool".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_jobpool(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_jobpool".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_jobpool_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_jobpool".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_job_group(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_job_group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_job_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_job_group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_jobgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_jobgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_jobgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_jobgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_machine_list(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_machine_list".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_machine_list_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_machine_list".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_machinelist(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_machinelist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_machinelist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_machinelist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_on_job_complete(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_on_job_complete".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_on_job_complete_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_on_job_complete".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_limits(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_limits".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_limits_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_limits".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_onjobcomplete(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_onjobcomplete".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_onjobcomplete_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_onjobcomplete".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_extrainfokey_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("deadline_extrainfokey{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_extrainfokey_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("deadline_extrainfokey{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_extrainfovalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("deadline_extrainfovalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_extrainfovalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("deadline_extrainfovalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_jobfile_key_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("deadline_jobfile_key{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_jobfile_key_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("deadline_jobfile_key{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_jobfile_value_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("deadline_jobfile_value{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_jobfile_value_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("deadline_jobfile_value{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_jobfilekey_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("deadline_jobfilekey{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_jobfilekey_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("deadline_jobfilekey{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_jobfilevalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("deadline_jobfilevalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_jobfilevalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("deadline_jobfilevalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_pluginfile_key_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("deadline_pluginfile_key{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_pluginfile_key_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("deadline_pluginfile_key{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_pluginfile_value_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("deadline_pluginfile_value{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_pluginfile_value_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("deadline_pluginfile_value{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_pluginfilekey_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("deadline_pluginfilekey{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_pluginfilekey_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("deadline_pluginfilekey{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_pluginfilevalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("deadline_pluginfilevalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_pluginfilevalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("deadline_pluginfilevalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_submitjobname(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_submitjobname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_submitjobname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_submitjobname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobnode(mut self, val: &str) -> Self {
        self.params.insert(
            "submitjobnode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_submitjobnode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobnode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remotegraphname(mut self, val: &str) -> Self {
        self.params.insert(
            "remotegraphname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_remotegraphname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remotegraphname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mqaddr(mut self, val: &str) -> Self {
        self.params.insert(
            "mqaddr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mqaddr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mqaddr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_mqjobbatchname(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_mqjobbatchname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_mqjobbatchname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_mqjobbatchname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_mqjobname(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_mqjobname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_mqjobname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_mqjobname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_mqjobcomment(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_mqjobcomment".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_mqjobcomment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_mqjobcomment".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_mqjobdept(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_mqjobdept".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_mqjobdept_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_mqjobdept".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_mqjobpool(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_mqjobpool".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_mqjobpool_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_mqjobpool".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_mqjobgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_mqjobgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_mqjobgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_mqjobgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_mqmachinelist(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_mqmachinelist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_mqmachinelist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_mqmachinelist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_mqlimits(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_mqlimits".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_mqlimits_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_mqlimits".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_mqonjobcomplete(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_mqonjobcomplete".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_mqonjobcomplete_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_mqonjobcomplete".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_hfs(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_hfs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_hfs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_hfs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_python(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_python".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_python_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_python".to_string(),
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
    pub fn with_deadline_prebatchscript(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_prebatchscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_prebatchscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_prebatchscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_postbatchscript(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_postbatchscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_postbatchscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_postbatchscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_prejobscript(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_prejobscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_prejobscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_prejobscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_postjobscript(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_postjobscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_postjobscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_postjobscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_pretaskscript(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_pretaskscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_pretaskscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_pretaskscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_posttaskscript(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_posttaskscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_posttaskscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_posttaskscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_pre_task_script(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_pre_task_script".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_pre_task_script_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_pre_task_script".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_post_task_script(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_post_task_script".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_post_task_script_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_post_task_script".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_post_job_script(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_post_job_script".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_post_job_script_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_post_job_script".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_pre_job_script(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_pre_job_script".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_pre_job_script_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_pre_job_script".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_envunset(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_envunset".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_envunset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_envunset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_env_file(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_env_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_env_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_env_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_envname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("deadline_envname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_envname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("deadline_envname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_envvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("deadline_envvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_envvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("deadline_envvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_cmdenvunset(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_cmdenvunset".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_cmdenvunset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_cmdenvunset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_cmdenvname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("deadline_cmdenvname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_cmdenvname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("deadline_cmdenvname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_cmdenvvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("deadline_cmdenvvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_cmdenvvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("deadline_cmdenvvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_gpusselectdevices(mut self, val: &str) -> Self {
        self.params.insert(
            "deadline_gpusselectdevices".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_deadline_gpusselectdevices_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_gpusselectdevices".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_deadline_verbose_log(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_verbose_log".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_verbose_log_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_verbose_log".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_verboselog(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_verboselog".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_verboselog_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_verboselog".to_string(),
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
    pub fn with_deadline_ignoreexitcode(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_ignoreexitcode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_ignoreexitcode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_ignoreexitcode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_pdgmq_use_ip(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_pdgmq_use_ip".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_pdgmq_use_ip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_pdgmq_use_ip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_pdgmq_as_task(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_pdgmq_as_task".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_pdgmq_as_task_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_pdgmq_as_task".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_force_reload_plugin(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_force_reload_plugin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_force_reload_plugin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_force_reload_plugin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_forcereloadplugin(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_forcereloadplugin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_forcereloadplugin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_forcereloadplugin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_usetasktimeout(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_usetasktimeout".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_usetasktimeout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_usetasktimeout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_deadline_scheduleworkitemsasjobs(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_scheduleworkitemsasjobs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_scheduleworkitemsasjobs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_scheduleworkitemsasjobs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_override_repo(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_override_repo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_override_repo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_override_repo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_overriderepo(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_overriderepo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_overriderepo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_overriderepo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_overrideplugin(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_overrideplugin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_overrideplugin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_overrideplugin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_copyplugin(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_copyplugin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_copyplugin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_copyplugin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overrideremoterootpath(mut self, val: bool) -> Self {
        self.params.insert(
            "overrideremoterootpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideremoterootpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overrideremoterootpath".to_string(),
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
    pub fn with_deadline_limitconcurrenttasks(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_limitconcurrenttasks".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_limitconcurrenttasks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_limitconcurrenttasks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_machine_list_black(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_machine_list_black".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_machine_list_black_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_machine_list_black".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_machinelistblack(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_machinelistblack".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_machinelistblack_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_machinelistblack".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesubmitjobnode(mut self, val: bool) -> Self {
        self.params.insert(
            "usesubmitjobnode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesubmitjobnode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesubmitjobnode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobfile(mut self, val: bool) -> Self {
        self.params.insert(
            "submitjobfile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_submitjobfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_submitjobusemqspecs(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_submitjobusemqspecs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_submitjobusemqspecs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_submitjobusemqspecs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledatalayerserver(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledatalayerserver".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledatalayerserver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledatalayerserver".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createremotegraph(mut self, val: bool) -> Self {
        self.params.insert(
            "createremotegraph".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createremotegraph_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createremotegraph".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetaskcallbackport(mut self, val: bool) -> Self {
        self.params.insert(
            "usetaskcallbackport".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetaskcallbackport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetaskcallbackport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemqrelayport(mut self, val: bool) -> Self {
        self.params.insert(
            "usemqrelayport".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemqrelayport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemqrelayport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_mqoverridebatchname(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_mqoverridebatchname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_mqoverridebatchname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_mqoverridebatchname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_mqmachinelistblack(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_mqmachinelistblack".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_mqmachinelistblack_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_mqmachinelistblack".to_string(),
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
    pub fn with_deadline_inheritlocalenv(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_inheritlocalenv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_inheritlocalenv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_inheritlocalenv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_usehoudinimaxthreads(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_usehoudinimaxthreads".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_usehoudinimaxthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_usehoudinimaxthreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_cmdinheritlocalenv(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_cmdinheritlocalenv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_cmdinheritlocalenv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_cmdinheritlocalenv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deadline_openclforcegpu(mut self, val: bool) -> Self {
        self.params.insert(
            "deadline_openclforcegpu".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deadline_openclforcegpu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deadline_openclforcegpu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopDeadlinescheduler {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "deadlinescheduler"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum TopDopnetCompresssims {
    NoCompression = 0,
    Blosc = 1,
    Gzip = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopDopnetXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopDopnetRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopDopnetPreXform {
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
pub enum TopDopnetUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct TopDopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopDopnet {
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
    pub fn trigger_resimulate(mut self) -> Self {
        self.params.insert(
            "resimulate".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_timestep(mut self, val: f32) -> Self {
        self.params.insert(
            "timestep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timestep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timestep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "timeoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timeoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeoffset".to_string(),
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
    pub fn with_substep(mut self, val: i32) -> Self {
        self.params.insert(
            "substep".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "substep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_startframe(mut self, val: i32) -> Self {
        self.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_startframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "startframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxfeedback(mut self, val: i32) -> Self {
        self.params.insert(
            "maxfeedback".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxfeedback_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxfeedback".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachemaxsize(mut self, val: i32) -> Self {
        self.params.insert(
            "cachemaxsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cachemaxsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachemaxsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_continuouscook_tick(mut self, val: i32) -> Self {
        self.params.insert(
            "continuouscook_tick".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_continuouscook_tick_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "continuouscook_tick".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_explicitcachensteps(mut self, val: i32) -> Self {
        self.params.insert(
            "explicitcachensteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_explicitcachensteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "explicitcachensteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_explicitcachecheckpointspacing(mut self, val: i32) -> Self {
        self.params.insert(
            "explicitcachecheckpointspacing".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_explicitcachecheckpointspacing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "explicitcachecheckpointspacing".to_string(),
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
    pub fn with_compresssims(mut self, val: TopDopnetCompresssims) -> Self {
        self.params.insert(
            "compresssims".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_compresssims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compresssims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xord(mut self, val: TopDopnetXord) -> Self {
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
    pub fn with_rord(mut self, val: TopDopnetRord) -> Self {
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
    pub fn with_pre_xform(mut self, val: TopDopnetPreXform) -> Self {
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
    pub fn with_uparmtype(mut self, val: TopDopnetUparmtype) -> Self {
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
    pub fn with_playfilesname(mut self, val: &str) -> Self {
        self.params.insert(
            "playfilesname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_playfilesname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "playfilesname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initialstate(mut self, val: &str) -> Self {
        self.params.insert(
            "initialstate".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_initialstate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initialstate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_explicitcachename(mut self, val: &str) -> Self {
        self.params.insert(
            "explicitcachename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_explicitcachename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "explicitcachename".to_string(),
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
    pub fn with_displayfilter(mut self, val: &str) -> Self {
        self.params.insert(
            "displayfilter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_displayfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

    // --- Toggle parameters ---
    pub fn with_isplayer(mut self, val: bool) -> Self {
        self.params.insert(
            "isplayer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_isplayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "isplayer".to_string(),
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
    pub fn with_autoresim(mut self, val: bool) -> Self {
        self.params.insert(
            "autoresim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autoresim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autoresim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_datahints(mut self, val: bool) -> Self {
        self.params.insert(
            "datahints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_datahints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "datahints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interpolate(mut self, val: bool) -> Self {
        self.params.insert(
            "interpolate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_interpolate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interpolate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cacheenabled(mut self, val: bool) -> Self {
        self.params.insert(
            "cacheenabled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cacheenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cacheenabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachetodisk(mut self, val: bool) -> Self {
        self.params.insert(
            "cachetodisk".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cachetodisk_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachetodisk".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachetodisknoninteractive(mut self, val: bool) -> Self {
        self.params.insert(
            "cachetodisknoninteractive".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cachetodisknoninteractive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachetodisknoninteractive".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cachesubsteps(mut self, val: bool) -> Self {
        self.params.insert(
            "cachesubsteps".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cachesubsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cachesubsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeless(mut self, val: bool) -> Self {
        self.params.insert(
            "timeless".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeless_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeless".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_continuouscook(mut self, val: bool) -> Self {
        self.params.insert(
            "continuouscook".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_continuouscook_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "continuouscook".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_explicitcache(mut self, val: bool) -> Self {
        self.params.insert(
            "explicitcache".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_explicitcache_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "explicitcache".to_string(),
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
}

impl crate::core::types::HoudiniNode for TopDopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "dopnet"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
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
pub enum TopDownloadfileDownloadduring {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopDownloadfilePdgCooktype {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
}

#[derive(Debug, Clone)]
pub struct TopDownloadfile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    >,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopDownloadfile {
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(index),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
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
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            crate::core::types::InputPort::Index(0),
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cachemode(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_cachemode".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_cachemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_cachemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_downloadduring(mut self, val: TopDownloadfileDownloadduring) -> Self {
        self.params.insert(
            "downloadduring".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_downloadduring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "downloadduring".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cooktype(mut self, val: TopDownloadfilePdgCooktype) -> Self {
        self.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_url_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("url{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_url_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("url{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_file_tag_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("file_tag{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_file_tag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("file_tag{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output_filename_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("output_filename{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_output_filename_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("output_filename{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_split(mut self, val: bool) -> Self {
        self.params.insert(
            "split".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_split_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "split".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verifysslcerts(mut self, val: bool) -> Self {
        self.params.insert(
            "verifysslcerts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_verifysslcerts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verifysslcerts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_specify_filename_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("specify_filename{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_specify_filename_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("specify_filename{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopDownloadfile {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "downloadfile"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        crate::core::types::InputPort,
        (usize, crate::core::types::OutputPort),
    > {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
