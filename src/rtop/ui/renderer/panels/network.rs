use rtop::app::App;
use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Gauge, Sparkline, Widget};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Modifier, Style};

pub fn render_network_info(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
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
                .data(&app.sys_info.net_out_history)
                .max(100000)
                .render(t, &chunks[0]);
            Sparkline::default()
                .block(Block::default().title(&app.net_out_str))
                .style(Style::default().fg(Color::Green))
                .data(&app.sys_info.net_out_history)
                .max(100000)
                .render(t, &chunks[2]);
        });
}