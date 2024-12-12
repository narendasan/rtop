use crate::rtop::app::App;
use crate::rtop::ui::panels::system::*;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

pub fn render_system_tab(f: &mut Frame, app: &App, area: Rect) {
    let sub_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    render_top_half(f, app, sub_areas[0]);
    render_charts(f, app, sub_areas[1]);
}

pub fn render_charts(f: &mut Frame, app: &App, area: Rect) {
    let sub_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(area);

    render_sidebar(f, app, sub_areas[0]);
    cpu_usage_history_panel(f, app, sub_areas[1]);
}

fn render_sidebar(f: &mut Frame, app: &App, area: Rect) {
    if cfg!(feature = "battery-monitor") {
        let sub_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(35),
                    Constraint::Percentage(40),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
            )
            .split(area);

        disk_usage_panel(f, app, sub_areas[0]);
        network_info_panel(f, app, sub_areas[1]);
        #[cfg(feature = "battery-monitor")]
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

fn render_top_half(f: &mut Frame, app: &App, area: Rect) {
    let sub_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    render_top_left_corner(f, app, sub_areas[0]);
    mem_and_swap_history_panel(f, app, sub_areas[1]);
}

fn render_top_left_corner(f: &mut Frame, app: &App, area: Rect) {
    processes_panel(f, app, area);
}
