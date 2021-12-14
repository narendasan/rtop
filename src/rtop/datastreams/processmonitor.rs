use sysinfo::{Pid, AsU32, Process, System, ProcessExt, SystemExt};
use crate::rtop::datastreams::datastream::SysDataStream;

pub struct ProcessMonitor {
    pub processes: Vec<(u32, String, f32, u64)>, //PID, Command, CPU. mem (kb)
}

impl SysDataStream for ProcessMonitor {
    fn new(_max_hist_len: usize, _inter_len: u16) -> Self {
        Self {
            processes: Vec::new(),
        }
    }

    fn poll(&mut self, system_info: &System) {
        let processes = system_info.processes();
        self.processes.clear();
        for (pid, process) in processes {
            self.processes.push(ProcessMonitor::parse_process_info(*pid, process));
        }
    }
}

impl ProcessMonitor {
    fn parse_process_info(pid: Pid, process: &Process) -> (u32, String, f32, u64) {
        (pid.as_u32(), String::from(process.name()), process.cpu_usage(), process.memory())
    }
}
