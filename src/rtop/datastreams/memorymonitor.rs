use sysinfo::{System, SystemExt};

use crate::rtop::datastreams::{datastream::SysDataStream, utils};

pub struct MemoryMonitor {
    pub memory_usage: u64,
    pub memory_usage_history: Vec<f64>, //Name, Usage
    pub total_memory: u64,
    pub swap_usage: u64,
    pub swap_usage_history: Vec<f64>,
    pub total_swap: u64,
    max_history_len: usize,
    interpolation_len: u16
}

impl SysDataStream for MemoryMonitor {
    fn new(max_hist_len: usize, inter_len: u16) -> Self {
        Self {
            memory_usage: 0,
            total_memory: 10,
            memory_usage_history: vec![0.0; max_hist_len],
            swap_usage: 0,
            total_swap: 10,
            swap_usage_history: vec![0.0; max_hist_len],
            max_history_len: max_hist_len,
            interpolation_len: inter_len
        }
    }

    fn poll(&mut self, system_info: &System) {
        self.memory_usage = system_info.used_memory();
        self.total_memory = system_info.total_memory();
        self.swap_usage = system_info.used_swap();
        self.total_swap = system_info.total_swap();
        if self.total_swap == 0 {
            self.total_swap = 10;
        }

        while self.memory_usage_history.len() >= self.max_history_len {
            self.memory_usage_history.remove(0);
        }
        let last_mem = match self.memory_usage_history.last() {
            Some(l) => *l,
            None => 0.0,
        };
        self.memory_usage_history.extend_from_slice(utils::interpolate(last_mem, self.memory_usage as f64 / self.total_memory as f64, self.interpolation_len).as_slice());

        while self.swap_usage_history.len() >= self.max_history_len {
            self.swap_usage_history.remove(0);
        }
        let last_swap = match self.swap_usage_history.last() {
            Some(l) => *l,
            None => 0.0,
        };
        self.swap_usage_history.push(self.swap_usage as f64 / self.total_swap as f64);
        self.swap_usage_history.extend_from_slice(utils::interpolate(last_swap, self.swap_usage as f64 / self.total_swap as f64, self.interpolation_len).as_slice());
    }
}