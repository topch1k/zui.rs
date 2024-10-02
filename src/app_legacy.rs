// pub mod connection;
// pub mod navigation;

use core::fmt;
use futures::TryFutureExt;
use ratatui::{
    style::{Style, Stylize},
    text::Line,
    widgets::{List, ListState},
};
use std::{cmp::min, mem, net::IpAddr, vec};
use zookeeper_async::{Acl, Stat};

use crate::{
    app::{App, BASE_RESOURCE, CONFIRMATION_STRING},
    node_data::NodeData,
    tab::Tab,
};

impl App {
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

    pub(crate) fn set_message(&mut self, msg: String) {
        self.message = msg;
    }

    pub(crate) fn append_message(&mut self, msg: String) {
        self.message.push_str(&msg);
    }

    pub(crate) fn clear_message(&mut self) {
        self.message.clear();
    }

    pub(crate) fn is_deletion_confirmed(&mut self) -> bool {
        let confirmation = mem::take(&mut self.input_buf);
        confirmation.eq(CONFIRMATION_STRING)
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
