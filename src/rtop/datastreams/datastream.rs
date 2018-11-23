extern crate sysinfo;
#[cfg(feature = "gpu-monitor")]
extern crate nvml_wrapper as nvml;

use self::sysinfo::System;
#[cfg(feature = "gpu-monitor")]
use self::nvml::NVML;


pub trait SysDataStream {
    fn new(max_hist_len: usize) -> Self;
    fn poll(&mut self, system_info: &System);
}

#[cfg(feature = "gpu-monitor")]
pub trait GPUDataStream {
    fn new(max_hist_len: usize) -> Self;
    fn poll(&mut self, nvml: &NVML);
}
