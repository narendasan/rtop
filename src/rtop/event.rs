use termion::event;

pub enum Event {
    Input(event::Key),
    Tick,
}