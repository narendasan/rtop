extern crate sysinfo;
#[cfg(feature = "gpu-monitor")]
extern crate nvml_wrapper as nvml;

use std::collections::HashMap;

use tui::style::Color;
use termion::event::Key;
use self::sysinfo::{System, SystemExt};
#[cfg(feature = "gpu-monitor")]
use self::nvml::{NVML};

use rtop::cmd::Cmd;
use rtop::ui::tabs::Tabs;
use rtop::datastreams::{SysDataStream, DiskMonitor, MemoryMonitor, 
                        CPUMonitor, NetworkMonitor, ProcessMonitor};
#[cfg(feature = "gpu-monitor")]
use rtop::datastreams::{GPUDataStream, GPUMonitor};

pub struct App<'a> {
    pub selected_proc: usize,
    pub tabs: Tabs<'a>,
    pub show_chart: bool,
    pub window: [f64; 2],
    pub cpu_panel_memory: HashMap<u32, (String, Vec<(f64, f64)>)>,
    pub mem_panel_memory: Vec<(f64, f64)>,
    pub mem_usage_str: String,
    pub swap_panel_memory: Vec<(f64, f64)>,
    pub swap_usage_str: String,
    pub net_in_str: String,
    pub net_out_str: String,
    pub disk_info: DiskMonitor,
    pub cpu_info: CPUMonitor,
    #[cfg(feature = "gpu-monitor")]
    pub gpu_info: GPUMonitor,
    pub net_info: NetworkMonitor,
    pub mem_info: MemoryMonitor,
    pub process_info: ProcessMonitor,
    pub sys_info_src: System,
    #[cfg(feature = "gpu-monitor")]
    pub gpu_info_src: NVML,
}

impl <'a> App<'a> {
    pub fn new(history_len: usize) -> Result<Self, String> {
        Ok(Self {
            selected_proc: 0,
            tabs: Tabs {
                titles: { 
                    if cfg!(feature = "gpu-monitor") { 
                        vec!["System/OS/CPU", "GPU"]
                    } else {
                        vec!["System/OS/CPU"]
                    }
                },
                selection: 0,
            },
            show_chart: true,
            window: [0.0, history_len as f64],
            cpu_panel_memory: HashMap::new(),
            mem_panel_memory: Vec::new(),
            mem_usage_str: String::new(),
            swap_panel_memory: Vec::new(),
            swap_usage_str: String::new(),
            net_in_str: String::new(),
            net_out_str: String::new(),
            disk_info: SysDataStream::new(history_len),
            cpu_info: SysDataStream::new(history_len),
            #[cfg(feature = "gpu-monitor")]
            gpu_info: GPUDataStream::new(history_len),
            net_info: SysDataStream::new(history_len),
            mem_info: SysDataStream::new(history_len),
            process_info: SysDataStream::new(history_len),
            sys_info_src: System::new(),
            #[cfg(feature = "gpu-monitor")]
            gpu_info_src: (NVML::init().or_else(|err| Err(String::from(err.description()))))?
        })
    }

    pub fn input_handler(&mut self, input: Key) -> Option<Cmd>{
        match input {
            Key::Char('q') => {
                return Some(Cmd::Quit);
            }
            Key::Up => {
                if self.selected_proc > 0 {
                    self.selected_proc -= 1
                };
            }
            Key::Down => if self.selected_proc < self.process_info.process_info.len() - 1 {
                self.selected_proc += 1;
            },
            Key::Left => {
                self.tabs.previous();
            }
            Key::Right => {
                self.tabs.next();
            }
            Key::Char('t') => {
                self.show_chart = !self.show_chart;
            }
            _ => {}
        }
        return None;
    }

