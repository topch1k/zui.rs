use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{palette::tailwind, Stylize},
    symbols,
    text::Line,
    widgets::{Block, ListState, Padding, Paragraph, Widget},
};
use zookeeper_async::Stat;

use crate::node_data::NodeData;
#[derive(Debug, Default)]
pub struct Tab {
    pub tab_data: Vec<String>,
    pub list_state: ListState,
    pub curr_resource: Option<String>, // selected node for current nest level
    pub prev_resources: Vec<String>,   // prev resources: e.g. /zookeeper/config
    pub current_node_stat: Option<Stat>,
    pub message: String,
    pub node_data: NodeData,
    pub node_path_buf: String,
    pub node_data_buf: String,
    pub input_buf: String,
}

impl Tab {
    pub fn title(&self) -> Line<'static> {
        format!("  Tab  ")
            .fg(tailwind::SLATE.c200)
            .bg(tailwind::BLUE.c900)
            .into()
    }

    pub fn render_tab(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Hello, World!")
            .block(self.block())
            .render(area, buf);
    }

    pub fn block(&self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(tailwind::BLUE.c700)
    }
}
