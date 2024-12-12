use crate::rtop::app::App;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Sparkline};
use ratatui::Frame;

pub fn network_info_panel(f: &mut Frame, app: &App, area: Rect) {
    let panel = Block::default().borders(Borders::ALL).title("Network");

    let sub_areas = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(area);

    let net_in = Sparkline::default()
        .block(Block::default().title(&app.net_in_str))
        .style(Style::default().fg(Color::Green))
        .data(&app.datastreams.net_info.net_in_history)
        .max(1_000_000);

    let net_out = Sparkline::default()
        .block(Block::default().title(&app.net_out_str))
        .style(Style::default().fg(Color::Green))
        .data(&app.datastreams.net_info.net_out_history)
        .max(1_000_000);

    f.render_widget(panel, area);
    f.render_widget(net_in, sub_areas[0]);
    f.render_widget(net_out, sub_areas[1]);
}
