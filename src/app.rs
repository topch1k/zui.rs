use core::fmt;
use ratatui::{
    style::{Style, Stylize},
    text::Line,
    widgets::{List, ListState},
};
use std::{net::IpAddr, vec};
use zookeeper_async::Stat;
#[derive(Default)]
pub struct App {
    pub state: AppState,
    pub connection: Option<Connection>,
    pub zk: Option<zookeeper_async::ZooKeeper>,
    pub connection_input: String,
    pub tab_data: Vec<String>,
    pub list_state: ListState,
    pub curr_resource: Option<String>, // selected node for current nest level
    pub prev_resources: Vec<String>,   // prev resources: e.g. /zookeeper/config
    pub current_node_stat: Option<Stat>,
}
#[derive(Debug)]
pub struct Connection {
    pub addr: IpAddr,
    pub port: u16,
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.addr, self.port)
    }
}

impl App {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection_input: connection.to_string(),
            connection: Some(connection),
            ..Default::default()
        }
    }
    pub fn connection_str(&self) -> String {
        match self.connection {
            Some(ref conn) => conn.to_string(),
            None => "".to_owned(),
        }
    }

    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.tab_data.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tab_data.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn selected_resource(&self) -> Option<String> {
        let selected_offset = self.list_state.selected();
        match selected_offset {
            Some(offset) => self.tab_data.get(offset).cloned().map(|p| format!("/{p}")),
            None => None,
        }
    }

    pub fn set_current_tab_path(&mut self, path: Option<String>) {
        self.curr_resource = path;
    }

    pub(crate) async fn store_node_stat(&mut self) {
        let full_path = self.full_resource_path();
        let stat = self
            .zk
            .as_ref()
            .unwrap()
            .exists(&full_path, false)
            .await
            .unwrap(); //TODO:
        self.current_node_stat = stat;
    }

    pub(crate) fn full_resource_path(&self) -> String {
        let prev = &self.prev_resources;
        let curr = &self.curr_resource;
        [prev.concat(), curr.clone().unwrap_or_default()].concat()
    }
    pub fn stat_list(&self) -> List {
        let Some(ref stat) = self.current_node_stat else {
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
#[derive(Debug, Default, PartialEq)]
pub enum AppState {
    #[default]
    EstablishingConnection,
    EditingConnection,
    Tab,
}
#[derive(Debug, PartialEq)]
pub enum TabState {}
