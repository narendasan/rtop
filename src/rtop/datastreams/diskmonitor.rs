use std::str;
use sysinfo::{Disk, System, SystemExt, DiskExt};

use crate::rtop::datastreams::datastream::SysDataStream;

pub struct DiskMonitor {
    pub disk_usage: Vec<(String, String, u64, u64)>, //Mount, type, used, total 
}

impl SysDataStream for DiskMonitor {
    fn new(_max_hist_len: usize, _inter_len: u16) -> Self {        
        Self {
            disk_usage: Vec::new(),
        }
    }

    fn poll(&mut self, system_info: &System) {
        let disks = system_info.get_disks();
        
        self.disk_usage.clear();
        for disk in disks {
            self.disk_usage.push(DiskMonitor::parse_disk_info(disk));
        }
    }
}

impl DiskMonitor {
    fn parse_disk_info(disk: &Disk) -> (String, String, u64, u64) {
        let name = disk.get_mount_point().to_str().expect("Optional Disk name returned None"); 
        let fs = str::from_utf8(disk.get_file_system()).unwrap();
        (String::from(name), String::from(fs), disk.get_total_space() - disk.get_available_space(), disk.get_total_space())
    }
}
