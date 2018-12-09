extern crate nvml_wrapper as nvml;
extern crate sysinfo;

use std::collections::HashMap;

use sysinfo::{System, SystemExt, Pid, AsU32, Process, ProcessExt};

use nvml::{NVML, Device};
use nvml::enum_wrappers::device::{TemperatureSensor, Clock, ClockId};
use nvml::enums::device::UsedGpuMemory;
use nvml::struct_wrappers::device::ProcessInfo;

use rtop::error::Error;
use rtop::datastreams::datastream::GPUDataStream;

#[derive(Debug, Clone)]
pub struct GPUClks {
    pub graphics: u32,
    pub sm: u32, 
    pub mem: u32, 
    pub video_enc_dec: u32, 
}

impl GPUClks {
    fn new(gpu: Option<&Device>) -> Result<Self, Error> {
        match gpu {
            Some(gpu) => Ok(Self {
                graphics: gpu.clock(Clock::Graphics, ClockId::Current)?,
                sm: gpu.clock(Clock::SM, ClockId::Current)?,
                mem: gpu.clock(Clock::Memory, ClockId::Current)?,
                video_enc_dec: gpu.clock(Clock::Video, ClockId::Current)?,
            }),
            None => Ok(Self {
                graphics: 0, 
                sm: 0,
                mem: 0, 
                video_enc_dec: 0,
            })      
        }
    }
}

#[derive(Debug, Clone)]
pub enum GPUProcessType {
    Compute,
    Graphics,
    Unknown, 
}

impl ToString for GPUProcessType {
    fn to_string(&self) -> String {
        match self {
            GPUProcessType::Compute => "C".to_string(), 
            GPUProcessType::Graphics => "G".to_string(),
            GPUProcessType::Unknown => "U".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GPUProcess {
    pub device_id: u32, 
    pub pid: u32,
    pub name: String, 
    pub mem: Option<u64>,
    pub proc_type: GPUProcessType
}

impl GPUProcess {
    fn new(proc: Option<&ProcessInfo>, device: u32, proc_type: GPUProcessType) -> Self {
        match proc {
            Some(p) => {
                let sys: System = SystemExt::new();
                Self {
                    device_id: device, 
                    pid: p.pid,
                    name: match sys.get_process(p.pid as Pid) {
                        Some(n) => {
                            let mut cmd = n.name().clone().split_whitespace();
                            match cmd.next() {
                                Some(name) => name.to_string(),
                                None => "UNKNOWN".to_string()
                            }
                        },
                        None => "unknown".to_string()
                    },
                    mem: match p.used_gpu_memory {
                        UsedGpuMemory::Used(u) => Some(u),
                        UsedGpuMemory::Unavailable => None
                    },
                    proc_type: proc_type, 
                }
            }, 
            None => {
                Self {
                    device_id: 0,
                    pid: 0,
                    name: "UNKNOWN".to_string(), 
                    mem: None, 
                    proc_type: GPUProcessType::Unknown
                }
            }
        }
    }

    fn proc_list(procs: &Vec<ProcessInfo>, device: u32, proc_type: GPUProcessType) -> Vec<GPUProcess> {
        procs.iter().map(|p| GPUProcess::new(Some(&p.clone()), device, proc_type.clone())).collect()
    }
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
    pub power_usage: HashMap<u32, u32>,
    pub power_usage_history: HashMap<u32, Vec<u32>>,
    pub processes: Vec<GPUProcess>, //Device ID, (PID, Mem usage) 
    max_history_len: usize,
}

impl GPUDataStream for GPUMonitor {
    fn new(max_hist_len: usize) -> Self {        
        Self {
            driver_version: "UNKNOWN".to_string(),
            total_memory: HashMap::new(),
            nvml_indices: HashMap::new(),
            names: HashMap::new(),
            temps: HashMap::new(),
            temp_history: HashMap::new(),
            memory_usage: HashMap::new(),
            memory_usage_history: HashMap::new(),
            clks: HashMap::new(),
            clk_history: HashMap::new(),  
            power_usage: HashMap::new(),
            power_usage_history: HashMap::new(),
            processes: vec![], 
            max_history_len: max_hist_len
        }
    }

    fn init(&mut self, nvml: &NVML) -> Result<(), Error> {
        self.driver_version = nvml.sys_driver_version()?;
        let num_gpus = match nvml.device_count() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let gpus: Vec<(u32, Device)> = (0..num_gpus).map(|id| (id, nvml.device_by_index(id).unwrap()))
                                                    .collect();

        for (id, gpu) in &gpus {
            //Static
            self.total_memory.insert(*id, gpu.memory_info()?.total);
            self.nvml_indices.insert(*id, gpu.index()?);
            self.names.insert(*id, gpu.name()?);
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

            let mem_usage = gpu.memory_info()?.used;
            self.memory_usage.insert(*id, mem_usage);
            let mem_usage_history = self.memory_usage_history.entry(*id).or_insert(vec![0; self.max_history_len]);
            while mem_usage_history.len() >= self.max_history_len {
                mem_usage_history.remove(0);
            }
            mem_usage_history.push(mem_usage);
 
            let clk = GPUClks::new(Some(gpu))?; 
            self.clks.insert(*id, clk.clone());
            let clks_history = self.clk_history.entry(*id).or_insert(vec![GPUClks::new(None)?; self.max_history_len]);
            while clks_history.len() >= self.max_history_len {
                clks_history.remove(0);
            }
            clks_history.push(clk.clone());
                
            //let pow = gpu.power_usage()?; 
            //self.power_usage.insert(*id, pow);
            //let power_history = self.power_usage_history.entry(*id).or_insert(vec![0; self.max_history_len]);
            //while power_history.len() >= self.max_history_len {
            //    power_history.remove(0);
            //}
            //power_history.push(pow);
            let cl = gpu.running_compute_processes()?;
            let cl_processes = GPUProcess::proc_list(&cl, *id, GPUProcessType::Compute);
            let gl = gpu.running_graphics_processes()?;
            let gl_processes = GPUProcess::proc_list(&gl, *id, GPUProcessType::Graphics);
            let processes: Vec<GPUProcess> = vec![cl_processes, gl_processes].iter()
                                                                             .flat_map(|p| (*p).clone())
                                                                             .collect();
            self.processes.clear();
            self.processes = processes;
            //println!("{:?}",  processes);
        }
        Ok(())      
    }
}