use crate::rtop::app::App;
use crate::rtop::ui::panels::utils;

use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Row, Table};
use ratatui::Frame;

pub fn processes_panel(f: &mut Frame, app: &App, area: Rect) {
    let mut processes_by_cpu = app.datastreams.process_info.processes.clone();
    processes_by_cpu.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    let (selected_proc, processes_to_display) =
        utils::scrolling(area, app.selected_proc, &processes_by_cpu[..]);

    let selected_style = Style::default().fg(Color::White).bg(Color::Green);
    let default_style = Style::default().fg(Color::Cyan);
    let proc_table = Table::new(
        ["PID", "Command", "%CPU▲", "Mem (KB)"].iter(),
        processes_to_display.iter().enumerate().map(|(i, s)| {
            let style = if i == selected_proc {
                &selected_style
            } else {
                &default_style
            };
            Row::StyledData(
                vec![
                    s.0.to_string(),
                    s.1.to_string(),
                    format!("{:.2}", s.2),
                    s.3.to_string(),
                ]
                .into_iter(),
                *style,
            )
        }),
    )
    .block(Block::default().title("Processes").borders(Borders::ALL))
    .header_style(Style::default().fg(Color::Yellow))
    .widths(
        [
            Constraint::Length(10),
            Constraint::Length(25),
            Constraint::Length(10),
            Constraint::Length(10),
        ]
        .as_ref(),
    );

    f.render_widget(proc_table, area);
}
