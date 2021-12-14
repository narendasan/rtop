use crate::rtop::app::App;

use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Block, Gauge, Paragraph, Wrap, Borders};
use tui::layout::{Layout, Direction, Rect, Constraint};
use tui::style::{Color, Style};
use tui::text::Text;

#[cfg(feature = "battery-monitor")]
pub fn battery_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let panel = Block::default()
        .borders(Borders::ALL)
        .title("Battery");


    let sub_areas = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(60)
            ].as_ref())
        .split(area);

    let battery_chart = Gauge::default()
        .block(Block::default()
               .borders(Borders::NONE))
        .percent(app.battery_level as u16)
        .style(Style::default()
               .fg(if app.battery_level < 20.0 {
                   Color::LightRed
               } else if app.battery_level < 40.0 {
                   Color::LightYellow
               } else {
                   Color::LightGreen
               }));

    let content = format!("\n{}\n", app.battery_status);
    let text = Text::from(content.as_str());
    let status = Paragraph::new(text)
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap {trim: false});

    f.render_widget(panel, area);
    f.render_widget(battery_chart, sub_areas[0]);
    f.render_widget(status, sub_areas[1]);
}

