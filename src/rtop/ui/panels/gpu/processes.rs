use crate::rtop::app::App;
use crate::rtop::ui::panels::utils;

use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Row, Table};
use ratatui::Frame;

pub fn processes_panel(f: &mut Frame, app: &App, area: Rect) {
    let (selected_proc, processes_to_display) = utils::scrolling(
        area,
        app.selected_gpu_proc,
        &app.datastreams.gpu_info.processes[..],
    );

    let selected_style = Style::default().fg(Color::White).bg(Color::Green);
    let default_style = Style::default().fg(Color::Cyan);
    let rows = processes_to_display.iter().enumerate().map(|(i, p)| {
        let style = if i == selected_proc {
            &selected_style
        } else {
            &default_style
        };
        Row::new(vec![
            p.device_id.to_string(),
            p.pid.to_string(),
            p.name.to_string(),
            bytes_to_mb(p.mem),
            p.proc_type.to_string(),
        ])
        .style(*style)
    });

    let widths = [
        Constraint::Length(7),
        Constraint::Length(5),
        Constraint::Length(15),
        Constraint::Length(10),
    ];

    let proc_table = Table::new(rows, widths)
        .header(
            Row::new(vec!["Device", "PID", "Command", "Mem (MB)", "Type"])
                .style(Style::default().fg(Color::Yellow)),
        )
        .block(Block::default().title("Processes").borders(Borders::ALL));

    f.render_widget(proc_table, area);
}

fn bytes_to_mb(mem: Option<u64>) -> String {
    match mem {
        Some(m) => {
            format!("{:.0} MB", (m / 1000000))
        }
        None => "UNKNOWN".to_string(),
    }
}
