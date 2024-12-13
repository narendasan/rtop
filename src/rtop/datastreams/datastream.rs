#[allow(unused_imports)]
use crate::rtop::error::Error;
#[cfg(feature = "battery-monitor")]
use battery::Manager;
#[cfg(feature = "gpu-monitor")]
use nvml_wrapper::Nvml;
use sysinfo::System as SysInfoSystem;

pub trait SysDataStream {
    fn new(max_hist_len: usize, interpolation_len: u16) -> Self;
    fn poll(&mut self, system_info: &SysInfoSystem);
}

#[cfg(feature = "battery-monitor")]
pub trait BatteryDataStream {
    fn new(max_hist_len: usize, interpolation_len: u16) -> Self;
    fn poll(&mut self, battery_manager: &Manager) -> Result<(), Error>;
}

#[cfg(feature = "gpu-monitor")]
pub trait GPUDataStream {
    fn new(max_hist_len: usize, interpolation_len: u16) -> Self;
    fn init(&mut self, nvml: &Nvml) -> Result<(), Error>;
    fn poll(&mut self, system_info: &SysInfoSystem, nvml: &Nvml) -> Result<(), Error>;
}
