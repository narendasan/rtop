use tui::style::Color;

pub fn color_map(key: u32) -> Color {
    match key % 4 {
        0 => Color::LightGreen,
        1 => Color::LightYellow,
        2 => Color::LightRed,
        3 => Color::LightBlue,
        _ => Color::White,
    }
}
