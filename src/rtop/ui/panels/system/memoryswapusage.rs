use crate::rtop::app::App;

use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset};
use tui::symbols::Marker;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};

pub fn mem_and_swap_history_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let datasets = [
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
    
    let mem_history_panel = Chart::default()
        .block(
            Block::default()
                .title("Memory and Swap Usage")
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

    f.render_widget(mem_history_panel, area);
}
