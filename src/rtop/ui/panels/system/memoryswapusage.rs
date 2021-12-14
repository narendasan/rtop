use crate::rtop::app::App;

use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset};
use tui::symbols::Marker;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;

pub fn mem_and_swap_history_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let datasets = vec![
        Dataset::default()
            .name(&app.mem_usage_str)
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::LightRed))
            .data(&app.mem_panel_memory),
        Dataset::default()
            .name(&app.swap_usage_str)
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::LightCyan))
            .data(&app.swap_panel_memory)
    ];

    let mem_history_panel = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled("Memory and Swap Usage", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("")
                .style(Style::default().fg(Color::Gray))
                .bounds(app.window)
                .labels(vec![Span::styled("", Style::default().add_modifier(Modifier::ITALIC))]),
        )
        .y_axis(
            Axis::default()
                .title("Usage (%)")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 1.0])
                .labels(["0", "20", "40", "60", "80", "100"].iter()
                    .map(|v| Span::styled(v.to_string(), Style::default().add_modifier(Modifier::ITALIC)))
                    .collect::<Vec<Span>>())
        );

    f.render_widget(mem_history_panel, area);
}
