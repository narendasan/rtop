use crate::rtop::datastreams::{
    CPUMonitor, DiskMonitor, MemoryMonitor, NetworkMonitor, ProcessMonitor, SysDataStream,
};
use crate::rtop::error::Error;
#[cfg(feature = "battery-monitor")]
use battery::Manager;
#[cfg(feature = "gpu-monitor")]
use nvml_wrapper::Nvml;
#[cfg(feature = "gpu-monitor")]
use std::ffi::OsStr;
#[cfg(feature = "gpu-monitor")]
use std::fs::exists;
use sysinfo::System as SysInfoSystem;

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
    pub gpu_info_src: Nvml,
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
            sys_info_src: SysInfoSystem::new_all(),
            #[cfg(feature = "battery-monitor")]
            battery_info_src: Manager::new()?,
            #[cfg(feature = "gpu-monitor")]
            gpu_info_src: Nvml::builder()
                .lib_path(OsStr::new(&get_nvml_install_path()?))
                .init()?,
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
        self.gpu_info.poll(&self.sys_info_src, &self.gpu_info_src)?;
        Ok(())
    }
}

#[cfg(feature = "gpu-monitor")]
fn get_nvml_install_path() -> Result<String, Error> {
    let potential_install_locations = vec![
        "/usr/lib/x86_64-linux-gnu".to_string(),
        "/usr/lib/aarch64-linux-gnu".to_string(),
    ];

    let mut install_location: Option<String> = None;
    for loc in potential_install_locations {
        match exists(format!("{}/libnvidia-ml.so.1", loc)) {
            Ok(e) => {
                if e {
                    install_location = Some(loc.to_string());
                    break;
                }
            }
            Err(_) => {
                return Err(Error::from(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Could not determine if libnvidia-ml.so.1 exists at {}", loc),
                )))
            }
        }
    }

    match install_location {
        Some(loc) => {
            return Ok(format!("{}/libnvidia-ml.so.1", loc));
        }
        None => {
            return Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unable to find libnvidia-ml.so.1",
            )))
        }
    }
}
