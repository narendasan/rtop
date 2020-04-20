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
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    render_half(f, app, sub_areas[0], vec![&utilization_history_panel, &mem_history_panel]);
    render_half(f, app, sub_areas[1], vec![&temp_history_panel, &power_history_panel]);
}

fn render_half<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect, panels: Vec<&dyn Fn(&mut Frame<B>, &App, Rect)>) {
    let sub_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    for (p, a) in panels.iter().zip(sub_areas.iter()) {
        p(f, app, *a);
    }
}
        
