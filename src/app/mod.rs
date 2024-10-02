pub mod connection;
pub mod navigation;
pub mod state;
pub mod zk_ops;
use connection::Connection;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Clear, List, ListItem, ListState, Paragraph, StatefulWidget, Tabs, Widget},
};
use state::AppState;
use zookeeper_async::Stat;

use crate::{node_data::NodeData, tab::Tab, ui::ui_legacy::AppUi};

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

    // -//- Moved to Tab struct
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

    pub fn curr_tab(&self) -> usize {
        self.curr_tab
    }

    pub fn tabs(&self) -> Tabs {
        Tabs::new(self.tabs_titles())
            .highlight_style(Tab::highlite_style())
            .select(self.curr_tab())
    }

    pub fn tab_data(&mut self) -> Vec<ListItem> {
        self.tab_data
            .iter()
            .map(|item| ListItem::new(item.clone()))
            .collect()
    }

    pub fn items_list(&self) -> List {
        let items = self
            .tab_data
            .iter()
            .map(|item| ListItem::new(item.as_str()))
            .collect::<Vec<ListItem>>();

        let list = List::new(items)
            .block(AppUi::nodes_block())
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");
        list
    }

    pub(crate) fn render_nodes_list(&mut self, area: Rect, buf: &mut Buffer) {
        let items = self
            .tab_data
            .iter()
            .map(|item| ListItem::new(item.as_str()))
            .collect::<Vec<ListItem>>();

        let list = List::new(items)
            .block(AppUi::nodes_block())
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        StatefulWidget::render(list, area, buf, &mut self.list_state);
    }

    pub(crate) fn render_message_block(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.message.to_string())
            .block(AppUi::message_block())
            .render(area, buf);
    }

    pub(crate) fn render_tabs(&mut self, area: Rect, buf: &mut Buffer) {
        self.tabs().render(area, buf);
    }

    fn current_node_stat(&self) -> &Option<Stat> {
        &self.current_node_stat
    }

    pub(crate) fn render_node_stat(&mut self, area: Rect, buf: &mut Buffer) {
        match self.current_node_stat() {
            Some(_) => {
                Clear.render(area, buf);
                let stat_list = self.stat_list().block(AppUi::info_block());
                Widget::render(stat_list, area, buf);
            }
            None => {
                Widget::render(AppUi::info_block(), area, buf);
            }
        }
    }
}
