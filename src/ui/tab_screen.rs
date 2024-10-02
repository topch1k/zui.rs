use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols,
    widgets::{Block, Borders},
    Frame,
};

use super::ui_legacy::AppUi;

impl AppUi {
    pub(crate) fn tab_screen_layout() -> Layout {
        Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![
                Constraint::Fill(1),
                Constraint::Percentage(85),
                Constraint::Fill(5),
            ],
        )
    }

    pub(crate) fn work_space_layout() -> Layout {
        Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![Constraint::Fill(3), Constraint::Fill(1)],
        )
    }

    fn default_styled_block() -> Block<'static> {
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

    pub(crate) fn data_popup_rect(work_rect: Rect) -> Rect {
        AppUi::horizontal_equal_layout().split(AppUi::vertical_equal_layout().split(work_rect)[1])
            [1]
    }

    pub(crate) fn vertical_equal_layout() -> Layout {
        Layout::vertical(vec![
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
    }
    pub(crate) fn vertical_doubled_layout() -> Layout {
        Layout::vertical(vec![
            Constraint::Fill(1),
            Constraint::Fill(2),
            Constraint::Fill(1),
        ])
    }
    pub(crate) fn horizontal_equal_layout() -> Layout {
        Layout::horizontal(vec![
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
    }

    pub(crate) fn vertical_double_popup_layout() -> Layout {
        Layout::vertical(vec![Constraint::Fill(1), Constraint::Fill(1)])
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
        Block::default()
            .title("Edit Data")
            .on_light_blue()
            .title_alignment(Alignment::Center)
            .title_bottom("ESC to cancel | Enter to Create | Tab to Change Window")
    }

    pub(crate) fn edit_data_active_block() -> Block<'static> {
        Block::default()
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
}
