use core::fmt;
use futures::TryFutureExt;
use ratatui::{
    style::{Style, Stylize},
    text::Line,
    widgets::{List, ListState},
};
use std::{net::IpAddr, vec};
use zookeeper_async::{Acl, Stat};

use crate::node_data::NodeData;

const BASE_RESOURCE: &str = "/";
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
    pub message: String,
    pub node_data: NodeData,
    pub create_node_path_buf: String,
    pub create_node_data_buf: String,
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
        let _ = self
            .zk
            .as_ref()
            .unwrap()
            .exists(&full_path, false)
            .and_then(|stat| async {
                self.current_node_stat = stat;
                Ok(())
            })
            .await;
    }

    pub(crate) fn full_resource_path(&self) -> String {
        let prev = &self.prev_resources;
        let curr = &self.curr_resource;
        [
            prev.concat(),
            curr.clone().unwrap_or(BASE_RESOURCE.to_owned()),
        ]
        .concat()
    }

    pub(crate) fn is_full_resources_path_empty(&self) -> bool {
        self.prev_resources.is_empty() && self.curr_resource.is_none()
    }
    pub(crate) async fn store_children(&mut self) {
        {
            self.clear_message();
            self.append_message(format!("Full path : {}\n", self.full_resource_path()));
            self.append_message(format!("Prev path : {:?}\n", self.prev_resources));
        }
        let Some(ref zk) = self.zk else {
            return;
        };

        let children = zk
            .get_children(&self.full_resource_path(), false)
            .await
            .ok();
        if let Some(ch) = children {
            self.tab_data = ch;
        }
    }

    pub(crate) fn set_message(&mut self, msg: String) {
        self.message = msg;
    }

    pub(crate) fn append_message(&mut self, msg: String) {
        self.message.push_str(&msg);
    }

    pub(crate) fn clear_message(&mut self) {
        self.message.clear();
    }

    pub(crate) async fn store_node_data(&mut self) {
        let Some(ref zk) = self.zk else {
            return;
        };

        let _ = zk
            .get_data(&self.full_resource_path(), false)
            .and_then(|(data, _)| async {
                self.node_data = NodeData::Raw(data);
                Ok(())
            })
            .await;
    }

    pub(crate) async fn create_node(&mut self) {
        let Some(ref zk) = self.zk else {
            "Failed to get zookeeper client".clone_into(&mut self.message);
            return;
        };

        let res = zk
            .create(
                &self.create_node_path_buf,
                self.create_node_data_buf.clone().into_bytes(),
                Acl::open_unsafe().clone(),
                zookeeper_async::CreateMode::Persistent,
            )
            .await;
        match res {
            Ok(created_path) => self.message = format!("Node {created_path} created successfully"),
            Err(e) => self.message = format!("Node creation failed : {e}"),
        }
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
    NodeData,
    EditCreateNodePath,
    EditCreateNodeData,
}
