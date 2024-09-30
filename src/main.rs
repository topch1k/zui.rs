pub mod app;
pub mod cli;
pub mod ui;
pub mod zk;

use app::{App, AppState};
use cli::parse_cli;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::Backend, Terminal};
use std::{io, time::Duration};
use ui::AppUi;
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
                app::AppState::EstablishingConnection => match key.code {
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
                app::AppState::EditingConnection if key.kind == KeyEventKind::Press => {
                    match key.code {
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
                    }
                }
                app::AppState::Tab => match key.code {
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
                    _ => {}
                },
                _ => todo!(),
            }
        }
    }
}
