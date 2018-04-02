extern crate sysinfo;

use self::sysinfo::{Pid, AsU32, Process, System, ProcessExt, SystemExt};
use rtop::datastreams::datastream::DataStream;

pub struct ProcessMonitor {
    pub process_info: Vec<(u32, String, f32, u64)>, //PID, Command, CPU. mem (kb)
    max_history_len: usize,
}

impl DataStream for ProcessMonitor {
    fn new(max_hist_len: usize) -> Self {        
        Self {
            process_info: Vec::new(),
            max_history_len: max_hist_len,
        }
    }

    fn poll(&mut self, system_info: &System) {
        let processes = system_info.get_process_list();
        self.process_info.clear();
        for (pid, process) in processes {
            self.process_info.push(ProcessMonitor::parse_process_info(pid, process));
        }
    }
}

impl ProcessMonitor {
    fn parse_process_info(pid: &Pid, process: &Process) -> (u32, String, f32, u64) {
        (pid.as_u32(), String::from(process.name()), process.cpu_usage(), process.memory())
    }
}