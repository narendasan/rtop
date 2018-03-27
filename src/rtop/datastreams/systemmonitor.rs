extern crate sysinfo;

use std::str;
use std::collections::HashMap;
use self::sysinfo::{Pid, Disk, Processor, Process, System, ProcessExt,
                    SystemExt, DiskExt, ProcessorExt, NetworkData, NetworkExt, AsU32};

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
    pub swap_usage_history: Vec<f64>,
    pub total_swap: u64,
    pub net_in_history: Vec<u64>,
    pub net_out_history: Vec<u64>, 
    pub net_in: u64,
    pub net_out: u64, 
    system_info: System,
    max_sparkline_len: usize,
    max_history_len: usize,
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
            total_memory: 0,
            memory_usage_history: vec![0.0; max_hist_len],
            swap_usage: 0,
            total_swap: 0,
            swap_usage_history: vec![0.0; max_hist_len],
            net_in_history: Vec::new(),
            net_out_history: Vec::new(), 
            net_in: 0,
            net_out: 0, 
            system_info: System::new(),
            max_sparkline_len: 50,
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

        //println!("{:?}", self.process_info);
        self.memory_usage = self.system_info.get_used_memory();
        self.total_memory = self.system_info.get_total_memory();
        self.swap_usage = self.system_info.get_used_swap();
        self.total_swap = self.system_info.get_total_swap();

        while self.memory_usage_history.len() >= self.max_history_len {
            self.memory_usage_history.remove(0);
        }
        self.memory_usage_history.push(self.memory_usage as f64 / self.total_memory as f64);

        while self.swap_usage_history.len() >= self.max_history_len {
            self.swap_usage_history.remove(0);
        }
        self.swap_usage_history.push(self.swap_usage as f64 / self.total_swap as f64);

        let net = self.system_info.get_network();
        self.net_in = net.get_income();
        self.net_out = net.get_outcome();

        let (inc, out) = SystemMonitor::parse_networking_info((self.net_in, self.net_out));

        while self.net_in_history.len() >= self.max_sparkline_len {
            self.net_in_history.remove(0);
        }
        self.net_in_history.push(inc);

        while self.net_out_history.len() >= self.max_sparkline_len {
            self.net_out_history.remove(0);
        }
        self.net_out_history.push(out);
    }
}

impl SystemMonitor {
    fn parse_disk_info(disk: &Disk) -> (String, String, u64, u64) {
        let name = disk.get_name().to_str().expect("Optional Disk name returned None"); 
        let fs = str::from_utf8(disk.get_file_system()).unwrap();
        (String::from(name), String::from(fs), disk.get_available_space(), disk.get_total_space())
    }

    fn parse_cpu_info(cpu: &Processor) -> (String, f32) {
        (String::from(cpu.get_name()), cpu.get_cpu_usage())
    }

    fn parse_process_info(pid: &Pid, process: &Process) -> (u32, String, f32, u64) {
        (pid.as_u32(), String::from(process.name()), process.cpu_usage() * 100.0, process.memory())
    }

    fn parse_networking_info(net: (u64, u64)) -> (u64, u64) {
        let (mut inc, mut out) = net;
        if inc == 0 {
            inc = 10;
        }
        if out == 0 {
            out = 10;
        }
        (inc, out)
    }
}