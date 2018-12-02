use rtop::app::App;
use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Sparkline, Widget};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Style};

pub fn network_info_panel(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Block::default()
        .borders(Borders::ALL)
        .title("Network")
        .render(t, area);
    Group::default()
        .direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Percent(40), Size::Percent(20), Size::Percent(40)])
        .render(t, area, |t, chunks| {
            Sparkline::default()
                .block(Block::default().title(&app.net_in_str))
                .style(Style::default().fg(Color::Green))
                .data(&app.net_info.net_in_history)
                .max(1000000)
                .render(t, &chunks[0]);
            Sparkline::default()
                .block(Block::default().title(&app.net_out_str))
                .style(Style::default().fg(Color::Green))
                .data(&app.net_info.net_out_history)
                .max(1000000)
                .render(t, &chunks[2]);
        });
}