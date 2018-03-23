use std::collections::HashMap;

use termion::event::Key;

use rtop::datastreams::systemmonitor::SystemMonitor;
use rtop::datastreams::randomsignal::RandomSignal;
use rtop::datastreams::sinsignal::SinSignal;

use tui::style::Color;

pub mod datastreams;
pub mod ui;

use self::ui::tabs::Tabs;
use self::datastreams::servers::Servers;

pub enum Cmd {
    Quit,
    Bell,
}

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
    pub cpu_panel_memory: HashMap<&'a str, Vec<(f64, f64)>>,
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

    pub fn update<'b:'a>(&mut self, rand_signal:  &mut RandomSignal, sys_info: &'b SystemMonitor) {
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
            let history = &sys_info.cpu_usage_history;
            for name in history.keys() {
                let data = history.get(name);
                match data {
                    Some(usage) => {
                        let pairwise_data = usage.iter()
                                                .enumerate()
                                                .map(|x| (x.0 as f64, x.1.clone() as f64))
                                                .collect::<Vec<(f64, f64)>>();

                        self.cpu_panel_memory.insert(name, pairwise_data);
                    },
                    None => (panic!("called `Option::unwrap()` on a `None` value")),
                }
                
            }
        }
    }
}