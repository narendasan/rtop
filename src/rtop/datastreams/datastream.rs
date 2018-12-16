use sysinfo::System;
#[cfg(feature = "gpu-monitor")]
use nvml::NVML;

use crate::rtop::error::Error;


pub trait SysDataStream {
    fn new(max_hist_len: usize) -> Self;
    fn poll(&mut self, system_info: &System);
}

#[cfg(feature = "gpu-monitor")]
pub trait GPUDataStream {
    fn new(max_hist_len: usize) -> Self;
    fn init(&mut self, nvml: &NVML) -> Result<(), Error>;
    fn poll(&mut self, nvml: &NVML) -> Result<(), Error>;
}
