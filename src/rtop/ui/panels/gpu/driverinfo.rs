use crate::rtop::app::App;

use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text};
use tui::Frame;

pub fn driver_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let text = [
        Text::raw(format!(
            " Driver Version: {}\n",
            app.datastreams.gpu_info.driver_version
        )),
        Text::raw(format!(
            " CUDA Version: {}",
            app.datastreams.gpu_info.cuda_version
        )),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title("System Info")
        .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD));

    let sys_info = Paragraph::new(text.iter()).block(block);
    f.render_widget(sys_info, area);
}
