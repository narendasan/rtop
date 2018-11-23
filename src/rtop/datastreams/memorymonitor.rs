extern crate sysinfo;

use self::sysinfo::{System, SystemExt};

use rtop::datastreams::datastream::SysDataStream;

pub struct MemoryMonitor {
    pub memory_usage: u64,
    pub memory_usage_history: Vec<f64>, //Name, Usage
    pub total_memory: u64,
    pub swap_usage: u64,
    pub swap_usage_history: Vec<f64>,
    pub total_swap: u64,
    max_history_len: usize,
}

impl SysDataStream for MemoryMonitor {
    fn new(max_hist_len: usize) -> Self {        
        Self {
            memory_usage: 0,
            total_memory: 10,
            memory_usage_history: vec![0.0; max_hist_len],
            swap_usage: 0,
            total_swap: 10,
            swap_usage_history: vec![0.0; max_hist_len],
            max_history_len: max_hist_len,
        }
    }

    fn poll(&mut self, system_info: &System) {
        self.memory_usage = system_info.get_used_memory();
        self.total_memory = system_info.get_total_memory();
        self.swap_usage = system_info.get_used_swap();
        self.total_swap = system_info.get_total_swap();
        if self.total_swap == 0 {
            self.total_swap = 10;
        }

        while self.memory_usage_history.len() >= self.max_history_len {
            self.memory_usage_history.remove(0);
        }
        self.memory_usage_history.push(self.memory_usage as f64 / self.total_memory as f64);

        while self.swap_usage_history.len() >= self.max_history_len {
            self.swap_usage_history.remove(0);
        }
        self.swap_usage_history.push(self.swap_usage as f64 / self.total_swap as f64);
    }
}