use rtop::app::App;
use rtop::ui::panels::{cpu_usage_history_panel, processes_panel, 
                       mem_and_swap_history_panel};

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{BarChart, Block, Borders, Item, List,
                   SelectableList, Widget};
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
            mem_and_swap_history_panel(t, app, &chunks[1]);
        });
}

fn render_bar_graph(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Bar chart"))
        .data(&app.data4)
        .bar_width(3)
        .bar_gap(2)
        .value_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .modifier(Modifier::Italic),
        )
        .label_style(Style::default().fg(Color::Yellow))
        .style(Style::default().fg(Color::Green))
        .render(t, area);
}
