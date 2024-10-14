use crate::{
    app::{state::TabState, BASE_RESOURCE},
    node_data::NodeData,
};
use ratatui::{
    style::{palette::tailwind, Color, Stylize},
    text::Line,
    widgets::ListState,
};
use zookeeper_async::Stat;
#[derive(Debug)]
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
    pub state: TabState,
    pub toggle_stats_auto_load: bool,
}

impl Default for Tab {
    fn default() -> Self {
        Self {
            tab_data: Default::default(),
            list_state: ListState::default().with_selected(Some(0usize)),
            curr_resource: Some(BASE_RESOURCE.to_string()),
            prev_resources: Default::default(),
            current_node_stat: Default::default(),
            message: Default::default(),
            node_data: Default::default(),
            node_path_buf: Default::default(),
            node_data_buf: Default::default(),
            input_buf: Default::default(),
            state: Default::default(),
            toggle_stats_auto_load: true,
        }
    }
}

impl Tab {
    pub fn title(&self) -> Line<'static> {
        "  Tab  "
            .to_string()
            .fg(tailwind::SLATE.c200)
            .bg(tailwind::BLUE.c900)
            .into()
    }

    pub fn highlite_style() -> (Color, Color) {
        (Color::default(), tailwind::AMBER.c700)
    }
}
