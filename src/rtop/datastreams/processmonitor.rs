use sysinfo::{Pid, AsU32, Process, System, ProcessExt, SystemExt};
use crate::rtop::datastreams::datastream::SysDataStream;

pub struct ProcessMonitor {
    pub processes: Vec<(u32, String, f32, u64)>, //PID, Command, CPU. mem (kb)
    max_history_len: usize,
}

impl SysDataStream for ProcessMonitor {
    fn new(max_hist_len: usize, inter_len: u16) -> Self {        
        Self {
            processes: Vec::new(),
            max_history_len: max_hist_len,
        }
    }

    fn poll(&mut self, system_info: &System) {
        let processes = system_info.get_process_list();
        self.processes.clear();
        for (pid, process) in processes {
            self.processes.push(ProcessMonitor::parse_process_info(pid, process));
        }
    }
}

impl ProcessMonitor {
    fn parse_process_info(pid: &Pid, process: &Process) -> (u32, String, f32, u64) {
        (pid.as_u32(), String::from(process.name()), process.cpu_usage(), process.memory())
    }
}