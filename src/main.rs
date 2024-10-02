pub mod app;
pub mod app_legacy;
pub mod cli;
pub mod node_data;
pub mod tab;
pub mod ui;
pub mod zk;

use app::{state::AppState, App};
use cli::parse_cli;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::Backend, Terminal};
use ui::ui_legacy::AppUi;
use std::{io, time::Duration};
use zk::LoggingWatcher;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal, App::new(parse_cli().connection())).await;
    ratatui::restore();
    app_result
}

async fn run<B: Backend>(mut terminal: Terminal<B>, mut app: App) -> io::Result<()> {
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
                        let connection_string = app.connection_input.clone();
                        let zk = zookeeper_async::ZooKeeper::connect(
                            &connection_string,
                            Duration::from_secs(1),
                            LoggingWatcher,
                        )
                        .await
                        .unwrap(); //TODO:
                        app.tab_data = zk.get_children("/", false).await.unwrap(); //TODO:
                        app.zk = Some(zk);
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
                AppState::Tab => match key.code {
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.next();
                        app.curr_resource = app.selected_resource();
                        app.store_node_stat().await;
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.previous();
                        app.curr_resource = app.selected_resource();
                        app.store_node_stat().await;
                    }
                    KeyCode::Char('q') => break Result::Ok(()),
                    KeyCode::Enter => {
                        let curr = app.selected_resource();
                        app.store_children().await;
                        if let Some(curr) = curr {
                            app.prev_resources.push(curr);
                        }
                        app.list_state.select(None);
                    }
                    KeyCode::Esc => {
                        if app.is_full_resources_path_empty() {
                            continue;
                        }
                        app.curr_resource = app.prev_resources.pop();
                        app.store_children().await;
                        app.list_state.select(None);
                    }
                    KeyCode::Char('R') => {
                        app.state = AppState::ReadNodeData;
                        app.store_node_data().await;
                    }
                    KeyCode::Char('C') => {
                        app.state = AppState::EditCreateNodePath;
                        app.node_path_buf = app.full_resource_path();
                    }
                    KeyCode::Char('D') => {
                        app.state = AppState::DeleteNode;
                        app.node_path_buf = app.full_resource_path();
                    }
                    KeyCode::Right => {
                        app.next_tab();
                    }
                    KeyCode::Left => {
                        app.previous_tab();
                    }

                    _ => {}
                },
                AppState::ReadNodeData => match key.code {
                    KeyCode::Esc => {
                        app.state = AppState::Tab;
                    }
                    KeyCode::Char('S') => {
                        app.node_data = app.node_data.convert_to_string();
                    }
                    KeyCode::Char('J') => {
                        app.node_data = app.node_data.convert_to_json();
                    }
                    KeyCode::Char('R') => {
                        app.node_data = app.node_data.convert_to_raw();
                    }
                    KeyCode::Char('E') => {
                        app.node_data = app.node_data.convert_to_string();
                        app.node_data_buf = app.node_data.to_string();
                        app.state = AppState::EditNodeData;
                    }
                    _ => {}
                },
                AppState::EditCreateNodePath => match key.code {
                    KeyCode::Esc => {
                        app.state = AppState::Tab;
                    }
                    KeyCode::Enter => {
                        app.create_node().await;
                    }
                    KeyCode::Tab => {
                        app.state = AppState::EditCreateNodeData;
                    }
                    KeyCode::Char(value) => {
                        app.node_path_buf.push(value);
                    }
                    KeyCode::Backspace => {
                        app.node_path_buf.pop();
                    }

                    _ => {}
                },
                AppState::EditCreateNodeData => match key.code {
                    KeyCode::Esc => {
                        app.state = AppState::Tab;
                    }
                    KeyCode::Enter => {
                        app.create_node().await;
                    }
                    KeyCode::Tab => {
                        app.state = AppState::EditCreateNodePath;
                    }
                    KeyCode::Char(value) => {
                        app.node_data_buf.push(value);
                    }
                    KeyCode::Backspace => {
                        app.node_data_buf.pop();
                    }

                    _ => {}
                },
                AppState::EditNodeData => match key.code {
                    KeyCode::Esc => {
                        app.state = AppState::ReadNodeData;
                        app.store_node_data().await;
                    }
                    KeyCode::Enter => {
                        app.set_data().await;
                        app.state = AppState::ReadNodeData;
                        app.store_node_data().await;
                    }
                    KeyCode::Char(value) => {
                        app.node_data_buf.push(value);
                    }
                    KeyCode::Backspace => {
                        app.node_data_buf.pop();
                    }
                    _ => {}
                },
                AppState::DeleteNode => match key.code {
                    KeyCode::Esc => {
                        app.state = AppState::Tab;
                    }
                    KeyCode::Enter => {
                        app.state = AppState::ConfirmDelete;
                    }
                    KeyCode::Char(value) => {
                        app.node_path_buf.push(value);
                    }
                    KeyCode::Backspace => {
                        app.node_path_buf.pop();
                    }
                    _ => {}
                },
                AppState::ConfirmDelete => match key.code {
                    KeyCode::Esc => {
                        app.state = AppState::DeleteNode;
                    }
                    KeyCode::Enter => {
                        if app.is_deletion_confirmed() {
                            app.delete_node().await;
                            app.state = AppState::Tab;
                            app.curr_resource = None;
                            app.store_children().await;
                        } else {
                            app.set_message("Incorrect confirmation string".to_owned());
                        }
                    }
                    KeyCode::Char(value) => {
                        app.input_buf.push(value);
                    }
                    KeyCode::Backspace => {
                        app.input_buf.pop();
                    }
                    _ => {}
                },
                _ => todo!(),
            }
        }
    }
}
