pub mod app;
pub mod cli;
pub mod ui;

use std::io;

use app::{App, AppState};
use cli::parse_cli;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::Backend, Terminal};
use ui::AppUi;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal, App::new(parse_cli().connection()));
    ratatui::restore();
    app_result
}

fn run<B: Backend>(mut terminal: Terminal<B>, mut app: App) -> io::Result<()> {
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
                        todo!()
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
                _ => todo!(),
            }
        }
    }
}
