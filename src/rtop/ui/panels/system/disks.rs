use crate::rtop::app::App;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{BarChart, Block, Borders};
use ratatui::Frame;

pub fn disk_usage_panel(f: &mut Frame, app: &App, area: Rect) {
    let num_drives = app.datastreams.disk_info.disk_usage.len() as u16;
    let gauge_width: u16 = 100 / num_drives;
    let constraints = (0..num_drives)
        .map(|_| Constraint::Percentage(gauge_width))
        .collect::<Vec<Constraint>>();

    let panel = Block::default().borders(Borders::ALL).title("Disk Usage");

    let sub_areas = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(constraints.as_ref())
        .split(area);

    for (drive_num, a) in sub_areas
        .iter()
        .enumerate()
        .take(app.datastreams.disk_info.disk_usage.len())
    {
        let drive = &app.datastreams.disk_info.disk_usage[drive_num];
        let label = drive.0.clone() + &(" (%)").to_string();
        let usage = (drive.2 as f64 / drive.3 as f64) * 100.0;
        let chunk = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(50)].as_ref())
            .split(*a);

        let data = [(&format!("{:.1}%", usage)[..], usage as u64)];

        let chart = BarChart::default()
            .block(Block::default().title(&(label)).borders(Borders::NONE))
            .data(&data)
            .bar_width(5)
            .bar_gap(0)
            .max(105)
            .style(Style::default().fg(if usage < 60.0 {
                Color::LightGreen
            } else if usage < 85.0 {
                Color::LightYellow
            } else {
                Color::LightRed
            }))
            .value_style(
                Style::default()
                    .bg(if usage < 60.0 {
                        Color::LightGreen
                    } else if usage < 85.0 {
                        Color::LightYellow
                    } else {
                        Color::LightRed
                    })
                    .modifier(Modifier::BOLD),
            );

        f.render_widget(panel, area);
        f.render_widget(chart, chunk[0]);
    }
}
