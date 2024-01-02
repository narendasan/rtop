#[cfg(feature = "battery-monitor")]
mod battery;
mod cpuusage;
mod disks;
mod memoryswapusage;
mod network;
mod processes;

#[cfg(feature = "battery-monitor")]
pub use self::battery::battery_panel;
pub use self::cpuusage::cpu_usage_history_panel;
pub use self::disks::disk_usage_panel;
pub use self::memoryswapusage::mem_and_swap_history_panel;
pub use self::network::network_info_panel;
pub use self::processes::processes_panel;
