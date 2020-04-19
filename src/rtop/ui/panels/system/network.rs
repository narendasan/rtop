use crate::rtop::app::App;
use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Block, Borders, Sparkline};
use tui::layout::{Direction, Layout, Rect, Constraint};
use tui::style::{Color, Style};

pub fn network_info_panel<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let panel = Block::default()
        .borders(Borders::ALL)
        .title("Network");

    let interfaces = app.datastreams.net_info.net_in.keys().map(|x| x.to_string()).collect::<Vec<String>>();
    let num_interfaces = interfaces.len();

    println!("{:?}", num_interfaces);
    
    let interface_areas = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints((1..num_interfaces)
                     .map(|_| Constraint::Percentage((100.0 / num_interfaces as f32) as u16))
                     .collect::<Vec<Constraint>>())
        .split(area);
    
    for (interface, interface_area) in interfaces.iter().zip(interface_areas.iter()) {
        let sub_areas = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40)
            ].as_ref())
            .split(*interface_area);
        
        let net_in_sparkline = Sparkline::default()
            .block(Block::default().title(&app.net_in_strs[interface]))
            .style(Style::default().fg(Color::Green))
            .data(&app.datastreams.net_info.net_in_history[interface])
            .max(1000000);
        
        let net_out_sparkline = Sparkline::default()
            .block(Block::default().title(&app.net_out_strs[interface]))
            .style(Style::default().fg(Color::Green))
            .data(&app.datastreams.net_info.net_out_history[interface])
            .max(1000000);
        
        f.render_widget(panel, area);
        f.render_widget(net_in_sparkline, sub_areas[0]);
        f.render_widget(net_out_sparkline, sub_areas[2]);
    }
}
