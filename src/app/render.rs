use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style, Stylize},
    widgets::{Clear, List, ListItem, Paragraph, StatefulWidget, Widget, Wrap},
};

use crate::ui::ui_legacy::AppUi;

use super::App;

impl App {
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

    pub(crate) fn render_node_data(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.node_data.to_string())
            .wrap(Wrap { trim: true })
            .block(AppUi::node_data_block())
            .render(area, buf);
    }

    pub(crate) fn render_edit_path_active_block(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.node_path_buf().as_str())
            .wrap(Wrap { trim: true })
            .block(AppUi::edit_path_active_block())
            .render(area, buf);
    }
    pub(crate) fn render_edit_path_non_active_block(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.node_path_buf().as_str())
            .wrap(Wrap { trim: true })
            .block(AppUi::edit_path_non_active_block())
            .render(area, buf);
    }

    pub(crate) fn render_edit_data_non_active_block(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.node_data_buf().as_str())
            .wrap(Wrap { trim: true })
            .block(AppUi::edit_data_non_active_block())
            .render(area, buf);
    }

    pub(crate) fn render_edit_data_active_block(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.node_data_buf().as_str())
            .wrap(Wrap { trim: true })
            .block(AppUi::edit_data_active_block())
            .render(area, buf);
    }

    pub(crate) fn render_current_node_data(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.node_data().to_string())
            .wrap(Wrap { trim: true })
            .block(AppUi::current_data_block())
            .render(area, buf);
    }

    pub(crate) fn render_edited_node_data(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.node_data_buf().to_string())
            .wrap(Wrap { trim: true })
            .block(AppUi::edited_data_block())
            .render(area, buf);
    }

    pub(crate) fn render_delete_node(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.node_path_buf().as_str())
            .wrap(Wrap { trim: true })
            .block(AppUi::delete_node_block())
            .render(area, buf);
    }

    pub(crate) fn render_confirm_delete_node(&mut self, area: Rect, buf: &mut Buffer) {
        let input_rect = AppUi::confirmation_input_rect(area);

        Paragraph::new(self.input_buf().as_str())
            .wrap(Wrap { trim: true })
            .block(AppUi::default_styled_block().on_red())
            .render(input_rect, buf);

        Paragraph::new(format!(
            "Type DELETE to confirm delete of {}",
            self.node_path_buf()
        ))
        .wrap(Wrap { trim: true })
        .block(AppUi::confirm_delete_block())
        .render(area, buf);
    }
}
