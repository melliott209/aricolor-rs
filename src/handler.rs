use crate::app::{App, AppResult, AppState};
use crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.state() {
        AppState::Menu => match key_event.code {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Char(' ') => {
                app.load_random_image();
                app.change_state();
            }
            _ => {}
        },
        AppState::Draw => match key_event.code {
            KeyCode::Char('q') => app.quit(),
            _ => app.reveal_part(),
        },
    };
    Ok(())
}
