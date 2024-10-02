use crate::{
    app::{App, BASE_RESOURCE, CONFIRMATION_STRING},
    tab::Tab,
};
use ratatui::{
    style::{Style, Stylize},
    text::Line,
    widgets::{List, ListState},
};
use std::{mem, vec};

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

    // pub fn set_current_tab_path(&mut self, path: Option<String>) {
    //     self.curr_resource = path;
    // }

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
