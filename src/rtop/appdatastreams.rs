#[cfg(feature = "battery-monitor")]
use battery::Manager;
#[cfg(feature = "gpu-monitor")]
use nvml_wrapper::NVML;
use sysinfo::System as SysInfoSystem;

use crate::rtop::datastreams::{
    CPUMonitor, DiskMonitor, MemoryMonitor, NetworkMonitor, ProcessMonitor, SysDataStream,
};
use crate::rtop::error::Error;

#[cfg(feature = "battery-monitor")]
use crate::rtop::datastreams::{BatteryDataStream, BatteryMonitor};
#[cfg(feature = "gpu-monitor")]
use crate::rtop::datastreams::{GPUDataStream, GPUMonitor};

pub struct AppDataStreams {
    pub disk_info: DiskMonitor,
    pub cpu_info: CPUMonitor,
    #[cfg(feature = "gpu-monitor")]
    pub gpu_info: GPUMonitor,
    pub net_info: NetworkMonitor,
    pub mem_info: MemoryMonitor,
    pub process_info: ProcessMonitor,
    #[cfg(feature = "battery-monitor")]
    pub battery_info: BatteryMonitor,
    pub sys_info_src: SysInfoSystem,
    #[cfg(feature = "gpu-monitor")]
    pub gpu_info_src: NVML,
    #[cfg(feature = "battery-monitor")]
    pub battery_info_src: Manager,
}

impl<'a> AppDataStreams {
    pub fn new(history_len: usize, interpolation_len: u16) -> Result<Self, Error> {
        Ok(Self {
            disk_info: SysDataStream::new(history_len, interpolation_len),
            cpu_info: SysDataStream::new(history_len, interpolation_len),
            #[cfg(feature = "gpu-monitor")]
            gpu_info: GPUDataStream::new(history_len, interpolation_len),
            net_info: SysDataStream::new(history_len, interpolation_len),
            mem_info: SysDataStream::new(history_len, interpolation_len),
            process_info: SysDataStream::new(history_len, interpolation_len),
            #[cfg(feature = "battery-monitor")]
            battery_info: BatteryDataStream::new(history_len, interpolation_len),
            sys_info_src: SysInfoSystem::new(),
            #[cfg(feature = "battery-monitor")]
            battery_info_src: Manager::new()?,
            #[cfg(feature = "gpu-monitor")]
            gpu_info_src: NVML::init()?,
        })
    }

    #[cfg(feature = "gpu-monitor")]
    pub fn init(&mut self) -> Result<(), Error> {
        self.gpu_info.init(&self.gpu_info_src)?;
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), Error> {
        self.sys_info_src.refresh_all();
        self.disk_info.poll(&self.sys_info_src);
        self.cpu_info.poll(&self.sys_info_src);
        self.net_info.poll(&self.sys_info_src);
        self.mem_info.poll(&self.sys_info_src);
        self.process_info.poll(&self.sys_info_src);
        #[cfg(feature = "battery-monitor")]
        self.battery_info.poll(&self.battery_info_src)?;
        #[cfg(feature = "gpu-monitor")]
        self.gpu_info.poll(&self.gpu_info_src)?;
        Ok(())
    }
}
