extern crate nvml_wrapper as nvml;

use std::collections::HashMap;
use nvml::{NVML, Device};

use rtop::datastreams::datastream::DataStream;


pub struct GPUMonitor {
    pub cpu_usage: f32,
    pub cpu_core_info: Vec<(String, f32)>, //Name, Usage
    pub cpu_usage_history: HashMap<String, Vec<f32>>, //Name, Usage
    pub cpu_temp: Option<f32>, 
    system_info: System,
    max_history_len: usize,
}

pub struct GPUMonitor {
    pub names: HashMap<u8, String>, //Device ID, Name
    pub temps: HashMap<u8, Vec<f32>>, //Device ID, Temps
    pub memory: HashMap<u8, Vec<f32>>, //Device ID, mem (GB) 
    pub total_memory: HashMap<u8, f32>,
    pub nvml_indices: HashMap<u8, u32>,
    pub clks: HashMap<u8, (i32, i32)>  
    pub driver_version: String, 
    pub power_usage: HashMap<u8, f32> 
    max_history_len: usize,
}

impl DataStream for GPUMonitor {
    fn new(nvml: &NVML, max_hist_len: usize) -> Self {        
        Self {
            names: HashMap::new(),
            temps: HashMap::new(),
            memory: HashMap::new(),
            total_memory: HashMap::new(),
            nvml_indices: HashMap::new(),
            clks: HashMap::new(),  
            driver_version: nvml.sys_driver_version(),
            power_usage: HashMap::new(),
            max_hist_len: max_hist_len
        }
    }

    fn poll(&mut self, nvml: &NVML) {
        let num_gpus = (0..nvml.device_count()).collect(); 
        println!(num_gpus);
    }
}

impl GPUMonitor {
    fn parse_gpu_info(cpu: &Processor) -> (String, f32) {
        (String::from(gpu.get_name()), gpu.get_cpu_usage())
    }
}