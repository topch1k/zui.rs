use crate::node_data::NodeData;
use ratatui::{
    style::{palette::tailwind, Color, Stylize},
    text::Line,
    widgets::ListState,
};
use zookeeper_async::Stat;
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

    pub fn highlite_style() -> (Color, Color) {
        (Color::default(), tailwind::AMBER.c700)
    }
}
