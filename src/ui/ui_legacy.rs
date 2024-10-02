use std::vec;

use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{palette::tailwind, Color, Modifier, Style, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, StatefulWidget, Tabs, Wrap},
    Frame,
};

use crate::{
    app::{state::AppState, App},
    tab::Tab,
};

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

    pub fn render_tab_screen(frame: &mut Frame, app: &mut App) {
        let [tabs_rect, work_rect, msg_rect] = AppUi::tab_screen_layout().areas(frame.area());
        let [nodes_list_rect, node_stat_rect] = AppUi::work_space_layout().areas(work_rect);

        app.render_node_stat(node_stat_rect, frame.buffer_mut());
        app.render_tabs(tabs_rect, frame.buffer_mut());
        app.render_nodes_list(nodes_list_rect, frame.buffer_mut());
        app.render_message_block(msg_rect, frame.buffer_mut());
    }

    pub fn render_node_data_screen(frame: &mut Frame, app: &mut App) {
        let [tabs_rect, work_rect, msg_rect] = AppUi::tab_screen_layout().areas(frame.area());
        let [nodes_list_rect, node_stat_rect] = AppUi::work_space_layout().areas(work_rect);
        let data_popup_rect = AppUi::data_popup_rect(work_rect);

        app.render_node_stat(node_stat_rect, frame.buffer_mut());
        app.render_tabs(tabs_rect, frame.buffer_mut());
        app.render_nodes_list(nodes_list_rect, frame.buffer_mut());
        app.render_message_block(msg_rect, frame.buffer_mut());
        app.render_node_data(data_popup_rect, frame.buffer_mut());
    }

    pub fn render_edit_create_node_path_screen(frame: &mut Frame, app: &mut App) {
        let [tabs_rect, work_rect, msg_rect] = AppUi::tab_screen_layout().areas(frame.area());

        let data_popup_rect = AppUi::horizontal_equal_layout()
            .split(AppUi::vertical_doubled_layout().split(work_rect)[1])[1];

        let [edit_path_rect, edit_data_rect] =
            AppUi::vertical_double_popup_layout().areas(data_popup_rect);

        let [nodes_list_rect, node_stat_rect] = AppUi::work_space_layout().areas(work_rect);

        app.render_node_stat(node_stat_rect, frame.buffer_mut());
        app.render_tabs(tabs_rect, frame.buffer_mut());
        app.render_nodes_list(nodes_list_rect, frame.buffer_mut());
        app.render_message_block(msg_rect, frame.buffer_mut());
        app.render_edit_path_active_block(edit_path_rect, frame.buffer_mut());
        app.render_edit_data_non_active_block(edit_data_rect, frame.buffer_mut());
    }
    pub fn render_edit_create_node_data_screen(frame: &mut Frame, app: &mut App) {
        let [tabs_rect, work_rect, msg_rect] = AppUi::tab_screen_layout().areas(frame.area());

        let data_popup_rect = AppUi::horizontal_equal_layout()
            .split(AppUi::vertical_doubled_layout().split(work_rect)[1])[1];

        let [edit_path_rect, edit_data_rect] =
            AppUi::vertical_double_popup_layout().areas(data_popup_rect);

        let [nodes_list_rect, node_stat_rect] = AppUi::work_space_layout().areas(work_rect);

        app.render_node_stat(node_stat_rect, frame.buffer_mut());
        app.render_tabs(tabs_rect, frame.buffer_mut());
        app.render_nodes_list(nodes_list_rect, frame.buffer_mut());
        app.render_message_block(msg_rect, frame.buffer_mut());
        app.render_edit_path_non_active_block(edit_path_rect, frame.buffer_mut());
        app.render_edit_data_active_block(edit_data_rect, frame.buffer_mut());
    }
    pub fn render_edit_node_data_screen(frame: &mut Frame, app: &mut App) {
        let [tabs_rect, work_rect, msg_rect] = AppUi::tab_screen_layout().areas(frame.area());

        let tabs = app.tabs();
        frame.render_widget(tabs, tabs_rect);

        let data_popup_rect = AppUi::horizontal_equal_layout()
            .split(AppUi::vertical_doubled_layout().split(work_rect)[1])[1];

        let [curr_data_rect, edited_data_rect] =
            AppUi::vertical_double_popup_layout().areas(data_popup_rect);

        let edit_create_node_path_paragraph = Paragraph::new(app.node_data.to_string())
            .wrap(Wrap { trim: true })
            .block(AppUi::current_data_block());

        let edit_create_node_data_paragraph = Paragraph::new(app.node_data_buf.as_str())
            .wrap(Wrap { trim: true })
            .block(AppUi::edited_data_block());

        let [nodes_list_rect, node_stat_rect] = AppUi::work_space_layout().areas(work_rect);

        let msg_paragraph = Paragraph::new(app.message.as_str()).block(AppUi::message_block());

        match &app.current_node_stat {
            Some(_) => {
                frame.render_widget(Clear, node_stat_rect);
                let stat_list = app.stat_list().block(AppUi::info_block());
                frame.render_widget(stat_list, node_stat_rect);
            }
            None => {
                frame.render_widget(AppUi::info_block(), node_stat_rect);
            }
        }
        let nodes_block = AppUi::nodes_block();

        let items = app
            .tab_data
            .iter()
            .map(|item| ListItem::new(item.as_str()))
            .collect::<Vec<ListItem>>();

        let list = List::new(items)
            .block(nodes_block)
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        frame.render_stateful_widget(list, nodes_list_rect, &mut app.list_state);
        frame.render_widget(msg_paragraph, msg_rect);
        frame.render_widget(edit_create_node_path_paragraph, curr_data_rect);
        frame.render_widget(edit_create_node_data_paragraph, edited_data_rect);
    }

    fn render_delete_node_screen(frame: &mut Frame, app: &mut App) {
        let [tabs_rect, work_rect, msg_rect] = AppUi::tab_screen_layout().areas(frame.area());

        let tabs = app.tabs();
        frame.render_widget(tabs, tabs_rect);

        let data_popup_rect = AppUi::data_popup_rect(work_rect);

        let edit_create_node_path_paragraph = Paragraph::new(app.node_path_buf.as_str())
            .wrap(Wrap { trim: true })
            .block(AppUi::delete_node_block());

        let [nodes_list_rect, node_stat_rect] = AppUi::work_space_layout().areas(work_rect);

        let msg_paragraph = Paragraph::new(app.message.as_str()).block(AppUi::message_block());

        match &app.current_node_stat {
            Some(_) => {
                frame.render_widget(Clear, node_stat_rect);
                let stat_list = app.stat_list().block(AppUi::info_block());
                frame.render_widget(stat_list, node_stat_rect);
            }
            None => {
                frame.render_widget(AppUi::info_block(), node_stat_rect);
            }
        }
        let nodes_block = AppUi::nodes_block();

        let items = app
            .tab_data
            .iter()
            .map(|item| ListItem::new(item.as_str()))
            .collect::<Vec<ListItem>>();

        let list = List::new(items)
            .block(nodes_block)
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        frame.render_stateful_widget(list, nodes_list_rect, &mut app.list_state);
        frame.render_widget(msg_paragraph, msg_rect);
        frame.render_widget(edit_create_node_path_paragraph, data_popup_rect);
    }

    fn render_confirm_delete_screen(frame: &mut Frame, app: &mut App) {
        let [tabs_rect, work_rect, msg_rect] = AppUi::tab_screen_layout().areas(frame.area());

        let tabs = app.tabs();
        frame.render_widget(tabs, tabs_rect);

        let data_popup_rect = AppUi::data_popup_rect(work_rect);

        let input_rect = AppUi::confirmation_input_rect(data_popup_rect);

        let input_paragraph = Paragraph::new(app.input_buf.as_str())
            .wrap(Wrap { trim: true })
            .block(AppUi::default_styled_block().on_red());

        let node_to_delete = app.node_path_buf.as_str();
        let edit_create_node_path_paragraph = Paragraph::new(format!(
            "Type DELETE to confirm delete of {}",
            node_to_delete
        ))
        .wrap(Wrap { trim: true })
        .block(AppUi::confirm_delete_block());

        let [nodes_list_rect, node_stat_rect] = AppUi::work_space_layout().areas(work_rect);

        let msg_paragraph = Paragraph::new(app.message.as_str()).block(AppUi::message_block());

        match &app.current_node_stat {
            Some(_) => {
                frame.render_widget(Clear, node_stat_rect);
                let stat_list = app.stat_list().block(AppUi::info_block());
                frame.render_widget(stat_list, node_stat_rect);
            }
            None => {
                frame.render_widget(AppUi::info_block(), node_stat_rect);
            }
        }
        let nodes_block = AppUi::nodes_block();

        let items = app
            .tab_data
            .iter()
            .map(|item| ListItem::new(item.as_str()))
            .collect::<Vec<ListItem>>();

        let list = List::new(items)
            .block(nodes_block)
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        frame.render_stateful_widget(list, nodes_list_rect, &mut app.list_state);
        frame.render_widget(msg_paragraph, msg_rect);
        frame.render_widget(edit_create_node_path_paragraph, data_popup_rect);
        frame.render_widget(input_paragraph, input_rect);
    }
}
