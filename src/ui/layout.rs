use ratatui::layout::{Constraint, Layout};

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
}
