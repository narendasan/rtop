extern crate sysinfo;

use std::str;
use std::collections::HashMap;
use self::sysinfo::{Pid, Disk, Processor, Process, System, ProcessExt,
                    SystemExt, DiskExt, ProcessorExt, AsU32};

use rtop::datastreams::datastream::DataStream;

pub struct SystemMonitor {
    pub process_info: Vec<(u32, String, f32, u64)>, //PID, Command, CPU. mem (kb)
    pub cpu_usage: f32,
    pub cpu_core_info: Vec<(String, f32)>, //Name, Usage
    pub cpu_usage_history: HashMap<String, Vec<f32>>, //Name, Usage
    pub disk_info: Vec<(String, String, u64, u64)>, //Name, type, available, total
    pub memory_usage: u64,
    pub memory_usage_history: Vec<f64>, //Name, Usage
    pub total_memory: u64,
    pub swap_usage: u64,
    pub total_swap: u64,
    system_info: System,
    max_history_len: usize,
    //networkUsage:
}

impl DataStream for SystemMonitor {
    fn new(max_hist_len: usize) -> Self {        
        Self {
            process_info: Vec::new(),
            cpu_usage: 0.0,
            cpu_core_info: Vec::new(),
            cpu_usage_history: HashMap::new(),
            disk_info: Vec::new(),
            memory_usage: 0,
            memory_usage_history: Vec::new(),
            total_memory: 0,
            swap_usage: 0,
            total_swap: 0,
            system_info: System::new(),
            max_history_len: max_hist_len,
        }
    }

    fn poll(&mut self) {
        self.system_info.refresh_all();

        let processes = self.system_info.get_process_list();
        let disks = self.system_info.get_disks();
        let cpus = self.system_info.get_processor_list();
        
        self.disk_info.clear();
        for disk in disks {
            self.disk_info.push(SystemMonitor::parse_disk_info(disk));
        }

        self.cpu_core_info.clear();
        for cpu in &cpus[1..cpus.len()] {
            let info = SystemMonitor::parse_cpu_info(cpu);
            self.cpu_core_info.push(info);
            match self.cpu_core_info.last() {
                Some(entry) => {
                    let history = self.cpu_usage_history.entry(entry.0.clone()).or_insert(vec![0.0; self.max_history_len]);
                    while history.len() >= self.max_history_len {
                        history.remove(0);
                    }
                    history.push(entry.1);
                },
                None => {},
            };
        }
        self.cpu_usage = cpus[0].get_cpu_usage();

        self.process_info.clear();
        for (pid, process) in processes {
            self.process_info.push(SystemMonitor::parse_process_info(pid, process));
        }

        while self.memory_usage_history.len() >= self.max_history_len {
            self.memory_usage_history.remove(0);
        }
        self.memory_usage_history.push(self.system_info.get_used_memory() as f64 / self.system_info.get_total_memory() as f64);

        self.memory_usage = self.system_info.get_used_memory();
        self.total_memory = self.system_info.get_total_memory();
        self.swap_usage = self.system_info.get_used_swap();
        self.total_swap = self.system_info.get_total_swap();
    }
}

impl SystemMonitor {
    fn parse_disk_info(disk: &Disk) -> (String, String, u64, u64) {
        let name = disk.get_name().to_str().expect("Optional Disk name returned None"); 
        let fs = str::from_utf8(disk.get_file_system()).unwrap();
        (String::from(name), String::from(fs), disk.get_available_space(), disk.get_total_space())
    }

    fn parse_cpu_info(cpu: &Processor) -> (String, f32) {
        //println!("{:?}", (String::from(cpu.get_name()), cpu.get_cpu_usage()));
        (String::from(cpu.get_name()), cpu.get_cpu_usage())
    }

    fn parse_process_info(pid: &Pid, process: &Process) -> (u32, String, f32, u64) {
        (pid.as_u32(), String::from(process.name()), process.cpu_usage(), process.memory())
    }
}