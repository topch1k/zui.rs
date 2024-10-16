pub mod app;
pub mod cli;
pub mod errors;
pub mod node_data;
pub mod tab;
pub mod ui;
pub mod zk;

use app::{
    state::{AppState, TabState},
    App, BASE_RESOURCE,
};
use cli::parse_cli;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use errors::AppResult;
use ratatui::{prelude::Backend, Terminal};
use ui::ui_handle::AppUi;

#[tokio::main]
async fn main() -> AppResult<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal, App::new(parse_cli().connection())).await;
    ratatui::restore();
    app_result
}

async fn run<B: Backend>(mut terminal: Terminal<B>, mut app: App) -> AppResult<()> {
    loop {
        terminal.draw(|frame| {
            AppUi::ui(frame, &mut app);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.state {
                AppState::EstablishingConnection => match key.code {
                    KeyCode::Esc => break Result::Ok(()),
                    KeyCode::Enter => {
                        let zk = App::connect_default(&app.connection_input).await?;
                        app.zk = Some(zk);
                        app.store_curr_tab_children_by_path(BASE_RESOURCE).await;
                        app.state = AppState::Tab;
                    }
                    KeyCode::Char('e') => {
                        app.state = AppState::EditingConnection;
                    }
                    KeyCode::Char('q') => break Result::Ok(()),
                    _ => {}
                },
                AppState::EditingConnection if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Esc => app.state = AppState::EstablishingConnection,
                    KeyCode::Enter => {
                        app.state = AppState::EstablishingConnection;
                    }
                    KeyCode::Char(value) => {
                        app.connection_input.push(value);
                    }
                    KeyCode::Backspace => {
                        app.connection_input.pop();
                    }
                    _ => {}
                },
                AppState::Tab => match &app.curr_tab().state {
                    TabState::Tab => match key.code {
                        KeyCode::Char('j') | KeyCode::Down => {
                            app.next();
                            app.curr_tab_mut().curr_resource = app.selected_resource();
                            if app.curr_tab().toggle_stats_auto_load {
                                app.store_node_stat().await;
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            app.previous();
                            app.curr_tab_mut().curr_resource = app.selected_resource();
                            if app.curr_tab().toggle_stats_auto_load {
                                app.store_node_stat().await;
                            }
                        }
                        KeyCode::Char('q') => break Result::Ok(()),
                        KeyCode::Char('S') => {
                            let curr = app.curr_tab().toggle_stats_auto_load;
                            app.curr_tab_mut().toggle_stats_auto_load = !curr;
                            if !curr {
                                app.store_node_stat().await;
                            } else {
                                app.curr_tab_mut().current_node_stat = None;
                            }
                        }
                        KeyCode::Enter => {
                            let curr = app.selected_resource();
                            let children = app.get_children(&app.tab_full_resource_path()).await;
                            if let Some(children) = children {
                                if !children.is_empty() {
                                    app.store_children(children).await;
                                    if let Some(curr) = curr {
                                        app.curr_tab_mut().prev_resources.push(curr);
                                    }
                                    app.curr_tab_mut().list_state.select(Some(0));
                                } else {
                                    app.set_tab_message(
                                        "Node does not have children nodes".to_owned(),
                                    );
                                }
                            }
                        }
                        KeyCode::Esc => {
                            if app.is_full_resources_path_empty() {
                                continue;
                            }
                            app.curr_tab_mut().curr_resource =
                                app.curr_tab_mut().prev_resources.pop();
                            let children = app.get_children(&app.tab_full_resource_path()).await;
                            if let Some(children) = children {
                                if !children.is_empty() {
                                    app.store_children(children).await;
                                    app.curr_tab_mut().list_state.select(Some(0));
                                } else {
                                    app.set_tab_message(
                                        "Node does not have children nodes".to_owned(),
                                    );
                                }
                            }
                            // app.curr_tab_mut().list_state.select(Some(0));
                        }
                        KeyCode::Char('R') => {
                            app.curr_tab_mut().state = TabState::ReadNodeData;
                            app.store_node_data().await;
                        }
                        KeyCode::Char('C') => {
                            app.curr_tab_mut().state = TabState::EditCreateNodePath;
                            app.curr_tab_mut().node_path_buf = app.tab_full_resource_path();
                        }
                        KeyCode::Char('D') => {
                            app.curr_tab_mut().state = TabState::DeleteNode;
                            app.curr_tab_mut().node_path_buf = app.tab_full_resource_path();
                        }
                        KeyCode::Right => {
                            app.next_tab();
                            let children = app.get_children(&app.tab_full_resource_path()).await;
                            if let Some(children) = children {
                                if !children.is_empty() {
                                    app.store_children(children).await;
                                } else {
                                    app.set_tab_message(
                                        "Node does not have children nodes".to_owned(),
                                    );
                                }
                            }
                        }
                        KeyCode::Left => {
                            app.previous_tab();
                            let children = app.get_children(&app.tab_full_resource_path()).await;
                            if let Some(children) = children {
                                if !children.is_empty() {
                                    app.store_children(children).await;
                                } else {
                                    app.set_tab_message(
                                        "Node does not have children nodes".to_owned(),
                                    );
                                }
                            }
                        }

                        _ => {}
                    },
                    TabState::ReadNodeData => match key.code {
                        KeyCode::Esc => {
                            app.curr_tab_mut().state = TabState::Tab;
                        }
                        KeyCode::Char('S') => {
                            app.curr_tab_mut().node_data =
                                app.curr_tab().node_data.clone().convert_to_string();
                        }
                        KeyCode::Char('J') => {
                            app.curr_tab_mut().node_data =
                                app.curr_tab().node_data.clone().convert_to_json();
                        }
                        KeyCode::Char('R') => {
                            app.curr_tab_mut().node_data =
                                app.curr_tab().node_data.clone().convert_to_raw();
                        }
                        KeyCode::Char('E') => {
                            app.curr_tab_mut().node_data =
                                app.curr_tab().node_data.clone().convert_to_string();
                            app.curr_tab_mut().node_data_buf = app.curr_tab().node_data.to_string();
                            app.curr_tab_mut().state = TabState::EditNodeData;
                        }
                        _ => {}
                    },
                    TabState::EditCreateNodePath => match key.code {
                        KeyCode::Esc => {
                            app.curr_tab_mut().state = TabState::Tab;
                        }
                        KeyCode::Enter => {
                            app.create_node().await;
                        }
                        KeyCode::Tab => {
                            app.curr_tab_mut().state = TabState::EditCreateNodeData;
                        }
                        KeyCode::Char(value) => {
                            app.curr_tab_mut().node_path_buf.push(value);
                        }
                        KeyCode::Backspace => {
                            app.curr_tab_mut().node_path_buf.pop();
                        }
                        _ => {}
                    },
                    TabState::EditCreateNodeData => match key.code {
                        KeyCode::Esc => {
                            app.curr_tab_mut().state = TabState::Tab;
                        }
                        KeyCode::Enter => {
                            app.create_node().await;
                        }
                        KeyCode::Tab => {
                            app.curr_tab_mut().state = TabState::EditCreateNodePath;
                        }
                        KeyCode::Char(value) => {
                            app.curr_tab_mut().node_data_buf.push(value);
                        }
                        KeyCode::Backspace => {
                            app.curr_tab_mut().node_data_buf.pop();
                        }

                        _ => {}
                    },
                    TabState::EditNodeData => match key.code {
                        KeyCode::Esc => {
                            app.curr_tab_mut().state = TabState::ReadNodeData;
                            app.store_node_data().await;
                        }
                        KeyCode::Enter => {
                            app.set_data().await;
                            app.curr_tab_mut().state = TabState::ReadNodeData;
                            app.store_node_data().await;
                        }
                        KeyCode::Char(value) => {
                            app.curr_tab_mut().node_data_buf.push(value);
                        }
                        KeyCode::Backspace => {
                            app.curr_tab_mut().node_data_buf.pop();
                        }
                        _ => {}
                    },
                    TabState::DeleteNode => match key.code {
                        KeyCode::Esc => {
                            app.curr_tab_mut().state = TabState::Tab;
                        }
                        KeyCode::Enter => {
                            app.curr_tab_mut().state = TabState::ConfirmDelete;
                        }
                        KeyCode::Char(value) => {
                            app.curr_tab_mut().node_path_buf.push(value);
                        }
                        KeyCode::Backspace => {
                            app.curr_tab_mut().node_path_buf.pop();
                        }
                        _ => {}
                    },
                    TabState::ConfirmDelete => match key.code {
                        KeyCode::Esc => {
                            app.curr_tab_mut().state = TabState::DeleteNode;
                        }
                        KeyCode::Enter => {
                            if app.is_deletion_confirmed() {
                                app.delete_node().await;
                                app.curr_tab_mut().state = TabState::Tab;
                                app.curr_tab_mut().curr_resource = None;
                                let children =
                                    app.get_children(&app.tab_full_resource_path()).await;
                                if let Some(children) = children {
                                    if !children.is_empty() {
                                        app.store_children(children).await;
                                    } else {
                                        app.set_tab_message(
                                            "Node does not have children nodes".to_owned(),
                                        );
                                    }
                                }
                            } else {
                                app.set_tab_message("Incorrect confirmation string".to_owned());
                            }
                        }
                        KeyCode::Char(value) => {
                            app.curr_tab_mut().input_buf.push(value);
                        }
                        KeyCode::Backspace => {
                            app.curr_tab_mut().input_buf.pop();
                        }
                        _ => {}
                    },
                },
                _ => todo!(),
            }
        }
    }
}
