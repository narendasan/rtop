extern crate sysinfo;

use std::collections::HashMap;

use tui::style::Color;
use termion::event::Key;
use self::sysinfo::{System, SystemExt};

use rtop::cmd::Cmd;
use rtop::ui::tabs::Tabs;
use rtop::datastreams::{DataStream, DiskMonitor, MemoryMonitor, 
                        CPUMonitor, NetworkMonitor, ProcessMonitor};
use rtop::datastreams::servers::Servers;

pub struct App<'a> {
    pub items: Vec<&'a str>,
    pub events: Vec<(&'a str, &'a str)>,
    pub selected_proc: usize,
    pub tabs: Tabs<'a>,
    pub show_chart: bool,
    pub progress: u16,
    pub data4: Vec<(&'a str, u64)>,
    pub window: [f64; 2],
    pub colors: [Color; 2],
    pub color_index: usize,
    pub servers: Servers<'a>,
    pub cpu_panel_memory: HashMap<u32, (String, Vec<(f64, f64)>)>,
    pub mem_panel_memory: Vec<(f64, f64)>,
    pub mem_usage_str: String,
    pub swap_panel_memory: Vec<(f64, f64)>,
    pub swap_usage_str: String,
    pub net_in_str: String,
    pub net_out_str: String,
    pub disk_info: DiskMonitor,
    pub cpu_info: CPUMonitor,
    pub net_info: NetworkMonitor,
    pub mem_info: MemoryMonitor,
    pub process_info: ProcessMonitor,
    pub sys_monitor: System,
}

impl <'a> App<'a> {
    pub fn new(history_len: usize) -> Self {
        Self {
            items: vec![
            "Item1", "Item2", "Item3", "Item4", 
            ],
            events: vec![("Event1", "INFO"),],
            selected_proc: 0,
            tabs: Tabs {
                titles: vec!["Tab0", "Tab1"],
                selection: 0,
            },
            show_chart: true,
            progress: 0,
            data4: vec![
                ("B1", 9),
                ("B2", 12),
                ("B3", 5),
                ("B4", 8),
                ("B5", 2),
                ("B6", 4),
                ("B7", 5),
                ("B8", 9),
                ("B9", 14),
            ],
            window: [0.0, history_len as f64],
            colors: [Color::Magenta, Color::Red],
            color_index: 0,
            servers: Servers::new(),
            cpu_panel_memory: HashMap::new(),
            mem_panel_memory: Vec::new(),
            mem_usage_str: String::new(),
            swap_panel_memory: Vec::new(),
            swap_usage_str: String::new(),
            net_in_str: String::new(),
            net_out_str: String::new(),
            disk_info: DataStream::new(history_len),
            cpu_info: DataStream::new(history_len),
            net_info: DataStream::new(history_len),
            mem_info: DataStream::new(history_len),
            process_info: DataStream::new(history_len),
            sys_monitor: System::new(),
        }
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
        self.progress += 5;
        if self.progress > 100 {
            self.progress = 0;
        }
        let i = self.data4.pop().unwrap();
        self.data4.insert(0, i);
        let i = self.events.pop().unwrap();
        self.events.insert(0, i);
        self.color_index += 1;
        if self.color_index >= self.colors.len() {
            self.color_index = 0;
        }
        self.sys_monitor.refresh_all();
        self.disk_info.poll(&self.sys_monitor);
        self.cpu_info.poll(&self.sys_monitor);
        self.net_info.poll(&self.sys_monitor);
        self.mem_info.poll(&self.sys_monitor);
        self.process_info.poll(&self.sys_monitor);
        //CPU History Parsing
        {
            for (name, usage) in &self.cpu_info.cpu_usage_history {
                let pairwise_data = usage.iter()
                                        .enumerate()
                                        .map(|x| (x.0 as f64, x.1.clone() as f64))
                                        .collect::<Vec<(f64, f64)>>();
                let mut core_name = name.clone();
                let mut core_num = 0;
                match core_name.parse::<u32>() {
                    Ok(num) => {core_num = num - 1}, //MacOS 
                    Err(_) => {  //Linux 
                        if core_name.contains("cpu") {
                            let (_,s) = core_name.split_at_mut(3);
                            match s.parse::<u32>() {
                                Ok(num) => {core_num = num},
                                Err(_) => (), 
                            }
                        } else {
                            panic!("Cannot get CPU ID");
                        }
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