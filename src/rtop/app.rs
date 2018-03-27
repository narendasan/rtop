use std::collections::HashMap;

use tui::style::Color;
use termion::event::Key;

use rtop::cmd::Cmd;
use rtop::ui::tabs::Tabs;
use rtop::datastreams::datastream::DataStream;
use rtop::datastreams::servers::Servers;
use rtop::datastreams::systemmonitor::SystemMonitor;
use rtop::datastreams::randomsignal::RandomSignal;

pub struct App<'a> {
    pub items: Vec<&'a str>,
    pub events: Vec<(&'a str, &'a str)>,
    pub selected_proc: usize,
    pub tabs: Tabs<'a>,
    pub show_chart: bool,
    pub progress: u16,
    pub data: Vec<u64>,
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
    pub sys_info: SystemMonitor,
}

impl <'a> App<'a> {
    pub fn new(history_len: usize, rand_signal: &RandomSignal) -> Self {
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
            data: rand_signal.clone().take(history_len).collect(),
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
            sys_info: DataStream::new(history_len),
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
            Key::Down => if self.selected_proc < self.sys_info.process_info.len() - 1 {
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

    pub fn update(&mut self, rand_signal:  &mut RandomSignal) {
        self.progress += 5;
        if self.progress > 100 {
            self.progress = 0;
        }
        self.data.insert(0, rand_signal.next().unwrap());
        self.data.pop();
        let i = self.data4.pop().unwrap();
        self.data4.insert(0, i);
        //self.window[0] += 1.0;
        //self.window[1] += 1.0;
        let i = self.events.pop().unwrap();
        self.events.insert(0, i);
        self.color_index += 1;
        if self.color_index >= self.colors.len() {
            self.color_index = 0;
        }
        //CPU History Parsing
        {
            for (name, usage) in &self.sys_info.cpu_usage_history {
                let pairwise_data = usage.iter()
                                        .enumerate()
                                        .map(|x| (x.0 as f64, x.1.clone() as f64))
                                        .collect::<Vec<(f64, f64)>>();
                let mut core_name = name.clone();
                let core_num = core_name.parse::<u32>().unwrap();
                core_name = format!("Core: {} ({:.2}%)", core_name, 
                                                      (self.sys_info.cpu_core_info[(core_num - 1) as usize].1 * 100.0).to_string());
                self.cpu_panel_memory.insert(core_num, (core_name, pairwise_data));
            }
        }
        //Memory History Parsing
        {
            for (i, usage) in self.sys_info.memory_usage_history.iter().enumerate() {                            
                self.mem_panel_memory.push((i as f64, usage.clone()));
            }
            self.mem_usage_str = format!("Memory ({:.2}%)", 100.0 * self.sys_info.memory_usage as f64 / self.sys_info.total_memory as f64);
        }
        //Swap History Parsing
        {
            for (i, usage) in self.sys_info.swap_usage_history.iter().enumerate() {                            
                self.swap_panel_memory.push((i as f64, usage.clone()));
            }
            self.swap_usage_str = format!("Swap ({:.2}%)", 100.0 * self.sys_info.swap_usage as f64 / self.sys_info.total_swap as f64);
        }
        self.sys_info.poll();
    }
}