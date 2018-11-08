
use std::io;
use rtop::app::App;
use rtop::ui::panels::gpu::*;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Row, Table, Tabs, Widget};
use tui::widgets::canvas::{Canvas, Line, Map, MapResolution};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Style};

pub fn render_gpu_tab(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
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