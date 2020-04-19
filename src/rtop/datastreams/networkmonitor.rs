use std::collections::HashMap;
use sysinfo::{System, SystemExt, NetworkExt};

use crate::rtop::datastreams::datastream::SysDataStream;


pub struct NetworkMonitor {
    pub net_in_history: HashMap<String, Vec<u64>>,
    pub net_out_history: HashMap<String, Vec<u64>>, 
    pub net_in: HashMap<String, u64>,
    pub net_out: HashMap<String, u64>, 
    max_sparkline_len: usize,
}

impl SysDataStream for NetworkMonitor {
    fn new(max_hist_len: usize, _inter_len: u16) -> Self {        
        Self {
            net_in_history: HashMap::new(),
            net_out_history: HashMap::new(), 
            net_in: HashMap::new(), //bits
            net_out: HashMap::new(), //bits
            max_sparkline_len: max_hist_len,
        }
    }

    fn poll(&mut self, system_info: &System) {
        let networks = system_info.get_networks();

        for (interface, network) in networks {
            self.net_in.insert(interface.to_string(), network.get_received() * 8);
            self.net_out.insert(interface.to_string(), network.get_transmitted() * 8);
            
            let (inc, out) = NetworkMonitor::parse_networking_info((self.net_in[interface], self.net_out[interface]));

            let in_history = self.net_in_history.entry(interface.to_string())
                                                .or_insert(vec![0; self.max_sparkline_len]);
            while in_history.len() >= self.max_sparkline_len {
                in_history.remove(0);
            }
            in_history.push(inc);
            
            let out_history = self.net_out_history.entry(interface.to_string())
                                                  .or_insert(vec![0; self.max_sparkline_len]);
            while out_history.len() >= self.max_sparkline_len {
                out_history.remove(0);
            }
            out_history.push(out);            
        }
    }
}

impl NetworkMonitor {
    fn parse_networking_info(net: (u64, u64)) -> (u64, u64) {
        let (mut inc, mut out) = net;
        if inc == 0 {
            inc = 10;
        }
        if out == 0 {
            out = 10;
        }
        (inc, out)
    }
}
