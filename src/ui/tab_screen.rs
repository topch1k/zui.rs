use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols,
    widgets::{Block, Borders},
    Frame,
};

use super::ui_legacy::AppUi;

impl AppUi {
    

    pub(crate) fn data_popup_rect(work_rect: Rect) -> Rect {
        AppUi::horizontal_equal_layout().split(AppUi::vertical_equal_layout().split(work_rect)[1])
            [1]
    }

    

    pub(crate) fn confirmation_input_rect(data_popup_rect: Rect) -> Rect {
        Layout::horizontal(vec![
            Constraint::Fill(1),
            Constraint::Fill(5),
            Constraint::Fill(1),
        ])
        .split(
            Layout::vertical(vec![
                Constraint::Fill(10),
                Constraint::Fill(4),
                Constraint::Fill(10),
            ])
            .split(data_popup_rect)[1],
        )[1]
    }

}
