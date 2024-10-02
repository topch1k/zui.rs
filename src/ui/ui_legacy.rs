use std::vec;

use ratatui::{
    layout::{Constraint, Layout},
    widgets::Paragraph,
    Frame,
};

use crate::app::{state::AppState, App};

pub struct AppUi {}

impl AppUi {
    pub fn ui(frame: &mut Frame, app: &mut App) {
        match app.state {
            AppState::EstablishingConnection => {
                AppUi::render_connection_screen(frame, app);
            }
            AppState::EditingConnection => {
                AppUi::render_connection_editing_screen(frame, app);
            }
            AppState::Tab => {
                AppUi::render_tab_screen(frame, app);
            }
            AppState::ReadNodeData => {
                AppUi::render_node_data_screen(frame, app);
            }
            AppState::EditCreateNodePath => {
                AppUi::render_edit_create_node_path_screen(frame, app);
            }
            AppState::EditCreateNodeData => {
                AppUi::render_edit_create_node_data_screen(frame, app);
            }
            AppState::EditNodeData => {
                AppUi::render_edit_node_data_screen(frame, app);
            }
            AppState::DeleteNode => {
                AppUi::render_delete_node_screen(frame, app);
            }
            AppState::ConfirmDelete => {
                AppUi::render_confirm_delete_screen(frame, app);
            }
            _ => {}
        }
    }

    pub fn render_connection_editing_screen(frame: &mut Frame, app: &mut App) {
        let frame_layout = AppUi::vertical_equal_layout().split(frame.area());

        let popup_layout = AppUi::horizontal_equal_layout().split(frame_layout[1]);

        let popup_block = AppUi::connection_editing_block();

        let input_layout = Layout::horizontal(vec![
            Constraint::Fill(1),
            Constraint::Fill(5),
            Constraint::Fill(1),
        ])
        .split(
            Layout::vertical(vec![
                Constraint::Fill(10),
                Constraint::Fill(3),
                Constraint::Fill(10),
            ])
            .split(popup_layout[1])[1],
        );

        let popup_input_block = AppUi::connection_input_block();

        let connection_string =
            Paragraph::new(app.connection_input.as_str()).block(popup_input_block);

        let frame_block = AppUi::connection_frame_block();

        frame.render_widget(frame_block, frame.area());
        frame.render_widget(popup_block, popup_layout[1]);
        frame.render_widget(connection_string, input_layout[1]);
    }
    pub fn render_connection_screen(frame: &mut Frame, app: &mut App) {
        let frame_layout = AppUi::vertical_equal_layout().split(frame.area());

        let popup_layout = AppUi::horizontal_equal_layout().split(frame_layout[1]);

        let popup_block = AppUi::connection_popup_block();

        let input_layout = Layout::horizontal(vec![
            Constraint::Fill(1),
            Constraint::Fill(5),
            Constraint::Fill(1),
        ])
        .split(
            Layout::vertical(vec![
                Constraint::Fill(10),
                Constraint::Fill(3),
                Constraint::Fill(10),
            ])
            .split(popup_layout[1])[1],
        );

        let popup_input_block = AppUi::connection_input_block();

        let connection_string =
            Paragraph::new(app.connection_input.as_str()).block(popup_input_block);

        let frame_block = AppUi::connection_frame_block();

        frame.render_widget(frame_block, frame.area());
        frame.render_widget(popup_block, popup_layout[1]);
        frame.render_widget(connection_string, input_layout[1]);
    }

    pub fn render_tab_screen(frame: &mut Frame, app: &mut App) {
        let [tabs_rect, work_rect, msg_rect] = AppUi::tab_screen_layout().areas(frame.area());
        let [nodes_list_rect, node_stat_rect] = AppUi::work_space_layout().areas(work_rect);

        app.render_node_stat(node_stat_rect, frame.buffer_mut());
        app.render_tabs(tabs_rect, frame.buffer_mut());
        app.render_nodes_list(nodes_list_rect, frame.buffer_mut());
        app.render_message_block(msg_rect, frame.buffer_mut());
    }

    pub fn render_node_data_screen(frame: &mut Frame, app: &mut App) {
        AppUi::render_tab_screen(frame, app);
        let work_rect = AppUi::tab_screen_layout().split(frame.area())[1];
        let data_popup_rect = AppUi::data_popup_rect(work_rect);

        app.render_node_data(data_popup_rect, frame.buffer_mut());
    }

    pub fn render_edit_create_node_path_screen(frame: &mut Frame, app: &mut App) {
        AppUi::render_tab_screen(frame, app);
        let work_rect = AppUi::tab_screen_layout().split(frame.area())[1];

        let data_popup_rect = AppUi::horizontal_equal_layout()
            .split(AppUi::vertical_doubled_layout().split(work_rect)[1])[1];

        let [edit_path_rect, edit_data_rect] =
            AppUi::vertical_double_popup_layout().areas(data_popup_rect);

        app.render_edit_path_active_block(edit_path_rect, frame.buffer_mut());
        app.render_edit_data_non_active_block(edit_data_rect, frame.buffer_mut());
    }
    pub fn render_edit_create_node_data_screen(frame: &mut Frame, app: &mut App) {
        AppUi::render_tab_screen(frame, app);
        let work_rect = AppUi::tab_screen_layout().split(frame.area())[1];

        let data_popup_rect = AppUi::horizontal_equal_layout()
            .split(AppUi::vertical_doubled_layout().split(work_rect)[1])[1];

        let [edit_path_rect, edit_data_rect] =
            AppUi::vertical_double_popup_layout().areas(data_popup_rect);
        app.render_edit_path_non_active_block(edit_path_rect, frame.buffer_mut());
        app.render_edit_data_active_block(edit_data_rect, frame.buffer_mut());
    }
    pub fn render_edit_node_data_screen(frame: &mut Frame, app: &mut App) {
        AppUi::render_tab_screen(frame, app);
        let work_rect = AppUi::tab_screen_layout().split(frame.area())[1];

        let data_popup_rect = AppUi::horizontal_equal_layout()
            .split(AppUi::vertical_doubled_layout().split(work_rect)[1])[1];

        let [curr_data_rect, edited_data_rect] =
            AppUi::vertical_double_popup_layout().areas(data_popup_rect);

        app.render_current_node_data(curr_data_rect, frame.buffer_mut());
        app.render_edited_node_data(edited_data_rect, frame.buffer_mut());
    }

    fn render_delete_node_screen(frame: &mut Frame, app: &mut App) {
        AppUi::render_tab_screen(frame, app);
        let work_rect = AppUi::tab_screen_layout().split(frame.area())[1];
        let data_popup_rect = AppUi::data_popup_rect(work_rect);
        app.render_delete_node(data_popup_rect, frame.buffer_mut());
    }

    fn render_confirm_delete_screen(frame: &mut Frame, app: &mut App) {
        AppUi::render_tab_screen(frame, app);
        let work_rect = AppUi::tab_screen_layout().split(frame.area())[1];
        let data_popup_rect = AppUi::data_popup_rect(work_rect);
        app.render_confirm_delete_node(data_popup_rect, frame.buffer_mut())
    }
}
