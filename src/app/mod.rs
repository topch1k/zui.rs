pub mod connection;
pub mod navigation;
pub mod render;
pub mod state;
pub mod zk_ops;
use std::mem;

use crate::{node_data::NodeData, tab::Tab};
use connection::Connection;
use ratatui::{
    style::{Style, Stylize},
    text::Line,
    widgets::{List, ListState, Tabs},
};
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

impl App {
    pub fn curr_tab(&self) -> &Tab {
        &self.tabs[self.curr_tab_index()]
    }
    pub fn curr_tab_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.curr_tab]
    }

    pub fn tab_list_state(&self) -> &ListState {
        &self.curr_tab().list_state
    }

    pub fn tab_data(&self) -> &Vec<String> {
        &self.curr_tab().tab_data
    }

    pub fn selected_resource(&self) -> Option<String> {
        let selected_offset = self.tab_list_state().selected();
        match selected_offset {
            Some(offset) => self
                .tab_data()
                .get(offset)
                .cloned()
                .map(|p| format!("/{p}")),
            None => None,
        }
    }

    pub(crate) fn tab_full_resource_path(&self) -> String {
        let prev = &self.curr_tab().prev_resources;
        let curr = &self.curr_tab().curr_resource;
        [
            prev.concat(),
            curr.clone().unwrap_or(BASE_RESOURCE.to_owned()),
        ]
        .concat()
    }

    pub(crate) fn is_full_resources_path_empty(&self) -> bool {
        self.curr_tab().prev_resources.is_empty() && self.curr_tab().curr_resource.is_none()
    }

    pub(crate) fn set_tab_message(&mut self, msg: String) {
        self.curr_tab_mut().message = msg;
    }

    pub(crate) fn append_tab_message(&mut self, msg: String) {
        self.curr_tab_mut().message.push_str(&msg);
    }

    pub(crate) fn clear_tab_message(&mut self) {
        self.curr_tab_mut().message.clear();
    }

    pub(crate) fn is_deletion_confirmed(&mut self) -> bool {
        let confirmation = mem::take(&mut self.curr_tab_mut().input_buf);
        confirmation.eq(CONFIRMATION_STRING)
    }
    pub fn stat_list(&self) -> List {
        let Some(ref stat) = self.curr_tab().current_node_stat else {
            return List::new(Vec::<Vec<Line>>::new());
        };

        let style = Style::new().on_gray();

        let lines = vec![
            Line::styled(format!("\t czxid : {}", stat.czxid), style),
            Line::styled(format!("\t mzxid : {}", stat.mzxid), style),
            Line::styled(format!("\t ctime : {}", stat.ctime), style),
            Line::styled(format!("\t mtime : {}", stat.mtime), style),
            Line::styled(format!("\t version : {}", stat.version), style),
            Line::styled(format!("\t cversion : {}", stat.cversion), style),
            Line::styled(format!("\t aversion : {}", stat.aversion), style),
            Line::styled(
                format!("\t ephemeral owner : {}", stat.ephemeral_owner),
                style,
            ),
            Line::styled(format!("\t data length : {}", stat.data_length), style),
            Line::styled(format!("\t num children : {}", stat.num_children), style),
            Line::styled(format!("\t pzxid : {}", stat.pzxid), style),
        ];
        List::from_iter(lines)
    }
}
