mod deviceinfo;
mod driverinfo;
mod memoryusage;
mod powerusage;
mod processes;
mod temps;
mod utilization;
mod utils;

pub use self::deviceinfo::device_panel;
pub use self::driverinfo::driver_panel;
pub use self::memoryusage::mem_history_panel;
pub use self::powerusage::power_history_panel;
pub use self::processes::processes_panel;
pub use self::temps::temp_history_panel;
pub use self::utilization::utilization_history_panel;
