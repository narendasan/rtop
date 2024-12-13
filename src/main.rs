#[macro_use]
extern crate log;

use std::io;
use std::process::exit;
use std::sync::mpsc;
use std::thread;
use std::time;

use crossterm::event;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal;
use ratatui::prelude::CrosstermBackend;

use ratatui::Terminal;

mod rtop;

use crate::rtop::app::App;
use crate::rtop::cmd::Cmd;
use crate::rtop::error::Error;
use crate::rtop::event::Event;
use crate::rtop::ui::renderer::render;

fn main() {
    exit(match _main() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn _main() -> Result<(), Error> {
    stderrlog::new()
        .module(module_path!())
        .verbosity(4)
        .init()
        .unwrap();

    debug!("Start");
    #[cfg(feature = "gpu-monitor")]
    debug!("GPU Monitoring Enabled");
    #[cfg(feature = "battery-monitor")]
    debug!("Battery Monitoring Enabled");
    //Program
    let mut app = App::new(5000, 50)?;
    #[cfg(feature = "gpu-monitor")]
    app.init()?;
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    thread::spawn(move || -> Result<_, Error> {
        loop {
            if event::poll(time::Duration::from_millis(500))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press {
                        input_tx.send(Event::Input(key.code)).unwrap();
                        if key.code == event::KeyCode::Char('q') {
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    });

    thread::spawn(move || {
        let tx = tx;
        loop {
            tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(2000));
        }
    });

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen,)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let clk_split = 0;

    loop {
        let evt = rx.recv()?;
        match evt {
            Event::Scroll(mouse_scroll) => match mouse_scroll {
                event::MouseEventKind::ScrollUp => {
                    app.input_handler(event::KeyCode::Up);
                }
                event::MouseEventKind::ScrollDown => {
                    app.input_handler(event::KeyCode::Down);
                }
                _ => {}
            },
            Event::Input(input) => {
                if let Some(command) = app.input_handler(input) {
                    match command {
                        Cmd::Quit => break,
                        //_ => (),
                    }
                }
            }
            Event::Tick => {
                if clk_split % 2 == 0 {
                    app.update()?;
                }
            }
        }
        render(&mut terminal, &app)?;
    }
    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
    terminal::disable_raw_mode()?;
    Ok(())
}
