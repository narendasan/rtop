use rtop::app::App;
use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Gauge, Sparkline, Widget};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Modifier, Style};


pub fn draw_gauges(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Block::default()
        .borders(Borders::ALL)
        .title("Graphs")
        .render(t, area);
    Group::default()
        .direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Fixed(2), Size::Fixed(3)])
        .render(t, area, |t, chunks| {
            Gauge::default()
                .block(Block::default().title("Gauge:"))
                .style(
                    Style::default()
                        .fg(Color::Magenta)
                        .bg(Color::Black)
                        .modifier(Modifier::Italic),
                )
                .label(&format!("{} / 100", app.progress))
                .percent(app.progress)
                .render(t, &chunks[0]);
            Sparkline::default()
                .block(Block::default().title("Sparkline:"))
                .style(Style::default().fg(Color::Green))
                .data(&app.data)
                .render(t, &chunks[1]);
        });
}