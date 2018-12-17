
use crate::rtop::app::App;
use crate::rtop::ui::panels::gpu::*;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::Widget;
use tui::layout::{Direction, Group, Rect, Size};

pub fn render_gpu_tab(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
     Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(100)])
        .render(t, area, |t, chunks| {
            processes_panel(t, app, &chunks[0]);
        });
}
