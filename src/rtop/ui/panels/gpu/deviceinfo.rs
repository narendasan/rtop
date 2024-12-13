use crate::rtop::app::App;

use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Row, Table};
use ratatui::Frame;

pub fn device_panel(f: &mut Frame, app: &App, area: Rect) {
    let default_style = Style::default().fg(Color::Cyan);
    let rows = app.datastreams.gpu_info.device_info.iter().map(|(_, gpu)| {
        let style = &default_style;
        Row::new(vec![
            gpu.id.to_string(),
            gpu.name.clone(),
            gpu.bus_id.clone(),
            bytes_to_gb(gpu.max_memory),
            gpu.vbios.clone(),
            format!("{}x", gpu.num_pcie_lanes),
            format!("{} MHz", gpu.max_sm_clock),
            format!("{} MHz", gpu.max_mem_clock),
            match gpu.power_limit {
                0 => "Unknown".to_string(),
                _ => format!("{} W", gpu.power_limit),
            },
        ])
        .style(*style)
    });
    let widths = [
        Constraint::Length(3),
        Constraint::Length(25),
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
    ];
    let device_table = Table::new(rows, widths)
        .header(
            Row::new(vec![
                "Device",
                "Name",
                "Bus ID",
                "Memory",
                "VBIOS",
                "PCIe Connection",
                "Max SM Clock",
                "Max Memory Clock",
                "Power Limit",
            ])
            .style(Style::default().fg(Color::Yellow)),
        )
        .block(Block::default().title("Devices").borders(Borders::ALL));

    f.render_widget(device_table, area);
}

fn bytes_to_gb(mem: u64) -> String {
    format!("{:.0} GB", (mem / 1000000000))
}
