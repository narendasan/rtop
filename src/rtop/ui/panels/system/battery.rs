use crate::rtop::app::App;

use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Block, Gauge, Paragraph, Text, Borders, Widget};
use tui::layout::{Layout, Direction, Rect, Constraint};
use tui::style::{Color, Modifier, Style};

#[cfg(feature = "battery-monitor")]
pub fn battery_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    Block::default()
        .borders(Borders::ALL)
        .title("Battery")
        .render(f, area);

    let sub_areas = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);
    
    Gauge::default()
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
               }))
        .render(f, sub_areas[0]);
    
    Paragraph::new()
        .block(
            Block::default()
                .borders(Borders::NONE)
        )
        .wrap(true)
        .text(Text::raw(format!("\n{}\n", app.battery_status).as_str()))
        .render(f, sub_area[1]);
}

