extern crate sysinfo;

use std::collections::HashMap;
use self::sysinfo::{System, SystemExt, Processor, ProcessorExt};

use rtop::datastreams::datastream::DataStream;


pub struct CPUMonitor {
    pub cpu_usage: f32,
    pub cpu_core_info: Vec<(String, f32)>, //Name, Usage
    pub cpu_usage_history: HashMap<String, Vec<f32>>, //Name, Usage
    pub cpu_temp: Option<f32>, 
    system_info: System,
    max_history_len: usize,
}

impl DataStream for CPUMonitor {
    fn new(max_hist_len: usize) -> Self {        
        Self {
            cpu_usage: 0.0,
            cpu_core_info: Vec::new(),
            cpu_usage_history: HashMap::new(),
            cpu_temp: None,
            system_info: System::new(),
            max_history_len: max_hist_len, 
        }
    }

    fn poll(&mut self, system_info: &System) {
        let cpus = system_info.get_processor_list();

        self.cpu_core_info.clear();
        for cpu in &cpus[1..cpus.len()] {
            let info = CPUMonitor::parse_cpu_info(cpu);
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
    }
}

impl CPUMonitor {
    fn parse_cpu_info(cpu: &Processor) -> (String, f32) {
        (String::from(cpu.get_name()), cpu.get_cpu_usage())
    }
}