
use std::io;
use crate::rtop::app::App;
use crate::rtop::ui::renderer::*;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Tabs, Widget};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Style};

pub fn render(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) -> Result<(), io::Error> {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(3), Size::Min(0)])
        .render(t, area, |t, chunks| {
            render_tab_bar(t, app, &chunks[0]);
            match app.tabs.selection {
                0 => {
                    cpu_tab::render_cpu_tab(t, app, &chunks[1]);
                }
                #[cfg(feature = "gpu-monitor")]
                1 => {
                    gpu_tab::render_gpu_tab(t, app, &chunks[1]);
                }
                _ => {}
            };
        });
    t.draw()?;
    Ok(())
}


fn render_tab_bar(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Tabs::default()
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .titles(&app.tabs.titles)
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.selection)
        .render(t, area);
}
