use sysinfo::System;
#[cfg(feature = "gpu-monitor")] use nvml_wrapper::NVML;

use crate::rtop::error::Error;


pub trait SysDataStream {
    fn new(max_hist_len: usize, interpolation_len: u16) -> Self;
    fn poll(&mut self, system_info: &System);
}

#[cfg(feature = "gpu-monitor")]
pub trait GPUDataStream {
    fn new(max_hist_len: usize, interpolation_len: u16) -> Self;
    fn init(&mut self, nvml: &NVML) -> Result<(), Error>;
    fn poll(&mut self, nvml: &NVML) -> Result<(), Error>;
}

