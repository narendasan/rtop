
use crate::rtop::app::App;
use crate::rtop::ui::panels::gpu::*;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::Widget;
use tui::layout::{Direction, Group, Rect, Size};

pub fn render_gpu_tab(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
     Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(60), Size::Percent(40)])
        .render(t, area, |t, chunks| {
            processes_panel(t, app, &chunks[0]);
            render_bottom_third(t, app, &chunks[1])
        });
}

fn render_bottom_third(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, area, |t, chunks| {
            mem_history_panel(t, app, &chunks[0]);
            temp_history_panel(t, app, &chunks[1]);
        });
}