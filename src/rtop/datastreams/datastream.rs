extern crate sysinfo;

use self::sysinfo::System;

pub trait DataStream {
    fn new(max_hist_len: usize) -> Self;
    fn poll(&mut self, system_info: &System); 
}