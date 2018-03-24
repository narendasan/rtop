use rtop::app::App;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Marker, Widget};
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};

pub fn render_cpu_usage(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Chart::default()
        .block(
            Block::default()
                .title("CPU Usage")
                .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::Bold))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .labels_style(Style::default().modifier(Modifier::Italic))
                .bounds(app.window)
                .labels(&[
                    &format!("{}", app.window[0]),
                    &format!("{}", (app.window[0] + app.window[1]) / 2.0),
                    &format!("{}", app.window[1]),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .labels_style(Style::default().modifier(Modifier::Italic))
                .bounds([0.0, 100.0])
                .labels(&["0", "20", "40", "60", "80", "100"]),
        )
        .datasets(&app.cpu_panel_memory.iter().enumerate().map(|x| {
                Dataset::default()
                    .name((x.1).0)
                    .marker(Marker::Dot)
                    .style(Style::default().fg(color_map(x.0)))
                    .data((x.1).1)
            }).collect::<Vec<Dataset>>().as_slice())
        .render(t, area);
}

fn color_map(key: usize) -> Color {
    match key % 10 {
        0 => {Color::Red},
        1 => {Color::Green},
        2 => {Color::Yellow},
        3 => {Color::Magenta},
        4 => {Color::Cyan},
        5 => {Color::LightRed},
        6 => {Color::LightGreen},
        7 => {Color::LightYellow},
        8 => {Color::LightMagenta},
        9 => {Color::LightCyan},
        _ => {Color::White},
    }
}