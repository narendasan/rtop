use crate::rtop::app::App;
use crate::rtop::ui::panels::gpu::*;

use tui::Frame;
use tui::backend::Backend;
use tui::layout::{Direction, Layout, Rect, Constraint};

pub fn render_gpu_tab<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
     let sub_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(area);
        
    processes_panel(f, app, sub_areas[0]);
    render_bottom_third(f, app, sub_areas[1])
}

fn render_bottom_third<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let sub_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);
        
    mem_history_panel(f, app, sub_areas[0]);
    temp_history_panel(f, app, sub_areas[1]);
}
