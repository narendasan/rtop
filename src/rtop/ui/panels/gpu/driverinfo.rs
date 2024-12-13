use crate::rtop::app::App;

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn driver_panel(f: &mut Frame, app: &App, area: Rect) {
    let text = vec![
        Line::from(format!(
            " Driver Version: {}\n",
            app.datastreams.gpu_info.driver_version
        )),
        Line::from(format!(
            " CUDA Version: {}",
            app.datastreams.gpu_info.cuda_version
        )),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title("System Info")
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    let sys_info = Paragraph::new(text).block(block);
    f.render_widget(sys_info, area);
}
