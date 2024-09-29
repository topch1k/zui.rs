use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::Stylize,
    symbols,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, AppState};

pub struct AppUi {}

impl AppUi {
    pub fn default_layout() -> Layout {
        Layout::new(
            ratatui::layout::Direction::Horizontal,
            [
                ratatui::layout::Constraint::Fill(3),
                ratatui::layout::Constraint::Fill(1),
            ],
        )
    }

    pub fn ui(frame: &mut Frame, app: &mut App) {
        match app.state {
            AppState::EstablishingConnection => {
                AppUi::render_connection_screen(frame, app);
            }
            AppState::EditingConnection => {
                AppUi::render_connection_editing_screen(frame, app);
            }
            _ => {}
        }
    }

    pub fn render_connection_editing_screen(frame: &mut Frame, app: &mut App) {
        let frame_layout = Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .split(frame.area());

        let popup_layout = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .split(frame_layout[1]);

        let popup_block = Block::default()
            .title("Connect")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_bottom("ESC to cancel")
            .title_bottom("ENTER to save")
            .title_alignment(Alignment::Center);

        let popup_input_layout = Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![
                Constraint::Fill(10),
                Constraint::Fill(3),
                Constraint::Fill(10),
            ],
        )
        .split(popup_layout[1]);

        let input_layout = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![
                Constraint::Fill(1),
                Constraint::Fill(5),
                Constraint::Fill(1),
            ],
        )
        .split(popup_input_layout[1]);

        let popup_input_block = Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::PLAIN)
            .on_dark_gray();

        let connection_string =
            Paragraph::new(app.connection_input.as_str()).block(popup_input_block);

        let frame_block = Block::default()
            .title("zui")
            .borders(Borders::ALL)
            .border_set(symbols::border::ONE_EIGHTH_WIDE)
            .title_bottom("q to quit");

        frame.render_widget(frame_block, frame.area());
        frame.render_widget(popup_block, popup_layout[1]);
        frame.render_widget(connection_string, input_layout[1]);
    }
    pub fn render_connection_screen(frame: &mut Frame, app: &mut App) {
        let frame_layout = Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .split(frame.area());

        let popup_layout = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .split(frame_layout[1]);

        let popup_block = Block::default()
            .title("Connect")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_bottom("ESC to cancel")
            .title_bottom("ENTER to connect")
            .title_bottom("e to edit")
            .title_alignment(Alignment::Center);

        let popup_input_layout = Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![
                Constraint::Fill(10),
                Constraint::Fill(3),
                Constraint::Fill(10),
            ],
        )
        .split(popup_layout[1]);

        let input_layout = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![
                Constraint::Fill(1),
                Constraint::Fill(5),
                Constraint::Fill(1),
            ],
        )
        .split(popup_input_layout[1]);

        let popup_input_block = Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::PLAIN)
            .on_dark_gray();
        // app.connection_input = app.connection_str();
        let connection_string =
            Paragraph::new(app.connection_input.as_str()).block(popup_input_block);

        let frame_block = Block::default()
            .title("zui")
            .borders(Borders::ALL)
            .border_set(symbols::border::ONE_EIGHTH_WIDE)
            .title_bottom("q to quit");

        frame.render_widget(frame_block, frame.area());
        frame.render_widget(popup_block, popup_layout[1]);
        frame.render_widget(connection_string, input_layout[1]);
    }
}
