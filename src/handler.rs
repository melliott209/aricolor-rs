use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if let KeyCode::Char('q') = key_event.code {
        // Exit application on `q`
        app.quit();
    }
    if let KeyCode::Char(' ') = key_event.code {
        app.load_random_image();
    }
    Ok(())
}
