use crate::rtop::app::App;
use crate::rtop::ui::panels::utils;

use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Block, Borders, Row, Table, Widget};
use tui::layout::{Rect, Constraint};
use tui::style::{Color, Style};

pub fn processes_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let (selected_proc, processes_to_display) = utils::scrolling(area, app.selected_gpu_proc, &app.datastreams.gpu_info.processes[..]);

    let selected_style = Style::default().fg(Color::White).bg(Color::Green);
    let default_style = Style::default().fg(Color::Cyan);
    Table::new(
        ["Device", "PID", "Command", "Mem (MB)", "Type"].into_iter(),
        processes_to_display.iter().enumerate().map(|(i, p)| {
            let style = if i == selected_proc {
                &selected_style
            } else {
                &default_style
            };
            Row::StyledData(vec![p.device_id.to_string(), 
                                 p.pid.to_string(), 
                                 p.name.to_string(), 
                                 bytes_to_mb(p.mem), 
                                 p.proc_type.to_string()].into_iter(), *style)
        }),
    ).block(Block::default().title("Processes").borders(Borders::ALL))
                            .header_style(Style::default().fg(Color::Yellow))
                            .widths(&[Constraint::Length(7), Constraint::Length(5), Constraint::Length(15), Constraint::Length(10)])
                            .render(f, area);  
}

fn bytes_to_mb(mem: Option<u64>) -> String {
    match mem {
        Some(m) => {
            format!("{:.0} MB", (m / 1000000)) 
        }, 
        None => {
            "UNKNOWN".to_string()
        }
    }
}
