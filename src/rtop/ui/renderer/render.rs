use crate::rtop::app::App;
use crate::rtop::ui::renderer::*;
use std::io;

use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Tabs};
use ratatui::{Frame, Terminal};

pub fn render<B: Backend>(t: &mut Terminal<B>, app: &App) -> Result<(), io::Error> {
    t.draw(|f| {
        let sub_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(f.area());

        render_tab_bar(f, app, sub_areas[0]);
        #[allow(clippy::single_match)]
        match app.tabs.selection {
            0 => {
                system_tab::render_system_tab(f, app, sub_areas[1]);
            }
            #[cfg(feature = "gpu-monitor")]
            1 => {
                gpu_tab::render_gpu_tab(&mut f, app, sub_areas[1]);
            }
            _ => {}
        };
    });
    Ok(())
}

fn render_tab_bar(f: &mut Frame, app: &App, area: Rect) {
    let tabs = Tabs::default()
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .titles(app.tabs.titles.clone())
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.selection);

    f.render_widget(tabs, area);
}
