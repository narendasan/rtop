use tui::layout::Rect;
use tui::style::Color;


pub mod datastreams;
pub mod server;
pub mod ui;

use self::server::Server;
use self::ui::tabs::Tabs;

pub struct App<'a> {
    pub size: Rect,
    pub items: Vec<&'a str>,
    pub events: Vec<(&'a str, &'a str)>,
    pub selected: usize,
    pub tabs: Tabs<'a>,
    pub show_chart: bool,
    pub progress: u16,
    pub data: Vec<u64>,
    pub data2: Vec<(f64, f64)>,
    pub data3: Vec<(f64, f64)>,
    pub data4: Vec<(&'a str, u64)>,
    pub window: [f64; 2],
    pub colors: [Color; 2],
    pub color_index: usize,
    pub servers: Vec<Server<'a>>,
}