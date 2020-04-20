use std::collections::HashMap;

use sysinfo::{System, SystemExt, Pid, ProcessExt};

//use ::phf::phf_map;

extern crate nvml_wrapper as nvml;
//use nvml_wrapper as nvml;
use self::nvml::{NVML, Device};
use self::nvml::enum_wrappers::device::{TemperatureSensor, Clock};
use self::nvml::enums::device::UsedGpuMemory;
use self::nvml::struct_wrappers::device::ProcessInfo;

use crate::rtop::error::Error;
use crate::rtop::datastreams::{datastream::GPUDataStream, utils};


//#[derive(Debug, Clone)]
// pub struct GPUClks {
//     pub graphics: u32,
//     pub sm: u32, 
//     pub mem: u32, 
//     pub video_enc_dec: u32, 
// }

// impl GPUClks {
//     fn new(gpu: Option<&Device>) -> Result<Self, Error> {
//         match gpu {
//             Some(gpu) => Ok(Self {
//                 graphics: gpu.clock(Clock::Graphics, ClockId::Current)?,
//                 sm: gpu.clock(Clock::SM, ClockId::Current)?,
//                 mem: gpu.clock(Clock::Memory, ClockId::Current)?,
//                 video_enc_dec: gpu.clock(Clock::Video, ClockId::Current)?,
//             }),
//             None => Ok(Self {
//                 graphics: 0, 
//                 sm: 0,
//                 mem: 0, 
//                 video_enc_dec: 0,
//             })      
//         }
//     }
// }

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

#[derive(Debug, Clone)]
pub struct GPUDeviceInfo {
    pub id: u32,
    pub name: String,
    pub bus_id: String,
    pub max_sm_clock: u32,
    pub max_mem_clock: u32,
    pub num_pcie_lanes: u32,
    pub max_memory: u64,
    pub power_limit: u32,
    pub vbios: String,
    //pub compute_capability: String,
    //pub arch: String,
}

impl GPUDeviceInfo {
    fn new(id: u32, gpu: &Device) -> Result<Self, Error> {
        let new = Self {
            id: id,
            name: gpu.name()?,
            bus_id: gpu.pci_info()?.bus_id,
            max_sm_clock: gpu.max_clock_info(Clock::SM)?,
            max_mem_clock: gpu.max_clock_info(Clock::Memory)?,
            num_pcie_lanes: gpu.max_pcie_link_width()?,
            max_memory: gpu.memory_info()?.total,
            power_limit: gpu.power_management_limit()? / 1000,
            vbios: gpu.vbios_version()?,
            //arch: gpu.board_part_number()?,
            //compute_capability: "UNKNOWN".to_string(),
        };
        //new.compute_capability = match ARCH_TO_CC.get(new.arch.as_str()).cloned() {
        //    Some(cc) => cc.to_string(),
        //    None => "UNKNOWN".to_string(),
        //};
        //println!("{}", new.arch);
        Ok(new)
    }
}
    

pub struct GPUMonitor {
    pub names: HashMap<u32, String>, //Device ID, Name
    pub device_info: HashMap<u32, GPUDeviceInfo>,
    pub temps: HashMap<u32, u32>, //Device ID, Temps
    pub temp_history: HashMap<u32, Vec<f64>>, //Device ID, Temps
    pub memory_usage: HashMap<u32, u64>,
    pub memory_usage_history: HashMap<u32, Vec<f64>>, //Device ID, mem (GB) 
    pub total_memory: HashMap<u32, u64>,
    pub nvml_indices: HashMap<u32, u32>,
    //pub clks: HashMap<u32, GPUClks>,
    //pub clk_history: HashMap<u32, Vec<GPUClks>>, 
    pub driver_version: String,
    pub cuda_version: String,
    pub power_usage: HashMap<u32, u32>,
    pub power_usage_history: HashMap<u32, Vec<f64>>,
    pub gpu_util: HashMap<u32, f64>,
    pub gpu_util_history: HashMap<u32, Vec<f64>>, 
    pub processes: Vec<GPUProcess>, //Device ID, (PID, Mem usage) 
    interpolation_len: u16,
    max_history_len: usize,
}

impl GPUDataStream for GPUMonitor {
    fn new(max_hist_len: usize, inter_len: u16) -> Self {        
        Self {
            driver_version: "UNKNOWN".to_string(),
            cuda_version: "UNKNOWN".to_string(),
            device_info: HashMap::new(),
            total_memory: HashMap::new(),
            nvml_indices: HashMap::new(),
            names: HashMap::new(),
            temps: HashMap::new(),
            temp_history: HashMap::new(),
            memory_usage: HashMap::new(),
            memory_usage_history: HashMap::new(),
            //clks: HashMap::new(),
            //clk_history: HashMap::new(),  
            power_usage: HashMap::new(),
            power_usage_history: HashMap::new(),
            gpu_util: HashMap::new(),
            gpu_util_history: HashMap::new(),
            processes: vec![], 
            interpolation_len: inter_len, 
            max_history_len: max_hist_len
        }
    }

    fn init(&mut self, nvml: &NVML) -> Result<(), Error> {
        self.driver_version = nvml.sys_driver_version()?;
        self.cuda_version = GPUMonitor::cuda_version(nvml.sys_cuda_driver_version()?);
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
            self.device_info.insert(*id, GPUDeviceInfo::new(*id, &gpu)?);
        } 
        
