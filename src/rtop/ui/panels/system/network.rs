use crate::rtop::app::App;
use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Block, Borders, Sparkline, Widget};
use tui::layout::{Direction, Layout, Rect, Constraint};
use tui::style::{Color, Style};

pub fn network_info_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    Block::default()
        .borders(Borders::ALL)
        .title("Network")
        .render(f, area);
    
    let sub_areas = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(20), Constraint::Percentage(40)].as_ref())
        .split(area);
    
    Sparkline::default()
        .block(Block::default().title(&app.net_in_str))
        .style(Style::default().fg(Color::Green))
        .data(&app.datastreams.net_info.net_in_history)
        .max(1000000)
        .render(f, sub_areas[0]);
    Sparkline::default()
        .block(Block::default().title(&app.net_out_str))
        .style(Style::default().fg(Color::Green))
        .data(&app.datastreams.net_info.net_out_history)
        .max(1000000)
        .render(f, sub_areas[2]);
}
