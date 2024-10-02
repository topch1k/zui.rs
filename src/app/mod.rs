pub mod connection;
pub mod navigation;
pub mod render;
pub mod state;
pub mod zk_ops;
use crate::{node_data::NodeData, tab::Tab};
use connection::Connection;
use ratatui::{text::Line, widgets::Tabs};
use state::AppState;
use zookeeper_async::Stat;

pub const BASE_RESOURCE: &str = "/";
pub const CONFIRMATION_STRING: &str = "DELETE";
#[derive(Default)]
pub struct App {
    pub state: AppState,
    pub connection: Option<Connection>,
    pub zk: Option<zookeeper_async::ZooKeeper>,
    pub connection_input: String,
    pub curr_tab: usize,
    pub tabs: Vec<Tab>,
}

impl App {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection_input: connection.to_string(),
            connection: Some(connection),
            tabs: vec![Tab::default(), Tab::default(), Tab::default()],
            curr_tab: 0usize,
            ..Default::default()
        }
    }
    pub fn connection_str(&self) -> String {
        match self.connection {
            Some(ref conn) => conn.to_string(),
            None => "".to_owned(),
        }
    }

    pub fn tabs_titles(&self) -> impl Iterator<Item = Line> {
        self.tabs.iter().map(|t| t.title())
    }

    pub fn curr_tab_index(&self) -> usize {
        self.curr_tab
    }

    pub fn tabs(&self) -> Tabs {
        Tabs::new(self.tabs_titles())
            .highlight_style(Tab::highlite_style())
            .select(self.curr_tab_index())
    }

    fn current_node_stat(&self) -> &Option<Stat> {
        &self.tabs[self.curr_tab_index()].current_node_stat
    }

    pub(crate) fn node_path_buf(&self) -> &String {
        &self.tabs[self.curr_tab_index()].node_path_buf
    }
    pub(crate) fn node_data_buf(&self) -> &String {
        &self.tabs[self.curr_tab_index()].node_data_buf
    }

    pub(crate) fn node_data(&self) -> &NodeData {
        &self.tabs[self.curr_tab_index()].node_data
    }

    pub(crate) fn input_buf(&self) -> &String {
        &self.tabs[self.curr_tab_index()].input_buf
    }
}
