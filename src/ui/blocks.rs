use ratatui::{
    layout::Alignment,
    style::Stylize,
    symbols,
    widgets::{Block, Borders},
};

use super::ui_legacy::AppUi;

impl AppUi {
    pub(crate) fn default_styled_block() -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
    }

    pub(crate) fn message_block() -> Block<'static> {
        AppUi::default_styled_block()
            .title("Message")
            .title_alignment(Alignment::Left)
    }

    pub(crate) fn info_block() -> Block<'static> {
        AppUi::default_styled_block()
            .title("Node Stat")
            .title_alignment(Alignment::Center)
    }

    pub(crate) fn nodes_block() -> Block<'static> {
        AppUi::default_styled_block()
            .title("Nodes")
            .title_alignment(Alignment::Left)
            .title_bottom("(q)uit | ↑ to Up | ↓ to Down | Enter to dir Down | Esc to dir Up | (C)reate Node | (D)elete Node")
    }

    pub(crate) fn node_data_block() -> Block<'static> {
        AppUi::default_styled_block()
            .title("Node Data")
            .on_dark_gray()
            .title_alignment(Alignment::Center)
            .title_bottom("ESC to cancel | (J)son | (S)tring | (R)aw | (E)dit")
    }

    pub(crate) fn edit_path_active_block() -> Block<'static> {
        AppUi::default_styled_block()
            .title("Edit Path")
            .on_blue()
            .title_alignment(Alignment::Center)
    }

    pub(crate) fn edit_path_non_active_block() -> Block<'static> {
        AppUi::default_styled_block()
            .title("Edit Path")
            .on_light_blue()
            .title_alignment(Alignment::Center)
    }

    pub(crate) fn edit_data_non_active_block() -> Block<'static> {
        AppUi::default_styled_block()
            .title("Edit Data")
            .on_light_blue()
            .title_alignment(Alignment::Center)
            .title_bottom("ESC to cancel | Enter to Create | Tab to Change Window")
    }

    pub(crate) fn edit_data_active_block() -> Block<'static> {
        AppUi::default_styled_block()
            .title("Edit Data")
            .on_blue()
            .title_alignment(Alignment::Center)
            .title_bottom("ESC to cancel | Enter to Create | Tab to Change Window")
    }

    pub(crate) fn current_data_block() -> Block<'static> {
        AppUi::default_styled_block()
            .title("Current Data")
            .on_dark_gray()
            .title_alignment(Alignment::Center)
    }

    pub(crate) fn edited_data_block() -> Block<'static> {
        AppUi::default_styled_block()
            .title("Edited Data")
            .on_dark_gray()
            .title_alignment(Alignment::Center)
            .title_bottom("ESC to cancel | Enter to Save")
    }

    pub(crate) fn delete_node_block() -> Block<'static> {
        AppUi::default_styled_block()
            .title("Node to Delete")
            .on_dark_gray()
            .title_alignment(Alignment::Center)
            .title_bottom("Esc to cancel | Enter to Delete")
    }

    pub(crate) fn confirm_delete_block() -> Block<'static> {
        AppUi::default_styled_block()
            .title("Confirm Delete")
            .on_red()
            .title_alignment(Alignment::Center)
            .title_bottom("Esc to cancel | Enter to Delete")
    }

    pub(crate) fn connection_editing_block() -> Block<'static> {
        AppUi::default_styled_block()
            .title("Connect")
            .title_bottom("ESC to cancel")
            .title_bottom("ENTER to save")
            .title_alignment(Alignment::Center)
    }
    pub(crate) fn connection_input_block() -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::PLAIN)
            .on_dark_gray()
    }

    pub(crate) fn connection_frame_block() -> Block<'static> {
        Block::default()
            .title("zui.rs")
            .borders(Borders::ALL)
            .border_set(symbols::border::ONE_EIGHTH_WIDE)
            .title_bottom("q to quit")
    }

    pub(crate) fn connection_popup_block() -> Block<'static> {
        Block::default()
            .title("Connect")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_bottom("ESC to cancel")
            .title_bottom("ENTER to connect")
            .title_bottom("e to edit")
            .title_alignment(Alignment::Center)
    }
}
