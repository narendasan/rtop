//#![feature(nll)]
#[macro_use]
extern crate log;
extern crate stderrlog;
extern crate termion;
extern crate tui;
extern crate sysinfo;
#[cfg(feature = "gpu-monitor")]
extern crate nvml_wrapper as nvml;

mod rtop;

use std::io;
use std::thread;
use std::time;
use std::sync::mpsc;
use std::process::exit;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::MouseBackend;

use rtop::app::App;
use rtop::cmd::Cmd;
use rtop::event::Event;
use rtop::ui::renderer::render::render;

fn main() {
    ::std::process::exit(match _main() {
       Ok(_) => 0,
       Err(err) => {
           eprintln!("error: {:?}", err);
           1
       }
    });
}

fn _main() -> Result<(), String> {
    stderrlog::new().module(module_path!())
                    .verbosity(4)
                    .init()
                    .unwrap();

    info!("Start");
    //Program
    let mut app = App::new(150)?;
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

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
            thread::sleep(time::Duration::from_millis(1000));
        }
    });

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
                    app.update();
                } 
            }
        }
        render(&mut terminal, &app, &term_size).unwrap();
    }
    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
    Ok(())
}