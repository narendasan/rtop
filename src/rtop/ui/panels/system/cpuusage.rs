use crate::rtop::app::App;

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::symbols::Marker;
use ratatui::text::Span;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset};
use ratatui::Frame;

pub fn cpu_usage_history_panel(f: &mut Frame, app: &App, area: Rect) {
    let mut data = app
        .cpu_panel_memory
        .iter()
        .map(|x| (*x.0, (x.1).0.clone(), (x.1).1.clone()))
        .collect::<Vec<(u32, String, Vec<(f64, f64)>)>>();
    data.sort_by_key(|k| k.0);
    //println!("{:?}", data);
    let datasets = &data
        .iter()
        .map(|x| {
            //println!("{:?}", &x.2);
            Dataset::default()
                .name(&*x.1)
                .marker(Marker::Braille)
                .style(Style::default().fg(color_map(x.0)))
                .data(&x.2)
        })
        .collect::<Vec<Dataset>>();

    let cpu_usage = Chart::new(datasets.clone())
        .block(
            Block::default()
                .title("CPU Usage")
                .title_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("")
                .style(Style::default().fg(Color::Gray))
                .bounds(app.window)
                .labels(["".italic()]),
        )
        .y_axis(
            Axis::default()
                .title("Usage (%)")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0])
                .labels(
                    ["0", "20", "40", "60", "80", "100"]
                        .into_iter()
                        .map(|x| x.italic())
                        .collect::<Vec<Span>>(),
                ),
        );

    f.render_widget(cpu_usage, area);
}

fn color_map(key: u32) -> Color {
    match key % 10 {
        0 => Color::Red,
        1 => Color::Green,
        2 => Color::Yellow,
        3 => Color::Magenta,
        4 => Color::Cyan,
        5 => Color::LightRed,
        6 => Color::LightGreen,
        7 => Color::LightYellow,
        8 => Color::LightMagenta,
        9 => Color::LightCyan,
        _ => Color::White,
    }
}
