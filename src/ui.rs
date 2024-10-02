use std::vec;

use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{palette::tailwind, Color, Modifier, Style, Stylize},
    symbols,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::{app::App, app_legacy::AppState};

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
        let titles = app.tabs.iter().map(|t| t.title());
        let highlight_style = (Color::default(), tailwind::AMBER.c700);
        let selected_tab_index = app.curr_tab;

        let tabs = Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index);

        let [tabs_rect, work_rect, msg_rect] = Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![
                Constraint::Fill(1),
                Constraint::Percentage(85),
                Constraint::Fill(5),
            ],
        )
        .areas(frame.area());

        frame.render_widget(tabs, tabs_rect);

        let [nodes_list_rect, node_stat_rect] = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![Constraint::Fill(3), Constraint::Fill(1)],
        )
        .areas(work_rect);

        let msg_paragraph = Paragraph::new(app.message.as_str()).block(
            Block::default()
                .title("Message")
                .borders(Borders::ALL)
                .border_set(symbols::border::THICK)
                .on_gray()
                .title_alignment(Alignment::Left),
        );

        let info_block = Block::default()
            .title("Node Stat")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_alignment(Alignment::Center);

        let node_stat = &app.current_node_stat;
        match node_stat {
            Some(_) => {
                frame.render_widget(Clear, node_stat_rect);
                let stat_list = app.stat_list().block(info_block);
                frame.render_widget(stat_list, node_stat_rect);
            }
            None => {
                frame.render_widget(info_block, node_stat_rect);
            }
        }

        let nodes_block = Block::default()
            .title("Nodes")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_alignment(Alignment::Left)
            .title_bottom("(q)uit | ↑ to Up | ↓ to Down | Enter to dir Down | Esc to dir Up | (C)reate Node | (D)elete Node");

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
    }

    pub fn render_node_data_screen(frame: &mut Frame, app: &mut App) {
        let titles = app.tabs.iter().map(|t| t.title());
        let highlight_style = (Color::default(), tailwind::AMBER.c700);
        let selected_tab_index = app.curr_tab;

        let tabs = Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index);

        let [tabs_rect, work_rect, msg_rect] = Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![
                Constraint::Fill(1),
                Constraint::Percentage(85),
                Constraint::Fill(5),
            ],
        )
        .areas(frame.area());

        frame.render_widget(tabs, tabs_rect);

        let data_popup_rect = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .split(
            Layout::new(
                ratatui::layout::Direction::Vertical,
                vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ],
            )
            .split(work_rect)[1],
        )[1];

        let data_paragraph = Paragraph::new(app.node_data.to_string())
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Node Data")
                    .borders(Borders::ALL)
                    .border_set(symbols::border::THICK)
                    .on_dark_gray()
                    .title_alignment(Alignment::Center)
                    .title_bottom("ESC to cancel | (J)son | (S)tring | (R)aw | (E)dit"),
            );

        let [nodes_list_rect, node_stat_rect] = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![Constraint::Fill(3), Constraint::Fill(1)],
        )
        .areas(work_rect);

        let msg_paragraph = Paragraph::new(app.message.as_str()).block(
            Block::default()
                .title("Message")
                .borders(Borders::ALL)
                .border_set(symbols::border::THICK)
                .on_gray()
                .title_alignment(Alignment::Left),
        );

        let info_block = Block::default()
            .title("Node Stat")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_alignment(Alignment::Center);

        let node_stat = &app.current_node_stat;
        match node_stat {
            Some(_) => {
                frame.render_widget(Clear, node_stat_rect);
                let stat_list = app.stat_list().block(info_block);
                frame.render_widget(stat_list, node_stat_rect);
            }
            None => {
                frame.render_widget(info_block, node_stat_rect);
            }
        }

        let nodes_block = Block::default()
            .title("Nodes")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_alignment(Alignment::Left);

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
        frame.render_widget(data_paragraph, data_popup_rect);
    }

    pub fn render_edit_create_node_path_screen(frame: &mut Frame, app: &mut App) {
        let titles = app.tabs.iter().map(|t| t.title());
        let highlight_style = (Color::default(), tailwind::AMBER.c700);
        let selected_tab_index = app.curr_tab;

        let tabs = Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index);

        let [tabs_rect, work_rect, msg_rect] = Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![
                Constraint::Fill(1),
                Constraint::Percentage(85),
                Constraint::Fill(5),
            ],
        )
        .areas(frame.area());

        frame.render_widget(tabs, tabs_rect);

        let data_popup_rect = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .split(
            Layout::new(
                ratatui::layout::Direction::Vertical,
                vec![
                    Constraint::Fill(1),
                    Constraint::Fill(2),
                    Constraint::Fill(1),
                ],
            )
            .split(work_rect)[1],
        )[1];

        let [edit_path_rect, edit_data_rect] = Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![Constraint::Fill(1), Constraint::Fill(1)],
        )
        .areas(data_popup_rect);

        let edit_create_node_path_paragraph = Paragraph::new(app.node_path_buf.as_str())
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Edit Path")
                    .borders(Borders::ALL)
                    .border_set(symbols::border::THICK)
                    .on_blue()
                    .title_alignment(Alignment::Center),
            );

        let edit_create_node_data_paragraph = Paragraph::new(app.node_data_buf.as_str())
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Edit Data")
                    .borders(Borders::ALL)
                    .border_set(symbols::border::THICK)
                    .on_light_blue()
                    .title_alignment(Alignment::Center)
                    .title_bottom("ESC to cancel | Enter to Create | Tab to Change Window"),
            );

        let [nodes_list_rect, node_stat_rect] = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![Constraint::Fill(3), Constraint::Fill(1)],
        )
        .areas(work_rect);

        let msg_paragraph = Paragraph::new(app.message.as_str()).block(
            Block::default()
                .title("Message")
                .borders(Borders::ALL)
                .border_set(symbols::border::THICK)
                .on_gray()
                .title_alignment(Alignment::Left),
        );

        let info_block = Block::default()
            .title("Node Stat")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_alignment(Alignment::Center);

        let node_stat = &app.current_node_stat;
        match node_stat {
            Some(_) => {
                frame.render_widget(Clear, node_stat_rect);
                let stat_list = app.stat_list().block(info_block);
                frame.render_widget(stat_list, node_stat_rect);
            }
            None => {
                frame.render_widget(info_block, node_stat_rect);
            }
        }

        let nodes_block = Block::default()
            .title("Nodes")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_alignment(Alignment::Left);

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
        frame.render_widget(edit_create_node_path_paragraph, edit_path_rect);
        frame.render_widget(edit_create_node_data_paragraph, edit_data_rect);
    }
    pub fn render_edit_create_node_data_screen(frame: &mut Frame, app: &mut App) {
        let titles = app.tabs.iter().map(|t| t.title());
        let highlight_style = (Color::default(), tailwind::AMBER.c700);
        let selected_tab_index = app.curr_tab;

        let tabs = Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index);

        let [tabs_rect, work_rect, msg_rect] = Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![
                Constraint::Fill(1),
                Constraint::Percentage(85),
                Constraint::Fill(5),
            ],
        )
        .areas(frame.area());

        frame.render_widget(tabs, tabs_rect);

        let data_popup_rect = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .split(
            Layout::new(
                ratatui::layout::Direction::Vertical,
                vec![
                    Constraint::Fill(1),
                    Constraint::Fill(2),
                    Constraint::Fill(1),
                ],
            )
            .split(work_rect)[1],
        )[1];

        let [edit_path_rect, edit_data_rect] = Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![Constraint::Fill(1), Constraint::Fill(1)],
        )
        .areas(data_popup_rect);

        let edit_create_node_path_paragraph = Paragraph::new(app.node_path_buf.as_str())
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Edit Path")
                    .borders(Borders::ALL)
                    .border_set(symbols::border::THICK)
                    .on_light_blue()
                    .title_alignment(Alignment::Center),
            );

        let edit_create_node_data_paragraph = Paragraph::new(app.node_data_buf.as_str())
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Edit Data")
                    .borders(Borders::ALL)
                    .border_set(symbols::border::THICK)
                    .on_blue()
                    .title_alignment(Alignment::Center)
                    .title_bottom("ESC to cancel | Enter to Create | Tab to Change Window"),
            );

        let [nodes_list_rect, node_stat_rect] = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![Constraint::Fill(3), Constraint::Fill(1)],
        )
        .areas(work_rect);

        let msg_paragraph = Paragraph::new(app.message.as_str()).block(
            Block::default()
                .title("Message")
                .borders(Borders::ALL)
                .border_set(symbols::border::THICK)
                .on_gray()
                .title_alignment(Alignment::Left),
        );

        let info_block = Block::default()
            .title("Node Stat")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_alignment(Alignment::Center);

        let node_stat = &app.current_node_stat;
        match node_stat {
            Some(_) => {
                frame.render_widget(Clear, node_stat_rect);
                let stat_list = app.stat_list().block(info_block);
                frame.render_widget(stat_list, node_stat_rect);
            }
            None => {
                frame.render_widget(info_block, node_stat_rect);
            }
        }

        let nodes_block = Block::default()
            .title("Nodes")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_alignment(Alignment::Left);

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
        frame.render_widget(edit_create_node_path_paragraph, edit_path_rect);
        frame.render_widget(edit_create_node_data_paragraph, edit_data_rect);
    }
    pub fn render_edit_node_data_screen(frame: &mut Frame, app: &mut App) {
        let titles = app.tabs.iter().map(|t| t.title());
        let highlight_style = (Color::default(), tailwind::AMBER.c700);
        let selected_tab_index = app.curr_tab;

        let tabs = Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index);

        let [tabs_rect, work_rect, msg_rect] = Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![
                Constraint::Fill(1),
                Constraint::Percentage(85),
                Constraint::Fill(5),
            ],
        )
        .areas(frame.area());

        frame.render_widget(tabs, tabs_rect);

        let data_popup_rect = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .split(
            Layout::new(
                ratatui::layout::Direction::Vertical,
                vec![
                    Constraint::Fill(1),
                    Constraint::Fill(2),
                    Constraint::Fill(1),
                ],
            )
            .split(work_rect)[1],
        )[1];

        let [curr_data_rect, edited_data_rect] = Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![Constraint::Fill(1), Constraint::Fill(1)],
        )
        .areas(data_popup_rect);

        let edit_create_node_path_paragraph = Paragraph::new(app.node_data.to_string())
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Current Data")
                    .borders(Borders::ALL)
                    .border_set(symbols::border::THICK)
                    .on_dark_gray()
                    .title_alignment(Alignment::Center),
            );

        let edit_create_node_data_paragraph = Paragraph::new(app.node_data_buf.as_str())
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Edited Data")
                    .borders(Borders::ALL)
                    .border_set(symbols::border::THICK)
                    .on_dark_gray()
                    .title_alignment(Alignment::Center)
                    .title_bottom("ESC to cancel | Enter to Save"),
            );

        let [nodes_list_rect, node_stat_rect] = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![Constraint::Fill(3), Constraint::Fill(1)],
        )
        .areas(work_rect);

        let msg_paragraph = Paragraph::new(app.message.as_str()).block(
            Block::default()
                .title("Message")
                .borders(Borders::ALL)
                .border_set(symbols::border::THICK)
                .on_gray()
                .title_alignment(Alignment::Left),
        );

        let info_block = Block::default()
            .title("Node Stat")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_alignment(Alignment::Center);

        let node_stat = &app.current_node_stat;
        match node_stat {
            Some(_) => {
                frame.render_widget(Clear, node_stat_rect);
                let stat_list = app.stat_list().block(info_block);
                frame.render_widget(stat_list, node_stat_rect);
            }
            None => {
                frame.render_widget(info_block, node_stat_rect);
            }
        }

        let nodes_block = Block::default()
            .title("Nodes")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_alignment(Alignment::Left);

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
        let titles = app.tabs.iter().map(|t| t.title());
        let highlight_style = (Color::default(), tailwind::AMBER.c700);
        let selected_tab_index = app.curr_tab;

        let tabs = Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index);

        let [tabs_rect, work_rect, msg_rect] = Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![
                Constraint::Fill(1),
                Constraint::Percentage(85),
                Constraint::Fill(5),
            ],
        )
        .areas(frame.area());

        frame.render_widget(tabs, tabs_rect);

        let data_popup_rect = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .split(
            Layout::new(
                ratatui::layout::Direction::Vertical,
                vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ],
            )
            .split(work_rect)[1],
        )[1];

        let edit_create_node_path_paragraph = Paragraph::new(app.node_path_buf.as_str())
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Node to Delete")
                    .borders(Borders::ALL)
                    .border_set(symbols::border::THICK)
                    .on_dark_gray()
                    .title_alignment(Alignment::Center)
                    .title_bottom("Esc to cancel | Enter to Delete"),
            );

        let [nodes_list_rect, node_stat_rect] = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![Constraint::Fill(3), Constraint::Fill(1)],
        )
        .areas(work_rect);

        let msg_paragraph = Paragraph::new(app.message.as_str()).block(
            Block::default()
                .title("Message")
                .borders(Borders::ALL)
                .border_set(symbols::border::THICK)
                .on_gray()
                .title_alignment(Alignment::Left),
        );

        let info_block = Block::default()
            .title("Node Stat")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_alignment(Alignment::Center);

        let node_stat = &app.current_node_stat;
        match node_stat {
            Some(_) => {
                frame.render_widget(Clear, node_stat_rect);
                let stat_list = app.stat_list().block(info_block);
                frame.render_widget(stat_list, node_stat_rect);
            }
            None => {
                frame.render_widget(info_block, node_stat_rect);
            }
        }

        let nodes_block = Block::default()
            .title("Nodes")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_alignment(Alignment::Left);

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
        let titles = app.tabs.iter().map(|t| t.title());
        let highlight_style = (Color::default(), tailwind::AMBER.c700);
        let selected_tab_index = app.curr_tab;

        let tabs = Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index);

        let [tabs_rect, work_rect, msg_rect] = Layout::new(
            ratatui::layout::Direction::Vertical,
            vec![
                Constraint::Fill(1),
                Constraint::Percentage(85),
                Constraint::Fill(5),
            ],
        )
        .areas(frame.area());

        frame.render_widget(tabs, tabs_rect);

        let data_popup_rect = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .split(
            Layout::new(
                ratatui::layout::Direction::Vertical,
                vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ],
            )
            .split(work_rect)[1],
        )[1];

        let input_rect = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![
                Constraint::Fill(1),
                Constraint::Fill(5),
                Constraint::Fill(1),
            ],
        )
        .split(
            Layout::new(
                ratatui::layout::Direction::Vertical,
                vec![
                    Constraint::Fill(10),
                    Constraint::Fill(4),
                    Constraint::Fill(10),
                ],
            )
            .split(data_popup_rect)[1],
        )[1];

        let input_paragraph = Paragraph::new(app.input_buf.as_str())
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    // .title("Confirm Delete")
                    .borders(Borders::ALL)
                    .border_set(symbols::border::THICK)
                    .on_red()
                    .title_alignment(Alignment::Center), // .title_bottom("Esc to cancel | Enter to Delete"),
            );

        let node_to_delete = app.node_path_buf.as_str();
        let edit_create_node_path_paragraph = Paragraph::new(format!(
            "Type DELETE to confirm delete of {}",
            node_to_delete
        ))
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title("Confirm Delete")
                .borders(Borders::ALL)
                .border_set(symbols::border::THICK)
                .on_red()
                .title_alignment(Alignment::Center)
                .title_bottom("Esc to cancel | Enter to Delete"),
        );

        let [nodes_list_rect, node_stat_rect] = Layout::new(
            ratatui::layout::Direction::Horizontal,
            vec![Constraint::Fill(3), Constraint::Fill(1)],
        )
        .areas(work_rect);

        let msg_paragraph = Paragraph::new(app.message.as_str()).block(
            Block::default()
                .title("Message")
                .borders(Borders::ALL)
                .border_set(symbols::border::THICK)
                .on_gray()
                .title_alignment(Alignment::Left),
        );

        let info_block = Block::default()
            .title("Node Stat")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_alignment(Alignment::Center);

        let node_stat = &app.current_node_stat;
        match node_stat {
            Some(_) => {
                frame.render_widget(Clear, node_stat_rect);
                let stat_list = app.stat_list().block(info_block);
                frame.render_widget(stat_list, node_stat_rect);
            }
            None => {
                frame.render_widget(info_block, node_stat_rect);
            }
        }

        let nodes_block = Block::default()
            .title("Nodes")
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .on_gray()
            .title_alignment(Alignment::Left);

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
