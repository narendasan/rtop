use crate::rtop::app::App;
use crate::rtop::ui::panels::gpu::utils::color_map;

use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::symbols::Marker;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset};
use tui::Frame;

pub fn temp_history_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let mut data = app
        .gpu_temp_panel_memory
        .iter()
        .map(|x| (x.0.clone(), (x.1).0.clone(), (x.1).1.clone()))
        .collect::<Vec<(u32, String, Vec<(f64, f64)>)>>();

    data.sort_by_key(|k| k.0);

    let datasets = data
        .iter()
        .map(|x| {
            Dataset::default()
                .name(&x.1)
                .marker(Marker::Braille)
                .style(Style::default().fg(color_map(x.0)))
                .data(&x.2)
        })
        .collect::<Vec<Dataset>>();

    let temp = Chart::default()
        .block(
            Block::default()
                .title("GPU Temperatures")
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
                .title("Temperature (°C)")
                .style(Style::default().fg(Color::Gray))
                .labels_style(Style::default().modifier(Modifier::ITALIC))
                .bounds([20.0, 130.0])
                .labels(&["10", "50", "90", "130"]),
        )
        .datasets(&datasets);

    f.render_widget(temp, area);
}
