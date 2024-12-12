use crossterm::event;

pub enum Event {
    Input(event::KeyCode),
    Scroll(event::MouseEventKind),
    Tick,
}
