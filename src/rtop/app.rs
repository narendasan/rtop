use std::collections::HashMap;

use tui::style::Color;
use termion::event::Key;

use rtop::cmd::Cmd;
use rtop::ui::tabs::Tabs;
use rtop::datastreams::servers::Servers;
use rtop::datastreams::systemmonitor::SystemMonitor;
use rtop::datastreams::randomsignal::RandomSignal;

pub struct App<'a> {
    pub items: Vec<&'a str>,
    pub events: Vec<(&'a str, &'a str)>,
    pub selected: usize,
    pub tabs: Tabs<'a>,
    pub show_chart: bool,
    pub progress: u16,
    pub data: Vec<u64>,
    pub data4: Vec<(&'a str, u64)>,
    pub window: [f64; 2],
    pub colors: [Color; 2],
    pub color_index: usize,
    pub servers: Servers<'a>,
    pub cpu_panel_memory: HashMap<String, Vec<(f64, f64)>>,
    pub sys_info: SystemMonitor,
}

impl <'a> App<'a> {
    pub fn input_handler(&mut self, input: Key) -> Option<Cmd>{
        match input {
            Key::Char('q') => {
                return Some(Cmd::Quit);
            }
            Key::Up => {
                if self.selected > 0 {
                    self.selected -= 1
                };
            }
            Key::Down => if self.selected < self.items.len() - 1 {
                self.selected += 1;
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
        self.window[0] += 1.0;
        self.window[1] += 1.0;
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

                self.cpu_panel_memory.insert(name.clone(), pairwise_data);
            }
        }
        self.sys_info.poll();
    }
}