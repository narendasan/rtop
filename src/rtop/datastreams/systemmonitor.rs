extern crate sysinfo;

use std::str;
use std::collections::HashMap;
use self::sysinfo::{Pid, Disk, Processor, Process, System, ProcessExt,
                    SystemExt, DiskExt, ProcessorExt, AsU32};

pub struct SystemMonitor<'a> {
    pub process_info: Vec<(u32, &'a str, f32, u64)>, //PID, Command, CPU. mem (kb)
    pub cpu_info: Vec<(&'a str, f32)>, //Name, Usage
    pub cpu_usage_history: HashMap<&'a str, Vec<f32>>, //Name, Usage
    pub disk_info: Vec<(&'a str, &'a str, u64, u64)>, //Name, type, available, total
    pub memory_usage: u64,
    pub memory_usage_history: Vec<f64>, //Name, Usage
    pub total_memory: u64,
    pub swap_usage: u64,
    pub total_swap: u64,
    system_info: System,
    processes: Option<&'a HashMap<Pid, Process>>,
    disk_list: Option<&'a[Disk]>, //&'a Vec<&'a DiskInfo<'a>>,
    cpu_list: Option<&'a[Processor]>, //&'a Vec<&'a CPUCore<'a>>,
    max_history_len: usize,
    //networkUsage:
}

impl <'a> SystemMonitor<'a> {
    pub fn new(max_hist_len: usize) -> Self {        
        Self {
            process_info: Vec::new(),
            cpu_info: Vec::new(),
            cpu_usage_history: HashMap::new(),
            disk_info: Vec::new(),
            memory_usage: 0,
            memory_usage_history: Vec::new(),
            total_memory: 0,
            swap_usage: 0,
            total_swap: 0,
            system_info: System::new(),
            processes: None,
            disk_list: None, 
            cpu_list: None,
            max_history_len: max_hist_len,
        }
    }

    pub fn poll(&'a mut self) {
        self.system_info.refresh_all();
        self.processes = Some(self.system_info.get_process_list());
        self.disk_list = Some(self.system_info.get_disks());
        self.cpu_list = Some(self.system_info.get_processor_list());

        self.disk_info.clear();
        match self.disk_list {
            Some(disks) => for disk in disks {
                self.disk_info.push(SystemMonitor::parse_disk_info(disk));
            },
            None => {},
        }

        self.cpu_info.clear();
        match self.cpu_list {
            Some(cpus) => { 
                    for cpu in cpus {
                        let info = SystemMonitor::parse_cpu_info(cpu);
                        self.cpu_info.push(info);
                        let history = self.cpu_usage_history.entry(info.0).or_insert(Vec::new());
                        while history.len() >= self.max_history_len {
                            history.remove(0);
                        }
                        history.push(info.1);
                    }
            },
            None => {},
        }

        self.process_info.clear();
        match self.processes {
            Some(processes) => for (pid, process) in processes {
                self.process_info.push(SystemMonitor::parse_process_info(pid, process));
            },
            None => {},
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

    fn parse_disk_info(disk: &'a Disk) -> (&'a str, &'a str, u64, u64) {
        let name = disk.get_name().to_str().expect("Optional Disk name returned None"); 
        let fs = str::from_utf8(disk.get_file_system()).unwrap();
        (name, fs, disk.get_available_space(), disk.get_total_space())
    }

    fn parse_cpu_info(cpu: &'a Processor) -> (&'a str, f32) {
        (cpu.get_name(), cpu.get_cpu_usage())
    }

    fn parse_process_info(pid: &'a Pid, process: &'a Process) -> (u32, &'a str, f32, u64) {
        (pid.as_u32(), process.name(), process.cpu_usage(), process.memory())
    }
}