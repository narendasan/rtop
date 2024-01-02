use std::collections::HashMap;
use termion::event::Key;

use crate::rtop::appdatastreams::AppDataStreams;
use crate::rtop::cmd::Cmd;
#[cfg(feature = "battery-monitor")]
use crate::rtop::datastreams::ChargingStatus;
use crate::rtop::error::Error;
use crate::rtop::ui::tabs::Tabs;

pub struct App<'a> {
    pub selected_proc: usize,
    pub tabs: Tabs<'a>,
    pub window: [f64; 2],
    pub cpu_panel_memory: HashMap<u32, (String, Vec<(f64, f64)>)>,
    pub mem_panel_memory: Vec<(f64, f64)>,
    pub mem_usage_str: String,
    pub swap_panel_memory: Vec<(f64, f64)>,
    pub swap_usage_str: String,
    pub net_in_str: String,
    pub net_out_str: String,
    #[cfg(feature = "battery-monitor")]
    pub battery_level: f32,
    #[cfg(feature = "battery-monitor")]
    pub battery_status: String,
    #[cfg(feature = "gpu-monitor")]
    pub gpu_mem_panel_memory: HashMap<u32, (String, Vec<(f64, f64)>)>,
    #[cfg(feature = "gpu-monitor")]
    pub gpu_temp_panel_memory: HashMap<u32, (String, Vec<(f64, f64)>)>,
    #[cfg(feature = "gpu-monitor")]
    pub gpu_power_panel_memory: HashMap<u32, (String, Vec<(f64, f64)>)>,
    #[cfg(feature = "gpu-monitor")]
    pub gpu_util_panel_memory: HashMap<u32, (String, Vec<(f64, f64)>)>,
    #[cfg(feature = "gpu-monitor")]
    pub selected_gpu_proc: usize,
    pub datastreams: AppDataStreams,
}

