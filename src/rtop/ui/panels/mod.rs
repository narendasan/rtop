mod text;
mod cpuusage;
mod processes;
mod memoryswapusage;
mod network;
mod disks;

pub use self::text::text_panel as text_panel;
pub use self::cpuusage::cpu_usage_history_panel as cpu_usage_history_panel;
pub use self::processes::processes_panel as processes_panel;
pub use self::memoryswapusage::mem_and_swap_history_panel as mem_and_swap_history_panel;
pub use self::network::network_info_panel as network_info_panel;
pub use self::disks::disk_usage_panel as disk_usage_panel;