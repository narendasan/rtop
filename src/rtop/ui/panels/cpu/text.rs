use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Paragraph, Widget};
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};

pub fn text_panel(t: &mut Terminal<MouseBackend>, area: &Rect) {
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
             it should display unicode characters properly: 😃 日本国, ٩(-̮̮̃-̃)۶ ٩(●̮̮̃•̃)۶ ٩(͡๏̯͡๏)۶ \
             ٩(-̮̮̃•̃).",
        )
        .render(t, area);
}