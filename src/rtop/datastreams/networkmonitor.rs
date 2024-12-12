use crate::rtop::datastreams::datastream::SysDataStream;
use sysinfo::{Networks, System};

pub struct NetworkMonitor {
    nets: Networks,
    pub net_in_history: Vec<u64>,
    pub net_out_history: Vec<u64>,
    pub net_in: u64,
    pub net_out: u64,
    max_sparkline_len: usize,
}

impl SysDataStream for NetworkMonitor {
    fn new(_max_hist_len: usize, _inter_len: u16) -> Self {
        Self {
            net_in_history: Vec::new(),
            net_out_history: Vec::new(),
            net_in: 0,  //in bits
            net_out: 0, //in bits
            max_sparkline_len: 50,
            nets: Networks::new_with_refreshed_list(),
        }
    }

    fn poll(&mut self, _: &System) {
        self.nets.refresh(true);
        self.net_in = self.nets.iter().fold(0_u64, |sum, (_, data)| -> u64 {
            sum + data.received()
        }) * 8_u64;
        self.net_out = self.nets.iter().fold(0_u64, |sum, (_, data)| -> u64 {
            sum + data.transmitted()
        }) * 8_u64;

        let (inc, out) = NetworkMonitor::parse_networking_info((self.net_in, self.net_out));

        while self.net_in_history.len() >= self.max_sparkline_len {
            self.net_in_history.remove(0);
        }
        self.net_in_history.push(inc);

        while self.net_out_history.len() >= self.max_sparkline_len {
            self.net_out_history.remove(0);
        }
        self.net_out_history.push(out);
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