impl<'a> App<'a> {
    pub fn new(history_len: usize, interpolation_len: u16) -> Result<Self, Error> {
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
            window: [0.0, history_len as f64],
            cpu_panel_memory: HashMap::new(),
            mem_panel_memory: Vec::new(),
            mem_usage_str: String::new(),
            swap_panel_memory: Vec::new(),
            swap_usage_str: String::new(),
            net_in_str: String::new(),
            net_out_str: String::new(),
            #[cfg(feature = "battery-monitor")]
            battery_level: 100.0,
            #[cfg(feature = "battery-monitor")]
            battery_status: String::new(),
            #[cfg(feature = "gpu-monitor")]
            gpu_mem_panel_memory: HashMap::new(),
            #[cfg(feature = "gpu-monitor")]
            gpu_temp_panel_memory: HashMap::new(),
            #[cfg(feature = "gpu-monitor")]
            gpu_power_panel_memory: HashMap::new(),
            #[cfg(feature = "gpu-monitor")]
            gpu_util_panel_memory: HashMap::new(),
            #[cfg(feature = "gpu-monitor")]
            selected_gpu_proc: 0,
            datastreams: AppDataStreams::new(history_len, interpolation_len)?,
        })
    }

    #[cfg(feature = "gpu-monitor")]
    pub fn init(&mut self) -> Result<(), Error> {
        self.datastreams.init()?;
        Ok(())
    }

    pub fn input_handler(&mut self, input: Key) -> Option<Cmd> {
        match input {
            Key::Char('q') => {
                return Some(Cmd::Quit);
            }
            Key::Up => {
                #[cfg(feature = "gpu-monitor")]
                {
                    if self.tabs.selection == 1 && self.selected_gpu_proc > 0 {
                        self.selected_gpu_proc -= 1
                    }
                }
                if self.tabs.selection == 0 && self.selected_proc > 0 {
                    self.selected_proc -= 1
                }
            }
            Key::Down => {
                #[cfg(feature = "gpu-monitor")]
                {
                    if self.tabs.selection == 1
                        && self.selected_gpu_proc < self.datastreams.gpu_info.processes.len() - 1
                    {
                        self.selected_gpu_proc += 1
                    }
                }
                if self.tabs.selection == 0
                    && self.selected_proc < self.datastreams.process_info.processes.len() - 1
                {
                    self.selected_proc += 1;
                }
            }
            Key::Left => {
                self.tabs.previous();
            }
            Key::Right => {
                self.tabs.next();
            }
            _ => {}
        }
        None
    }

    pub fn update(&mut self) -> Result<(), Error> {
        self.datastreams.update()?;
        //CPU History Parsing
        {
            for (name, usage) in &self.datastreams.cpu_info.cpu_usage_history {
                let pairwise_data = usage
                    .iter()
                    .enumerate()
                    .map(|x| (x.0 as f64, *x.1 as f64))
                    .collect::<Vec<(f64, f64)>>();
                let mut core_name = name.clone();
                let core_num;
                if cfg!(target_os = "macos") {
                    #[allow(clippy::match_wild_err_arm)]
                    match core_name.parse::<u32>() {
                        Ok(num) => core_num = num - 1, //MacOS
                        Err(_) => {
                            panic!("Unable to parse CPU ID")
                        }
                    }
                } else if core_name.contains("cpu") {
                    let (_, s) = core_name.split_at_mut(3);
                    #[allow(clippy::match_wild_err_arm)]
                    match s.parse::<u32>() {
                        Ok(num) => core_num = num,
                        Err(_) => {
                            panic!("Unable to parse CPU ID")
                        }
                    }
                } else {
                    panic!("Cannot get CPU ID");
                }

                let core_label = core_num.to_string();
                core_name = format!(
                    "Core: {} ({:.2}%)",
                    core_label,
                    (self.datastreams.cpu_info.cpu_core_info[(core_num) as usize].1 * 100.0)
                        .to_string()
                );
                self.cpu_panel_memory
                    .insert(core_num, (core_name, pairwise_data));
            }
        }
        //Memory History Parsing
        {
            self.mem_panel_memory = self
                .datastreams
                .mem_info
                .memory_usage_history
                .iter()
                .enumerate()
                .map(|(i, u)| (i as f64, *u))
                .collect::<Vec<(f64, f64)>>();
            self.mem_usage_str = format!(
                "Memory ({:.2}%)",
                100.0 * self.datastreams.mem_info.memory_usage as f64
                    / self.datastreams.mem_info.total_memory as f64
            );
        }
        //Swap History Parsing
        {
            self.swap_panel_memory = self
                .datastreams
                .mem_info
                .swap_usage_history
                .iter()
                .enumerate()
                .map(|(i, u)| (i as f64, *u))
                .collect::<Vec<(f64, f64)>>();
            self.swap_usage_str = format!(
                "Swap ({:.2}%)",
                100.0 * self.datastreams.mem_info.swap_usage as f64
                    / self.datastreams.mem_info.total_swap as f64
            );
        }
        //Network Parsing
        {
            let (scalar, unit) = App::si_prefix(self.datastreams.net_info.net_in);
            self.net_in_str = format!(
                "Current Incoming Traffic: {} {}b/s",
                (self.datastreams.net_info.net_in) / scalar,
                unit
            );

            let (scalar, unit) = App::si_prefix(self.datastreams.net_info.net_out);
            self.net_out_str = format!(
                "Current Outgoing Network Traffic: {} {}b/s",
                (self.datastreams.net_info.net_out) / scalar,
                unit
            );
        }
        #[cfg(feature = "battery-monitor")]
        {
            self.battery_level = self.datastreams.battery_info.battery_lvl;
            self.battery_status = match self.datastreams.battery_info.charging_status {
                ChargingStatus::Discharging(time) => {
                    let remaining_time = App::time_from_secs(time);
                    format!("🔋 On Battery (Time to empty: {})", remaining_time)
                }
                ChargingStatus::Charging(time) => {
                    let remaining_time = App::time_from_secs(time);
                    format!("⚡ Charging (Time to full: {})", remaining_time)
                }
                ChargingStatus::Full => "🔌  Connected to Power".to_string(),
                ChargingStatus::Empty => "😵 Empty Battery".to_string(),
                ChargingStatus::Unknown => "Unknown".to_string(),
            } + &format!(
                "\n⚕️ Battery Health: {:.2}% (Cycle count: {})",
                self.datastreams.battery_info.health, self.datastreams.battery_info.cycle_count
            ) + &format!(
                "\n〽️ Power Draw: {:.2}W ⚡ Voltage: {:.2}V 🌡  Temperature: {}",
                self.datastreams.battery_info.power_draw,
                self.datastreams.battery_info.voltage,
                self.datastreams.battery_info.temp
            ) + &format!(
                "\nBattery Energy: {:.2}/{:.2}Wh (Designed Capacity: {:.2}Wh)",
                self.datastreams.battery_info.energy,
                self.datastreams.battery_info.energy_full,
                self.datastreams.battery_info.designed_energy_full
            ) + &format!(
                "\nModel: {} Serial: {} Kind: {}",
                self.datastreams.battery_info.model,
                self.datastreams.battery_info.serial,
                self.datastreams.battery_info.kind
            );
        }
        #[cfg(feature = "gpu-monitor")]
        //GPU Usage Parsing
        {
            for (id, used_mem) in self.datastreams.gpu_info.memory_usage_history.iter() {
                let pairwise_data = used_mem
                    .iter()
                    .enumerate()
                    .map(|x| (x.0 as f64, x.1.clone() as f64))
                    .collect::<Vec<(f64, f64)>>();
                let (scalar, unit) =
                    App::si_prefix(*self.datastreams.gpu_info.memory_usage.get(id).unwrap() as u64);
                let device_name = format!(
                    "GPU {} ({:.2}{}B)",
                    id,
                    *self.datastreams.gpu_info.memory_usage.get(id).unwrap() as f64 / scalar as f64,
                    unit
                );
                self.gpu_mem_panel_memory
                    .insert(*id, (device_name, pairwise_data));
            }

            for (id, temps) in self.datastreams.gpu_info.temp_history.iter() {
                let pairwise_data = temps
                    .iter()
                    .enumerate()
                    .map(|x| (x.0 as f64, x.1.clone() as f64))
                    .collect::<Vec<(f64, f64)>>();
                let device_name = format!(
                    "GPU {} ({:.0}°C)",
                    id,
                    *self.datastreams.gpu_info.temps.get(id).unwrap() as f64
                );
                self.gpu_temp_panel_memory
                    .insert(*id, (device_name, pairwise_data));
            }

            for (id, power) in self.datastreams.gpu_info.power_usage_history.iter() {
                let pairwise_data = power
                    .iter()
                    .enumerate()
                    .map(|x| (x.0 as f64, x.1.clone() as f64))
                    .collect::<Vec<(f64, f64)>>();
                let device_name = format!(
                    "GPU {} ({:.0}W)",
                    id,
                    *self.datastreams.gpu_info.power_usage.get(id).unwrap() as f64
                );
                self.gpu_power_panel_memory
                    .insert(*id, (device_name, pairwise_data));
            }

            for (id, util) in self.datastreams.gpu_info.gpu_util_history.iter() {
                let pairwise_data = util
                    .iter()
                    .enumerate()
                    .map(|x| (x.0 as f64, x.1.clone() as f64))
                    .collect::<Vec<(f64, f64)>>();
                let device_name = format!(
                    "GPU {} ({:.0}%)",
                    id,
                    *self.datastreams.gpu_info.gpu_util.get(id).unwrap() as f64
                );
                self.gpu_util_panel_memory
                    .insert(*id, (device_name, pairwise_data));
            }
        }
        Ok(())
    }

    #[cfg(feature = "battery-monitor")]
    fn time_from_secs(secs: u64) -> String {
        let hrs = secs / 3600;
        let mut remainder = secs % 3600;
        let mins = remainder / 60;
        remainder = remainder % 60;
        format!("{} hr {} min {} sec", hrs, mins, remainder)
    }

    fn si_prefix(num: u64) -> (u64, String) {
        let n = num as f64;
        if n == 0.0 {
            return (1_u64, String::from(""));
        }
        match n.log(10.0) as u64 {
            0 | 1 | 2 => (10_u64.pow(0), String::from("")),
            3 | 4 | 5 => (10_u64.pow(3), String::from("K")),
            6 | 7 | 8 => (10_u64.pow(6), String::from("M")),
            9 | 10 | 11 => (10_u64.pow(9), String::from("G")),
            12 | 13 | 14 => (10_u64.pow(12), String::from("T")),
            15 | 16 | 17 => (10_u64.pow(15), String::from("P")),
            _ => (10_u64.pow(18), String::from("E")),
        }
    }
}
