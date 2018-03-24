//#![feature(nll)]
#[macro_use]
extern crate log;
extern crate stderrlog;
extern crate termion;
extern crate tui;
extern crate sysinfo;

mod rtop;

use std::io;
use std::thread;
use std::time;
use std::sync::mpsc;
use std::collections::HashMap;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::style::Color;

use rtop::app::App;
use rtop::cmd::Cmd;
use rtop::ui::event::Event;
use rtop::ui::tabs::Tabs as rTabs;
use rtop::ui::renderer::render::render;
use rtop::datastreams::randomsignal::RandomSignal;
use rtop::datastreams::servers::Servers;
use rtop::datastreams::systemmonitor::SystemMonitor;



fn main() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(4)
        .init()
        .unwrap();

    info!("Start");

    //DATA STREAMS
    let mut rand_signal = RandomSignal::new(0, 100);

    //Program
    let mut app = App {
        items: vec![
            "Item1", "Item2", "Item3", "Item4", 
        ],
        events: vec![("Event1", "INFO"),],
        selected: 0,
        tabs: rTabs {
            titles: vec!["Tab0", "Tab1"],
            selection: 0,
        },
        show_chart: true,
        progress: 0,
        data: rand_signal.clone().take(300).collect(),
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
        ],
        window: [0.0, 20.0],
        colors: [Color::Magenta, Color::Red],
        color_index: 0,
        servers: Servers::new(),
        cpu_panel_memory: HashMap::new(),
        sys_info: SystemMonitor::new(100 as usize),
    };
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    //Key Watcher
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
    ////////////////////////////////////////

    //CLK
    thread::spawn(move || {
        let tx = tx.clone();
        loop {
            tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(200));
        }
    });
    ////////////////////////////////////////

    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    let mut term_size = terminal.size().unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    loop {
        let size = terminal.size().unwrap();
        if size != term_size {
            terminal.resize(size).unwrap();
            term_size = size;
        }
        render(&mut terminal, &app, &term_size).unwrap();
        let evt = rx.recv().unwrap();
        {
            match evt {
                Event::Input(input) => {
                    match app.input_handler(input) {
                        Some(command) => {
                            match command {
                                Cmd::Quit => {break},
                                _ => (),
                            }
                        },
                        None => (),
                    }
                },

                Event::Tick => {
                    app.update(&mut rand_signal);
                } 
            }
        }
    }
    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
}

//Need to add back in q for quit, make sure resizing works and polling for sys monitor