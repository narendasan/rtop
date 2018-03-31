use rtop::app::App;
use rtop::ui::renderer::panels::cpuusage::render_cpu_usage_history;
use rtop::ui::renderer::panels::processes::render_processes;
use rtop::ui::renderer::panels::memoryswapusage::render_mem_and_swap_history;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{BarChart, Block, Borders, Item, List,
                   SelectableList, Widget};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Modifier, Style};


pub fn draw_charts(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    let sizes = vec![Size::Percent(35), Size::Percent(65)];
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&sizes)
        .render(t, area, |t, chunks| {
                render_sidebar(t, app, &chunks[0]);
                render_cpu_usage_history(t, app, &chunks[1]);
        });
}

fn render_sidebar(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, area, |t, chunks| {
            render_processes(t, app, &chunks[0]);
            render_mem_and_swap_history(t, app, &chunks[1]);
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
