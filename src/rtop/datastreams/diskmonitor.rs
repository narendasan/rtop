use sysinfo::{Disk, Disks, System};

use crate::rtop::datastreams::datastream::SysDataStream;

pub struct DiskMonitor {
    pub disk_usage: Vec<(String, String, u64, u64)>, //Mount, type, used, total
    disks: Disks,
}

impl SysDataStream for DiskMonitor {
    fn new(_max_hist_len: usize, _inter_len: u16) -> Self {
        Self {
            disk_usage: Vec::new(),
            disks: Disks::new_with_refreshed_list(),
        }
    }

    fn poll(&mut self, _: &System) {
        self.disks.refresh(false);
        self.disk_usage.clear();
        for disk in self.disks.list() {
            self.disk_usage.push(DiskMonitor::parse_disk_info(disk));
        }
    }
}

impl DiskMonitor {
    fn parse_disk_info(disk: &Disk) -> (String, String, u64, u64) {
        let name = disk
            .mount_point()
            .to_str()
            .expect("Optional Disk name returned None");
        let fs = disk.file_system().to_str().unwrap().to_string();
        (
            String::from(name),
            fs,
            disk.total_space() - disk.available_space(),
            disk.total_space(),
        )
    }
}
