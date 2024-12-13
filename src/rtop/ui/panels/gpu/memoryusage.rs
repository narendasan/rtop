use crate::rtop::app::App;
use crate::rtop::ui::panels::gpu::utils::color_map;

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::symbols::Marker;
use ratatui::text::Span;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset};
use ratatui::Frame;

pub fn mem_history_panel(f: &mut Frame, app: &App, area: Rect) {
    let mut data = app
        .gpu_mem_panel_memory
        .iter()
        .map(|x| (x.0.clone(), (x.1).0.clone(), (x.1).1.clone()))
        .collect::<Vec<(u32, String, Vec<(f64, f64)>)>>();

    data.sort_by_key(|k| k.0);

    let datasets = data
        .iter()
        .map(|x| {
            Dataset::default()
                .name(&*x.1)
                .marker(Marker::Braille)
                .style(Style::default().fg(color_map(x.0)))
                .data(&x.2)
        })
        .collect::<Vec<Dataset>>();

    let mem_usage = Chart::new(datasets)
        .block(
            Block::default()
                .title("GPU Memory Usage")
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
                .bounds([0.0, 1.0])
                .labels(
                    ["0", "20", "40", "60", "80", "100"]
                        .into_iter()
                        .map(|x| x.italic())
                        .collect::<Vec<Span>>(),
                ),
        );

    f.render_widget(mem_usage, area);
}
