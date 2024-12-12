use crate::rtop::app::App;

use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Gauge, Paragraph, Wrap};
use ratatui::Frame;

#[cfg(feature = "battery-monitor")]
use crate::rtop::datastreams::ChargingStatus;

#[cfg(feature = "battery-monitor")]
pub fn battery_panel(f: &mut Frame, app: &App, area: Rect) {
    let panel = Block::default().borders(Borders::ALL).title("Battery");

    let sub_areas = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    let battery_chart = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .percent(app.battery_level as u16)
        .style(Style::default().fg(if app.battery_level < 20.0 {
            Color::LightRed
        } else if app.battery_level < 40.0 {
            Color::LightYellow
        } else {
            Color::LightGreen
        }));

    let content = vec![
        Line::from(""),
        match app.datastreams.battery_info.charging_status {
            ChargingStatus::Discharging(time) => {
                let remaining_time = App::time_from_secs(time);
                Line::from(format!("🔋 On Battery (Time to empty: {})", remaining_time))
            }
            ChargingStatus::Charging(time) => {
                let remaining_time = App::time_from_secs(time);
                Line::from(format!("⚡ Charging (Time to full: {})", remaining_time))
            }
            ChargingStatus::Full => Line::from("🔌  Connected to Power"),
            ChargingStatus::Empty => Line::from("😵 Empty Battery"),
            ChargingStatus::Unknown => Line::from("Unknown"),
        },
        Line::from(format!(
            "⚕️ Battery Health: {:.2}% (Cycle count: {})",
            app.datastreams.battery_info.health, app.datastreams.battery_info.cycle_count
        )),
        Line::from(format!(
            "〽️ Power Draw: {:.2}W ⚡ Voltage: {:.2}V 🌡  Temperature: {}",
            app.datastreams.battery_info.power_draw,
            app.datastreams.battery_info.voltage,
            app.datastreams.battery_info.temp
        )),
        Line::from(format!(
            "Battery Energy: {:.2}/{:.2}Wh (Designed Capacity: {:.2}Wh)",
            app.datastreams.battery_info.energy,
            app.datastreams.battery_info.energy_full,
            app.datastreams.battery_info.designed_energy_full
        )),
        Line::from(format!(
            "Model: {} Serial: {} Kind: {}",
            app.datastreams.battery_info.model,
            app.datastreams.battery_info.serial,
            app.datastreams.battery_info.kind
        )),
        Line::from(""),
    ];

    let status = Paragraph::new(content)
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: true });

    f.render_widget(panel, area);
    f.render_widget(battery_chart, sub_areas[0]);
    f.render_widget(status, sub_areas[1]);
}
