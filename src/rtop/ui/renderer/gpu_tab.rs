
use std::io;
use rtop::app::App;
use rtop::ui::panels::gpu::*;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Row, Table, Tabs, Widget};
use tui::widgets::canvas::{Canvas, Line, Map, MapResolution};
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Style};

pub fn render_gpu_tab(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    
}