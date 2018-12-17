
use crate::rtop::app::App;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Row, Table, Widget};
use tui::layout::Rect;
use tui::style::{Color, Style};

pub fn processes_panel(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    let mut processes_by_gpu = app.gpu_info.processes.clone();
    //processes_by_gpu.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    let selected_style = Style::default().fg(Color::White).bg(Color::Green);
    let default_style = Style::default().fg(Color::Cyan);
    Table::new(
        ["Device", "PID", "Command", "Mem (MB)", "Type"].into_iter(),
        processes_by_gpu.iter().enumerate().map(|(i, p)| {
            let style = if i == app.selected_proc {
                &selected_style
            } else {
                &default_style
            };
            Row::StyledData(vec![p.device_id.to_string(), 
                                 p.pid.to_string(), 
                                 p.name.to_string(), 
                                 bytes_to_mb(p.mem), 
                                 p.proc_type.to_string()].into_iter(), style)
        }),
    ).block(Block::default().title("Processes").borders(Borders::ALL))
                            .header_style(Style::default().fg(Color::Yellow))
                            .widths(&[15, 15, 10, 10])
                            .render(t, &area);  
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
