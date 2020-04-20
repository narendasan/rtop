use crate::rtop::app::App;

use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Block, Borders, Row, Table};
use tui::layout::{Rect, Constraint};
use tui::style::{Color, Style};

pub fn device_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let default_style = Style::default().fg(Color::Cyan);
    let device_table = Table::new(
        ["Device", "Name", "Bus ID", "Memory", "VBIOS", "PCIe Connection", "Max SM Clock", "Max Memory Clock", "Power Limit"].iter(),
        app.datastreams.gpu_info.device_info.iter().map(|(_, gpu)| {
            let style = &default_style;
            Row::StyledData(vec![gpu.id.to_string(), 
                                 gpu.name.clone(), 
                                 gpu.bus_id.clone(), 
                                 bytes_to_gb(gpu.max_memory), 
                                 gpu.vbios.clone(),
                                 format!("{}x", gpu.num_pcie_lanes),
                                 format!("{} MHz", gpu.max_sm_clock),
                                 format!("{} MHz", gpu.max_mem_clock),
                                 format!("{} W", gpu.power_limit),
            ].into_iter(), *style)
        }),
    ).block(Block::default().title("Devices").borders(Borders::ALL))
        .header_style(Style::default().fg(Color::Yellow))
        .widths(&[
            Constraint::Length(3),
            Constraint::Length(25),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(20),
        ]);

    f.render_widget(device_table, area);  
}

fn bytes_to_gb(mem: u64) -> String {
    format!("{:.0} GB", (mem / 1000000000)) 
}
