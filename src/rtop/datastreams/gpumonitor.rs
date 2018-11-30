extern crate nvml_wrapper as nvml;

use std::collections::HashMap;
use nvml::{NVML, Device};
use nvml::enum_wrappers::device::TemperatureSensor;

use rtop::error::Error;
use rtop::datastreams::datastream::GPUDataStream;

pub struct GPUClks {
    pub Graphics: u32,
    pub SM: u32, 
    pub Mem: u32, 
    pub VideoEncDec: u32, 
}

pub struct GPUMonitor {
    pub names: HashMap<u32, String>, //Device ID, Name
    pub temps: HashMap<u32, u32>, //Device ID, Temps
    pub temp_history: HashMap<u32, Vec<u32>>, //Device ID, Temps
    pub memory_usage: HashMap<u32, u64>,
    pub memory_usage_history: HashMap<u32, Vec<u64>>, //Device ID, mem (GB) 
    pub total_memory: HashMap<u32, u64>,
    pub nvml_indices: HashMap<u32, u32>,
    pub clks: HashMap<u32, GPUClks>,
    pub clk_history: HashMap<u32, Vec<GPUClks>>, 
    pub driver_version: String, 
    pub power_usage: HashMap<u32, f32>,
    pub power_usage_history: HashMap<u32, Vec<u32>>,
    pub processes: HashMap<u32, (u32, u64)>, //Device ID, (PID, Mem usage) 
    max_history_len: usize,
}

impl GPUDataStream for GPUMonitor {
    fn new(max_hist_len: usize) -> Self {        
        Self {
            names: HashMap::new(),
            temps: HashMap::new(),
            temp_history: HashMap::new(),
            memory_usage: HashMap::new(),
            memory_usage_history: HashMap::new(),
            total_memory: HashMap::new(),
            nvml_indices: HashMap::new(),
            clks: HashMap::new(),
            clk_history: HashMap::new(),  
            driver_version: "UNKNOWN".to_string(),
            power_usage: HashMap::new(),
            power_usage_history: HashMap::new(),
            processes: HashMap::new(),
            max_history_len: max_hist_len
        }
    }

    fn init(&mut self, nvml: &NVML) -> Result<(), Error> {
        self.driver_version = nvml.sys_driver_version()?;
        let num_gpus = match nvml.device_count() {
            Ok(n) => n,
            Err(e) => 0,
        };
        let gpus: Vec<(u32, Device)> = (0..num_gpus).map(|id| (id, nvml.device_by_index(id).unwrap()))
                                                    .collect();

        for (id, gpu) in &gpus {
            //Static
            self.total_memory.insert(*id, gpu.memory_info()?.total);
            self.nvml_indices.insert(*id, gpu.index()?);
            self.names.insert(*id, gpu.name()?);
            
            //Update 1
            self.memory_usage.insert(*id, gpu.memory_info()?.used);
            self.memory_usage_history.insert(*id, vec![0; self.max_history_len]);
        } 
        
        Ok(())
    }

    fn poll(&mut self, nvml: &NVML) -> Result<(), Error> {
        let num_gpus = match nvml.device_count() {
            Ok(n) => n,
            Err(e) => 0,
        };
        let gpus: Vec<(u32, Device)> = (0..num_gpus).map(|id| (id, nvml.device_by_index(id).unwrap()))
                                                    .collect(); 
        for (id, gpu) in &gpus { 
            let gpu_temp = gpu.temperature(TemperatureSensor::Gpu)?;
            self.temps.insert(*id, gpu_temp);
            let temp_history = self.temp_history.entry(*id).or_insert(vec![0; self.max_history_len]);
            while temp_history.len() >= self.max_history_len {
                temp_history.remove(0);
            }
            temp_history.push(gpu_temp);

            self.memory_usage.insert(*id, gpu.memory_info()?.used); 
        }

        

        println!("{:?}", self.temp_history);

        Ok(())      
    }
}