        Ok(())
    }

    fn poll(&mut self, nvml: &NVML) -> Result<(), Error> {
        let num_gpus = match nvml.device_count() {
            Ok(n) => n,
            Err(_e) => 0,
        };
        let gpus: Vec<(u32, Device)> = (0..num_gpus).map(|id| (id, nvml.device_by_index(id).unwrap()))
                                                    .collect(); 
        for (id, gpu) in &gpus { 
            let gpu_temp = gpu.temperature(TemperatureSensor::Gpu)?;
            self.temps.insert(*id, gpu_temp);
            let temp_history = self.temp_history.entry(*id).or_insert(vec![0.0; self.max_history_len]);
            while temp_history.len() >= self.max_history_len {
                temp_history.remove(0);
            }
            let last_temp = match temp_history.last() {
                Some(l) => l.clone(),
                None => 0.0,
            };
            temp_history.extend_from_slice(utils::interpolate(last_temp, gpu_temp as f64, self.interpolation_len).as_slice());

            let mem_usage = gpu.memory_info()?.used;
            self.memory_usage.insert(*id, mem_usage);
            let mem_usage_history = self.memory_usage_history.entry(*id).or_insert(vec![0.0; self.max_history_len]);
            while mem_usage_history.len() >= self.max_history_len {
                mem_usage_history.remove(0);
            }
            let last_mem = match mem_usage_history.last() {
                Some(l) => l.clone(),
                None => 0.0,
            };
            mem_usage_history.extend_from_slice(utils::interpolate(last_mem, mem_usage as f64 / *self.total_memory.get(&*id).unwrap() as f64, self.interpolation_len).as_slice());
 
            // let clk = GPUClks::new(Some(gpu))?; 
            // self.clks.insert(*id, clk.clone());
            // let clks_history = self.clk_history.entry(*id).or_insert(vec![GPUClks::new(None)?; self.max_history_len]);
            // while clks_history.len() >= self.max_history_len {
            //     clks_history.remove(0);
            // }
            // clks_history.push(clk.clone());
                
            let pow = gpu.power_usage()? / 1000; 
            self.power_usage.insert(*id, pow);
            let power_history = self.power_usage_history.entry(*id).or_insert(vec![0.0; self.max_history_len]);
            while power_history.len() >= self.max_history_len {
                power_history.remove(0);
            }

            let last_power = match power_history.last() {
                Some(l) => l.clone(),
                None => 0.0,
            };
            power_history.extend_from_slice(utils::interpolate(last_power, pow as f64, self.interpolation_len).as_slice());

            let util = gpu.utilization_rates()?.gpu as f64; 
            self.gpu_util.insert(*id, util);
            let gpu_util_history = self.gpu_util_history.entry(*id).or_insert(vec![0.0; self.max_history_len]);
            while gpu_util_history.len() >= self.max_history_len {
                gpu_util_history.remove(0);
            }

            let last_util = match gpu_util_history.last() {
                Some(l) => l.clone(),
                None => 0.0,
            };
            gpu_util_history.extend_from_slice(utils::interpolate(last_util, util, self.interpolation_len).as_slice());

            let cl = gpu.running_compute_processes()?;
            let cl_processes = GPUProcess::proc_list(&cl, *id, GPUProcessType::Compute);
            let gl = gpu.running_graphics_processes()?;
            let gl_processes = GPUProcess::proc_list(&gl, *id, GPUProcessType::Graphics);
            let processes: Vec<GPUProcess> = vec![cl_processes, gl_processes].iter()
                                                                             .flat_map(|p| (*p).clone())
                                                                             .collect();
            self.processes.clear();
            self.processes = processes;
        }
        Ok(())      
    }
}

impl GPUMonitor {
    fn cuda_version(version: i32) -> String {
        format!("{}.{}", nvml::cuda_driver_version_major(version), nvml::cuda_driver_version_minor(version))
    }
}


// static ARCH_TO_CC: phf::Map<&'static str, &'static str> = phf_map! {
//     "G80" => "1.0",
//     "G92" => "1.1",
//     "G94" => "1.1",
//     "G96" => "1.1",
//     "G98" => "1.1",
//     "G84" => "1.1",
//     "G86" => "1.1",
//     "GT218" => "1.2",
//     "GT216" => "1.2",
//     "GT215" => "1.2",
//     "GT200," => "1.3", 
//     "GT200b" => "1.3", 
//     "GF100" => "2.0",
//     "GF110" => "2.0",
//     "GF104" => "2.1",
//     "GF106" => "2.1",
//     "GF108" => "2.1",
//     "GF114" => "2.1",
//     "GF116" => "2.1",
//     "GF117" => "2.1",
//     "GF119" => "2.1",
//     "GK104" => "3.0",
//     "GK106" => "3.0",
//     "GK107" => "3.0",
//     "GK20A" => "3.2",
//     "GK110" => "3.5",
//     "GK208" => "3.5",
//     "GK210" => "3.7",
//     "GM107" => "5.0",
//     "GM108" => "5.0",
//     "GM200" => "5.2",
//     "GM204" => "5.2",
//     "GM206" => "5.2",
//     "GM20B" => "5.3",
//     "GP100" => "6.0",
//     "GP102" => "6.1",
//     "GP104" => "6.1",
//     "GP106" => "6.1",
//     "GP107" => "6.1",
//     "GP108" => "6.1",
//     "GP10B" => "6.2",
//     "GV100" => "7.0",
//     "GV10B" => "7.2",
//     "TU102" => "7.5",
//     "TU104" => "7.5",
//     "TU106" => "7.5",
//     "TU116" => "7.5",
//     "TU117" => "7.5", 
// };
