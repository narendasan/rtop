
use std::io;
use crate::rtop::app::App;
use crate::rtop::ui::renderer::*;

use tui::{Terminal, Frame};
use tui::backend::Backend;
use tui::widgets::{Block, Borders, Tabs};
use tui::layout::{Direction, Layout, Rect, Constraint};
use tui::style::{Color, Style};
use tui::text::Spans;

pub fn render<B: Backend>(t: &mut Terminal<B>, app: &App) -> Result<(), io::Error> {
    t.draw(|mut f| {
        let sub_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(f.size());

        render_tab_bar(&mut f, app, sub_areas[0]);
        #[allow(clippy::single_match)]
        match app.tabs.selection {
            0 => {
                system_tab::render_system_tab(&mut f, app, sub_areas[1]);
            }
            #[cfg(feature = "gpu-monitor")]
            1 => {
                gpu_tab::render_gpu_tab(&mut f, app, sub_areas[1]);
            }
            _ => {}
        };
    })?;
    Ok(())
}


fn render_tab_bar<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let titles = app.tabs.titles.iter().cloned().map(Spans::from).collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.selection);

    f.render_widget(tabs, area);
}
