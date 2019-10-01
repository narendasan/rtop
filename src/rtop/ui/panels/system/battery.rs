use crate::rtop::app::App;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Gauge, Paragraph, Borders, Widget};
use tui::layout::{Group, Direction, Rect, Size};
use tui::style::{Color, Modifier, Style};

#[cfg(feature = "battery-monitor")]
pub fn battery_panel(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Block::default()
        .borders(Borders::ALL)
        .title("Battery")
        .render(t, area);

    Group::default()
        .direction(Direction::Vertical)
        .margin(2)
        .sizes(&[Size::Percent(40), Size::Percent(60)])
        .render(t, area, |t, chunks| {
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
                .render(t, &chunks[0]);
                Paragraph::default()
                    .block(
                        Block::default()
                            .borders(Borders::NONE)
                    )
                    .wrap(true)
                    .text(format!("\n{}\n", app.battery_status).as_str())
                    .render(t, &chunks[1]);
        });
}

