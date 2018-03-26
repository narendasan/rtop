use rtop::app::App;
use rtop::ui::renderer::panels::cpuusage::render_cpu_usage;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{BarChart, Block, Borders, Item, List,
                   SelectableList, Widget};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Modifier, Style};


pub fn draw_charts(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    let sizes = vec![Size::Percent(20), Size::Percent(80)];
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&sizes)
        .render(t, area, |t, chunks| {
                render_sidebar(t, app, &chunks[0]);
                render_cpu_usage(t, app, &chunks[1]);
        });
}

fn render_sidebar(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, area, |t, chunks| {
            render_lists(t, app, &chunks[0]);
            render_bar_graph(t, app, &chunks[1]);
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

fn render_lists(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, area, |t, chunks| {
            render_selctable_list(t, app, &chunks[0]);
            render_streams(t, app, &chunks[1]);
        });
}

fn render_selctable_list(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    SelectableList::default()
        .block(Block::default().borders(Borders::ALL).title("List"))
        .items(&app.items)
        .select(app.selected)
        .highlight_style(
            Style::default().fg(Color::Yellow).modifier(Modifier::Bold),
        )
        .highlight_symbol(">")
        .render(t, area);
}

fn render_streams(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    let info_style = Style::default().fg(Color::White);
            let warning_style = Style::default().fg(Color::Yellow);
            let error_style = Style::default().fg(Color::Magenta);
            let critical_style = Style::default().fg(Color::Red);
            let events = app.events.iter().map(|&(evt, level)| {
                Item::StyledData(
                    format!("{}: {}", level, evt),
                    match level {
                        "ERROR" => &error_style,
                        "CRITICAL" => &critical_style,
                        "WARNING" => &warning_style,
                        _ => &info_style,
                    },
                )
            });
            List::new(events)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .render(t, area);
}