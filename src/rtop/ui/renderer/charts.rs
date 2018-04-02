use rtop::app::App;
use rtop::ui::panels::{cpu_usage_history_panel, processes_panel, 
                       disk_usage_panel};

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{BarChart, Block, Borders, Widget};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Modifier, Style};


pub fn render_charts(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    let sizes = vec![Size::Percent(35), Size::Percent(65)];
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&sizes)
        .render(t, area, |t, chunks| {
                sidebar(t, app, &chunks[0]);
                cpu_usage_history_panel(t, app, &chunks[1]);
        });
}

fn sidebar(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, area, |t, chunks| {
            processes_panel(t, app, &chunks[0]);
            disk_usage_panel(t, app, &chunks[1]);
        });
}