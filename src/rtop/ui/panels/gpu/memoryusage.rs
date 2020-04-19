use crate::rtop::app::App;

use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset};
use tui::symbols::Marker;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};

pub fn mem_history_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let mut data = app.gpu_mem_panel_memory.iter().map(|x| {
        (x.0.clone(), (x.1).0.clone(), (x.1).1.clone())
    }).collect::<Vec<(u32, String, Vec<(f64, f64)>)>>();
    
    data.sort_by_key(|k| k.0);

    let datasets = data.iter().map(|x| {
        Dataset::default()
            .name(&x.1)
            .marker(Marker::Braille)
            .style(Style::default().fg(color_map(x.0)))
            .data(&x.2)
    }).collect::<Vec<Dataset>>();
    
    let mem_usage = Chart::default()
        .block(
            Block::default()
                .title("GPU Memory Usage")
                .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("")
                .style(Style::default().fg(Color::Gray))
                .labels_style(Style::default().modifier(Modifier::ITALIC))
                .bounds(app.window)
                .labels(&[""]),
        )
        .y_axis(
            Axis::default()
                .title("Usage (%)")
                .style(Style::default().fg(Color::Gray))
                .labels_style(Style::default().modifier(Modifier::ITALIC))
                .bounds([0.0, 1.0])
                .labels(&["0", "20", "40", "60", "80", "100"]),
        )
        .datasets(&datasets);

    f.render_widget(mem_usage, area);
}



fn color_map(key: u32) -> Color {
    match key % 10 {
        0 => {Color::LightRed},
        1 => {Color::LightGreen},
        2 => {Color::LightYellow},
        3 => {Color::LightMagenta},
        _ => {Color::White},
    }
}
