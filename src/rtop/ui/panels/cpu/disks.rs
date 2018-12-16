use crate::rtop::app::App;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, BarChart, Borders, Widget};
use tui::layout::{Group, Direction, Rect, Size};
use tui::style::{Color, Modifier, Style};

pub fn disk_usage_panel(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    let num_drives = app.disk_info.disk_usage.len() as u16;
    let gauge_width: u16 = 100 / num_drives;
    let sizes = (0..num_drives).map(|_| {Size::Percent(gauge_width)})
                                .collect::<Vec<Size>>();

    Block::default()
        .borders(Borders::ALL)
        .title("Disk Usage")
        .render(t, area);
    Group::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .sizes(&sizes)
        .render(t, area, |t, chunks| {
            for drive_num in 0..app.disk_info.disk_usage.len() {
                let drive = &app.disk_info.disk_usage[drive_num];
                let label = drive.0.clone() + &(" (%)").to_string();
                let usage = (drive.2 as f64 / drive.3 as f64) * 100.0;
                Group::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .sizes(&[Size::Percent(50)])
                    .render(t, &chunks[drive_num], |t, chunk| {
                        BarChart::default()
                            .block(Block::default()
                                        .title(&(label))
                                        .borders(Borders::NONE))
                            .data(&[(&format!("{:.1}%", usage), usage as u64)])
                            .bar_width(5)
                            .bar_gap(0)
                            .max(105)
                            .style(Style::default()
                                    .fg(if usage < 60.0 { 
                                            Color::LightGreen
                                        } else if usage < 85.0 { 
                                            Color::LightYellow
                                        } else {
                                            Color::LightRed
                                        }))
                            .value_style(Style::default()
                                            .bg(if usage < 60.0 { 
                                                    Color::LightGreen
                                                } else if usage < 85.0 { 
                                                    Color::LightYellow
                                                } else {
                                                    Color::LightRed
                                                }).modifier(Modifier::Bold))
                            .render(t, &chunk[0]);
                    });
            }
        });
}

