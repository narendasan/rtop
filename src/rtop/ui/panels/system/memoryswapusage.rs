use crate::rtop::app::App;

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::symbols::Marker;
use ratatui::text::Span;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset};
use ratatui::Frame;

pub fn mem_and_swap_history_panel(f: &mut Frame, app: &App, area: Rect) {
    let datasets = vec![
        Dataset::default()
            .name(app.mem_usage_str.clone())
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::LightRed))
            .data(&app.mem_panel_memory),
        Dataset::default()
            .name(app.swap_usage_str.clone())
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::LightCyan))
            .data(&app.swap_panel_memory),
    ];

    let mem_history_panel = Chart::new(datasets.clone())
        .block(
            Block::default()
                .title("Memory and Swap Usage")
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

    f.render_widget(mem_history_panel, area);
}
