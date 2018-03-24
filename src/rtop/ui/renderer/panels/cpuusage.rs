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
        .datasets(&app.cpu_panel_memory.iter().map(|x| {
                Dataset::default()
                    .name(x.0)
                    .marker(Marker::Dot)
                    .style(Style::default().fg(Color::Cyan))
                    .data(x.1)
            }).collect::<Vec<Dataset>>())
        .render(t, area);
}