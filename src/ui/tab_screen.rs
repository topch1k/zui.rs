use ratatui::{
    layout::{Alignment, Constraint, Layout},
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
}
