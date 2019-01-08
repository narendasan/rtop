mod processes;
mod memoryusage;
mod temps;

pub use self::processes::processes_panel as processes_panel;
pub use self::memoryusage::mem_history_panel as mem_history_panel;
pub use self::temps::temp_history_panel as temp_history_panel;