    pub fn update(&mut self) {
        self.sys_info_src.refresh_all();
        self.disk_info.poll(&self.sys_info_src);
        self.cpu_info.poll(&self.sys_info_src);
        self.net_info.poll(&self.sys_info_src);
        self.mem_info.poll(&self.sys_info_src);
        self.process_info.poll(&self.sys_info_src);
        #[cfg(feature = "gpu-monitor")]
        self.gpu_info.poll(&self.gpu_info_src);
        //CPU History Parsing
        {
            for (name, usage) in &self.cpu_info.cpu_usage_history {
                let pairwise_data = usage.iter()
                                        .enumerate()
                                        .map(|x| (x.0 as f64, x.1.clone() as f64))
                                        .collect::<Vec<(f64, f64)>>();
                let mut core_name = name.clone();
                let mut core_num = 0;
                if cfg!(target_os = "macos") {
                    match core_name.parse::<u32>() {
                        Ok(num) => {core_num = num - 1}, //MacOS 
                        Err(_) => {panic!("Unable to parse CPU ID")}
                    }
                } else {
                    if core_name.contains("cpu") {
                        let (_,s) = core_name.split_at_mut(3);
                        match s.parse::<u32>() {
                            Ok(num) => {core_num = num},
                            Err(_) => {panic!("Unable to parse CPU ID")}, 
                        }
                    } else {
                        panic!("Cannot get CPU ID");
                    }
                }
                let core_label = core_num.to_string();
                core_name = format!("Core: {} ({:.2}%)", core_label, 
                                                      (self.cpu_info.cpu_core_info[(core_num) as usize].1 * 100.0).to_string());
                self.cpu_panel_memory.insert(core_num, (core_name, pairwise_data));
            }
        }
        //Memory History Parsing 
        {
            self.mem_panel_memory =  self.mem_info.memory_usage_history.iter()
                                                                        .enumerate()
                                                                        .map(|(i, u)| (i as f64, u.clone()))
                                                                        .collect::<Vec<(f64, f64)>>();                         
            self.mem_usage_str = format!("Memory ({:.2}%)", 100.0 * self.mem_info.memory_usage as f64 / self.mem_info.total_memory as f64);
        }
        //Swap History Parsing
        {
            self.swap_panel_memory =  self.mem_info.swap_usage_history.iter()
                                                                      .enumerate()
                                                                      .map(|(i, u)| (i as f64, u.clone()))
                                                                      .collect::<Vec<(f64, f64)>>(); 
            self.swap_usage_str = format!("Swap ({:.2}%)", 100.0 * self.mem_info.swap_usage as f64 / self.mem_info.total_swap as f64);
        }
        //Network Parsing
        {
            
            let (scalar, unit) = App::si_prefix(self.net_info.net_in); 
            self.net_in_str = format!("Current Incoming Traffic: {} {}/s", (self.net_info.net_in) / scalar, unit);


            let (scalar, unit) = App::si_prefix(self.net_info.net_out); 
            self.net_out_str = format!("Current Outgoing Network Traffic: {} {}/s", (self.net_info.net_out) / scalar, unit);    
        }
        #[cfg(feature = "gpu-monitor")]
        //GPU Usage Parsing 
        {
            println!("What up");
            println!("Waz up");
        }
    }

    fn si_prefix(bits: u64) -> (u64, String) {
        let b = bits as f64;
        if b == 0.0 {
            return (1 as u64, String::from("B"));
        }
        match b.log(10.0) as u64 {
            0 | 1 | 2 => ((10 as u64).pow(0), String::from("b")),
            3 | 4 | 5 => ((10 as u64).pow(3), String::from("Kb")),
            6 | 7 | 8 => ((10 as u64).pow(6), String::from("Mb")),
            9 | 10 | 11 => ((10 as u64).pow(9), String::from("Gb")),
            12 | 13 | 14 => ((10 as u64).pow(12), String::from("Tb")),
            15 | 16 | 17 => ((10 as u64).pow(15), String::from("Pb")),
            _ => ((10 as u64).pow(18), String::from("Eb")),
        }
    }
}