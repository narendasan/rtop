use crate::rtop::app::App;
use crate::rtop::ui::panels::gpu::*;

use tui::Frame;
use tui::backend::Backend;
use tui::layout::{Direction, Layout, Rect, Constraint};

pub fn render_gpu_tab<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
     let sub_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(5),
            Constraint::Percentage(15),
            Constraint::Percentage(65),
            Constraint::Percentage(15)
        ].as_ref())
        .split(area);

    driver_panel(f, app, sub_areas[0]);
    device_panel(f, app, sub_areas[1]);
    render_charts(f, app, sub_areas[2]);
    processes_panel(f, app, sub_areas[3]);
}

fn render_charts<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let sub_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)].as_ref())
        .split(area);
        
    mem_history_panel(f, app, sub_areas[0]);
    temp_history_panel(f, app, sub_areas[1]);
    power_history_panel(f, app, sub_areas[2]);
}
