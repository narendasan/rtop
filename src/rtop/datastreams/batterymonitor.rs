use crate::rtop::datastreams::BatteryDataStream;
use crate::rtop::error::Error;
use battery::units::{
    electric_potential::volt, energy::watt_hour, power::watt, ratio::percent,
    thermodynamic_temperature::degree_celsius, time::second, Time,
};
use battery::Manager;
use battery::State as BatteryState;

#[derive(Debug)]
pub enum ChargingStatus {
    Discharging(u64),
    Charging(u64),
    Full,
    Empty,
    Unknown,
}

#[derive(Debug)]
pub struct BatteryMonitor {
    pub battery_lvl: f32,
    pub cycle_count: String,
    pub health: f32,
    pub temp: String,
    pub kind: String,
    pub vendor: String,
    pub serial: String,
    pub model: String,
    pub voltage: f32,
    pub power_draw: f32,
    pub energy: f32,
    pub energy_full: f32,
    pub designed_energy_full: f32,
    pub charging_status: ChargingStatus,
}

impl BatteryDataStream for BatteryMonitor {
    fn new(_max_hist_len: usize, _inter_len: u16) -> Self {
        Self {
            battery_lvl: 100.0,
            cycle_count: String::new(),
            health: 100.0,
            temp: String::new(),
            kind: String::new(),
            vendor: String::new(),
            serial: String::new(),
            model: String::new(),
            voltage: 0.0,
            power_draw: 0.0,
            energy: 0.0,
            energy_full: 0.0,
            designed_energy_full: 0.0,
            charging_status: ChargingStatus::Unknown,
        }
    }

    fn poll(&mut self, manager: &Manager) -> Result<(), Error> {
        let get_time = |t: Option<Time>| -> u64 {
            match t {
                Some(time) => time.get::<second>() as u64,
                None => 0,
            }
        };

        let to_string = |t: Option<&str>| -> String {
            match t {
                Some(v) => v.to_string(),
                None => "Unknown".to_string(),
            }
        };

        for bat in manager.batteries()? {
            let battery = bat?;
            self.charging_status = match battery.state() {
                BatteryState::Unknown => ChargingStatus::Unknown,
                BatteryState::Full => ChargingStatus::Full,
                BatteryState::Empty => ChargingStatus::Empty,
                BatteryState::Discharging => {
                    ChargingStatus::Discharging(get_time(battery.time_to_empty()))
                }
                BatteryState::Charging => {
                    ChargingStatus::Charging(get_time(battery.time_to_full()))
                }
                BatteryState::__Nonexhaustive => ChargingStatus::Unknown,
            };
            self.battery_lvl = battery.state_of_charge().get::<percent>() as f32;
            self.cycle_count = match battery.cycle_count() {
                Some(count) => count.to_string(),
                None => "Unknown".to_string(),
            };
            self.health = battery.state_of_health().get::<percent>() as f32;
            self.temp = match battery.temperature() {
                Some(temp) => format!("{:.2}°C", temp.get::<degree_celsius>() as f32),
                None => "Unknown".to_string(),
            };
            self.kind = battery.technology().to_string();
            self.vendor = to_string(battery.vendor());
            self.model = to_string(battery.model());
            self.serial = to_string(battery.serial_number());
            self.voltage = battery.voltage().get::<volt>() as f32;
            self.power_draw = battery.energy_rate().get::<watt>() as f32;
            self.energy = battery.energy().get::<watt_hour>() as f32;
            self.energy_full = battery.energy_full().get::<watt_hour>() as f32;
            self.designed_energy_full = battery.energy_full_design().get::<watt_hour>() as f32;
        }
        Ok(())
    }
}
