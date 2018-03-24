
use std::io;
use rtop::app::App;
use rtop::ui::renderer::panels;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Row, Table, Tabs, Widget};
use tui::widgets::canvas::{Canvas, Line, Map, MapResolution};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Style};

pub fn render(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) -> Result<(), io::Error> {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(3), Size::Min(0)])
        .render(t, area, |t, chunks| {
            //render_tab_bar(t, app, &chunks[0]);
            match app.tabs.selection {
                0 => {
                    draw_first_tab(t, app, &chunks[1]);
                }
                1 => {
                    draw_second_tab(t, app, &chunks[1]);
                }
                _ => {}
            };
        });
    try!(t.draw());
    Ok(())
}

fn render_tab_bar(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Tabs::default()
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .titles(&app.tabs.titles)
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.selection)
        .render(t, area);
}

fn draw_first_tab(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(34), Size::Percent(33), Size::Percent(33)])
        .render(t, area, |t, chunks| {
            panels::charts::draw_charts(t, app, &chunks[0]);
            //panels::gauges::draw_gauges(t, app, &chunks[1]);
            //panels::text::render_text(t, &chunks[2]);
        });
}

fn draw_second_tab(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&[Size::Percent(30), Size::Percent(70)])
        .render(t, area, |t, chunks| {
            let up_style = Style::default().fg(Color::Green);
            let failure_style = Style::default().fg(Color::Red);
            Table::new(
                ["Server", "Location", "Status"].into_iter(),
                app.servers.nodes.iter().map(|s| {
                    let style = if s.status == "Up" {
                        &up_style
                    } else {
                        &failure_style
                    };
                    Row::StyledData(vec![s.name, s.location, s.status].into_iter(), style)
                }),
            ).block(Block::default().title("Servers").borders(Borders::ALL))
                .header_style(Style::default().fg(Color::Yellow))
                .widths(&[15, 15, 10])
                .render(t, &chunks[0]);

            Canvas::default()
                .block(Block::default().title("World").borders(Borders::ALL))
                .paint(|ctx| {
                    ctx.draw(&Map {
                        color: Color::White,
                        resolution: MapResolution::High,
                    });
                    ctx.layer();
                    for (i, s1) in app.servers.nodes.iter().enumerate() {
                        for s2 in &app.servers.nodes[i + 1..] {
                            ctx.draw(&Line {
                                x1: s1.coords.1,
                                y1: s1.coords.0,
                                y2: s2.coords.0,
                                x2: s2.coords.1,
                                color: Color::Yellow,
                            });
                        }
                    }
                    for server in &app.servers.nodes {
                        let color = if server.status == "Up" {
                            Color::Green
                        } else {
                            Color::Red
                        };
                        ctx.print(server.coords.1, server.coords.0, "X", color);
                    }
                })
                .x_bounds([-180.0, 180.0])
                .y_bounds([-90.0, 90.0])
                .render(t, &chunks[1]);
        })
}