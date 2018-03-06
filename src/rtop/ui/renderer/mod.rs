fn draw(t: &mut Terminal<MouseBackend>, app: &App) -> Result<(), io::Error> {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(3), Size::Min(0)])
        .render(t, &app.size, |t, chunks| {
            Tabs::default()
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .titles(&app.tabs.titles)
                .style(Style::default().fg(Color::Green))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(app.tabs.selection)
                .render(t, &chunks[0]);
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

fn draw_first_tab(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(7), Size::Min(7), Size::Fixed(7)])
        .render(t, area, |t, chunks| {
            draw_gauges(t, app, &chunks[0]);
            draw_charts(t, app, &chunks[1]);
            draw_text(t, &chunks[2]);
        });
}

fn draw_gauges(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Block::default()
        .borders(Borders::ALL)
        .title("Graphs")
        .render(t, area);
    Group::default()
        .direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Fixed(2), Size::Fixed(3)])
        .render(t, area, |t, chunks| {
            Gauge::default()
                .block(Block::default().title("Gauge:"))
                .style(
                    Style::default()
                        .fg(Color::Magenta)
                        .bg(Color::Black)
                        .modifier(Modifier::Italic),
                )
                .label(&format!("{} / 100", app.progress))
                .percent(app.progress)
                .render(t, &chunks[0]);
            Sparkline::default()
                .block(Block::default().title("Sparkline:"))
                .style(Style::default().fg(Color::Green))
                .data(&app.data)
                .render(t, &chunks[1]);
        });
}

fn draw_charts(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    let sizes = if app.show_chart {
        vec![Size::Percent(50), Size::Percent(50)]
    } else {
        vec![Size::Percent(100)]
    };
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&sizes)
        .render(t, area, |t, chunks| {
            Group::default()
                .direction(Direction::Vertical)
                .sizes(&[Size::Percent(50), Size::Percent(50)])
                .render(t, &chunks[0], |t, chunks| {
                    Group::default()
                        .direction(Direction::Horizontal)
                        .sizes(&[Size::Percent(50), Size::Percent(50)])
                        .render(t, &chunks[0], |t, chunks| {
                            SelectableList::default()
                                .block(Block::default().borders(Borders::ALL).title("List"))
                                .items(&app.items)
                                .select(app.selected)
                                .highlight_style(
                                    Style::default().fg(Color::Yellow).modifier(Modifier::Bold),
                                )
                                .highlight_symbol(">")
                                .render(t, &chunks[0]);
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
                                .render(t, &chunks[1]);
                        });
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
                        .render(t, &chunks[1]);
                });
            if app.show_chart {
                Chart::default()
                    .block(
                        Block::default()
                            .title("Chart")
                            .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::Bold))
                            .borders(Borders::ALL),
                    )
                    .x_axis(
                        Axis::default()
                            .title("X Axis")
                            .style(Style::default().fg(Color::Gray))
                            .labels_style(Style::default().modifier(Modifier::Italic))
                            .bounds(app.window)
                            .labels(&[
                                &format!("{}", app.window[0]),
                                &format!("{}", (app.window[0] + app.window[1]) / 2.0),
                                &format!("{}", app.window[1]),
                            ]),
                    )
                    .y_axis(
                        Axis::default()
                            .title("Y Axis")
                            .style(Style::default().fg(Color::Gray))
                            .labels_style(Style::default().modifier(Modifier::Italic))
                            .bounds([-20.0, 20.0])
                            .labels(&["-20", "0", "20"]),
                    )
                    .datasets(&[
                        Dataset::default()
                            .name("data2")
                            .marker(Marker::Dot)
                            .style(Style::default().fg(Color::Cyan))
                            .data(&app.data2),
                        Dataset::default()
                            .name("data3")
                            .marker(Marker::Braille)
                            .style(Style::default().fg(Color::Yellow))
                            .data(&app.data3),
                    ])
                    .render(t, &chunks[1]);
            }
        });
}

fn draw_text(t: &mut Terminal<MouseBackend>, area: &Rect) {
    Paragraph::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Footer")
                .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::Bold)),
        )
        .wrap(true)
        .text(
            "This is a paragraph with several lines.\nYou can change the color.\nUse \
             \\{fg=[color];bg=[color];mod=[modifier] [text]} to highlight the text with a color. \
             For example, {fg=red u}{fg=green n}{fg=yellow d}{fg=magenta e}{fg=cyan r} \
             {fg=gray t}{fg=light_gray h}{fg=light_red e} {fg=light_green r}{fg=light_yellow a} \
             {fg=light_magenta i}{fg=light_cyan n}{fg=white b}{fg=red o}{fg=green w}.\n\
             Oh, and if you didn't {mod=italic notice} you can {mod=bold automatically} \
             {mod=invert wrap} your {mod=underline text} =).\nOne more thing is that \
             it should display unicode characters properly: 日本国, ٩(-̮̮̃-̃)۶ ٩(●̮̮̃•̃)۶ ٩(͡๏̯͡๏)۶ \
             ٩(-̮̮̃•̃).",
        )
        .render(t, area);
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
                app.servers.iter().map(|s| {
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
                    for (i, s1) in app.servers.iter().enumerate() {
                        for s2 in &app.servers[i + 1..] {
                            ctx.draw(&Line {
                                x1: s1.coords.1,
                                y1: s1.coords.0,
                                y2: s2.coords.0,
                                x2: s2.coords.1,
                                color: Color::Yellow,
                            });
                        }
                    }
                    for server in &app.servers {
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