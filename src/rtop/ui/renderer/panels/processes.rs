
use rtop::app::App;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Row, Table, Widget};
use tui::layout::Rect;
use tui::style::{Color, Style};


pub fn render_processes(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    let mut processes_by_cpu = app.sys_info.process_info.clone();
    processes_by_cpu.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let selected_style = Style::default().fg(Color::White).bg(Color::Green);
    let default_style = Style::default().fg(Color::Cyan);
    Table::new(
        ["PID", "Command", "%CPU▲", "Mem (KB)"].into_iter(),
        processes_by_cpu.iter().enumerate().map(|(i, s)| {
            let style = if i == app.selected_proc {
                &selected_style
            } else {
                &default_style
            };
            //println!("{}", s.2);
            Row::StyledData(vec![s.0.to_string(), s.1.to_string(), s.2.to_string(), s.3.to_string()].into_iter(), style)
        }),
    ).block(Block::default().title("Processes").borders(Borders::ALL))
        .header_style(Style::default().fg(Color::Yellow))
        .widths(&[15, 15, 10, 10])
        .render(t, &area);  
}