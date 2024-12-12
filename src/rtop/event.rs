use crossterm::event;

pub enum Event {
    Input(event::KeyEvent),
    Scroll(event::MouseEvent),
    Tick,
}
