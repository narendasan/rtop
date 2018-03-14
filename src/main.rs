#[macro_use]
extern crate log;
extern crate stderrlog;
extern crate termion;
extern crate tui;

mod rtop;

use std::io;
use std::thread;
use std::time;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::layout::Rect;
use tui::style::Color;

use rtop::App;
use rtop::ui::event::Event;
use rtop::ui::tabs::Tabs as rTabs;
use rtop::ui::renderer::draw::draw;
use rtop::datastreams::randomsignal::RandomSignal;
use rtop::datastreams::sinsignal::SinSignal;
use rtop::datastreams::servers::Servers;



fn main() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(4)
        .init()
        .unwrap();

    info!("Start");

    //DATA STREAMS
    let mut rand_signal = RandomSignal::new(0, 100);
    let mut sin_signal = SinSignal::new(0.2, 3.0, 18.0);
    let mut sin_signal2 = SinSignal::new(0.1, 2.0, 10.0);

    //Program
    let mut app = App {
        size: Rect::default(),
        items: vec![
            "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9",
            "Item10", "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17",
            "Item18", "Item19", "Item20", "Item21", "Item22", "Item23", "Item24",
        ],
        events: vec![("Event1", "INFO"),
                     ("Event2", "INFO"),
                     ("Event3", "CRITICAL"),
                     ("Event4", "ERROR"),],
        selected: 0,
        tabs: rTabs {
            titles: vec!["Tab0", "Tab1"],
            selection: 0,
        },
        show_chart: true,
        progress: 0,
        data: rand_signal.clone().take(300).collect(),
        data2: sin_signal.clone().take(100).collect(),
        data3: sin_signal2.clone().take(200).collect(),
        data4: vec![
            ("B1", 9),
            ("B2", 12),
            ("B3", 5),
            ("B4", 8),
            ("B5", 2),
            ("B6", 4),
            ("B7", 5),
            ("B8", 9),
            ("B9", 14),
            ("B10", 15),
            ("B11", 1),
            ("B12", 0),
            ("B13", 4),
            ("B14", 6),
            ("B15", 4),
            ("B16", 6),
            ("B17", 4),
            ("B18", 7),
            ("B19", 13),
            ("B20", 8),
            ("B21", 11),
            ("B22", 9),
            ("B23", 3),
            ("B24", 5),
        ],
        window: [0.0, 20.0],
        colors: [Color::Magenta, Color::Red],
        color_index: 0,
        servers: Servers::new(),
    };
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    for _ in 0..100 {
        sin_signal.next();
    }
    for _ in 0..200 {
        sin_signal2.next();
    }

    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    thread::spawn(move || {
        let tx = tx.clone();
        loop {
            tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(200));
        }
    });

    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    loop {
        ////////////////////////////////////////////////
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }
        ////////////////////////////////////////////////
        draw(&mut terminal, &app).unwrap();
        ////////////////////////////////////////////////
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => match input {
                event::Key::Char('q') => {
                    break;
                }
                event::Key::Up => {
                    if app.selected > 0 {
                        app.selected -= 1
                    };
                }
                event::Key::Down => if app.selected < app.items.len() - 1 {
                    app.selected += 1;
                },
                event::Key::Left => {
                    app.tabs.previous();
                }
                event::Key::Right => {
                    app.tabs.next();
                }
                event::Key::Char('t') => {
                    app.show_chart = !app.show_chart;
                }
                _ => {}
            },
            Event::Tick => {
                app.progress += 5;
                if app.progress > 100 {
                    app.progress = 0;
                }
                app.data.insert(0, rand_signal.next().unwrap());
                app.data.pop();
                for _ in 0..5 {
                    app.data2.remove(0);
                    app.data2.push(sin_signal.next().unwrap());
                }
                for _ in 0..10 {
                    app.data3.remove(0);
                    app.data3.push(sin_signal2.next().unwrap());
                }
                let i = app.data4.pop().unwrap();
                app.data4.insert(0, i);
                app.window[0] += 1.0;
                app.window[1] += 1.0;
                let i = app.events.pop().unwrap();
                app.events.insert(0, i);
                app.color_index += 1;
                if app.color_index >= app.colors.len() {
                    app.color_index = 0;
                }
            }
        }
        ////////////////////////////////////////////////
    }
    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
}