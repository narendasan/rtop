use crate::rtop::app::App;
use crate::rtop::ui::panels::system::*;

use tui::Frame;
use tui::backend::Backend;
use tui::layout::{Direction, Layout, Rect, Constraint};

pub fn render_system_tab<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let sub_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    render_top_half(f, app, sub_areas[0]);
    render_charts(f, app, sub_areas[1]);
}

pub fn render_charts<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let sub_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(area);

    render_sidebar(f, app, sub_areas[0]);
    cpu_usage_history_panel(f, app, sub_areas[1]);
}

fn render_sidebar<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    if cfg!(feature = "battery-monitor") {
        let sub_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(35), Constraint::Percentage(40), Constraint::Percentage(25)].as_ref())
            .split(area);

            disk_usage_panel(f, app, sub_areas[0]);
            network_info_panel(f, app, sub_areas[1]);
            battery_panel(f, app, sub_areas[2]);
    } else {
        let sub_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        disk_usage_panel(f, app, sub_areas[0]);
        network_info_panel(f, app, sub_areas[1]);
    }
}

fn render_top_half<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let sub_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    render_top_left_corner(f, app, sub_areas[0]);
    mem_and_swap_history_panel(f, app, sub_areas[1]);
}

fn render_top_left_corner<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    processes_panel(f, app, area);
}
