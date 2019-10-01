use crate::rtop::app::App;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Marker, Widget};
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};

pub fn temp_history_panel(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    let mut data = app.gpu_temp_panel_memory.iter()
                                            .map(|x| {(x.0.clone(), (x.1).0.clone(), (x.1).1.clone())})
                                            .collect::<Vec<(u32, String, Vec<(f64, f64)>)>>();
    data.sort_by_key(|k| k.0);


    Chart::default()
        .block(
            Block::default()
                .title("GPU Temperatures")
                .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::Bold))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("")
                .style(Style::default().fg(Color::Gray))
                .labels_style(Style::default().modifier(Modifier::Italic))
                .bounds(app.window)
                .labels(&[""]),
        )
        .y_axis(
            Axis::default()
                .title("Temperature (°C)")
                .style(Style::default().fg(Color::Gray))
                .labels_style(Style::default().modifier(Modifier::Italic))
                .bounds([20.0, 130.0])
                .labels(&["10", "50", "90", "130"]),
        )
        .datasets(&data.iter().map(|x| {
                        Dataset::default()
                            .name(&x.1)
                            .marker(Marker::Braille)
                            .style(Style::default().fg(color_map(x.0)))
                            .data(&x.2)
                    }).collect::<Vec<Dataset>>())
        .render(t, area);
}



fn color_map(key: u32) -> Color {
    match key % 10 {
        0 => {Color::LightGreen},
        1 => {Color::LightYellow},
        2 => {Color::LightRed},
        3 => {Color::LightBlue},
        _ => {Color::White},
    }
}