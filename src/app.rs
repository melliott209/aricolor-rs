use std::{error, char};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    state: AppState,
    image: [[char;40];40],
}

#[derive(Debug)]
pub enum AppState {
    Menu,
    Draw,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            state: AppState::Menu,
            image: [[' ';40];40],
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(state: AppState) -> Self {
        App {
            running: true,
            state,
            image: [[' ';40];40],
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    pub fn running(&self) -> bool { self.running }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn change_state(&mut self) {
        self.state = match self.state {
            AppState::Menu => AppState::Draw,
            AppState::Draw => AppState::Menu,
        };
    }

    pub fn new_image() {
        todo!("Not yet implemented")
    }

    pub fn reveal() {
        todo!("Not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyEvent, KeyModifiers, KeyEventState, KeyCode};

    use crate::handler::handle_key_events;

    use super::*;

    #[test]
    fn initial_state_is_menu() {
        let app = App::default();
        assert!(matches!(app.state, AppState::Menu));
    }

    #[test]
    fn menu_state_to_draw_state() {
        let mut app = App::new(AppState::Menu);
        app.change_state();
        assert!(matches!(app.state, AppState::Draw))
    }

    #[test]
    fn draw_state_to_menu_state() {
        let mut app = App::new(AppState::Draw);
        app.change_state();
        assert!(matches!(app.state, AppState::Menu))
    }

    #[test]
    fn q_to_quit() {
        let mut app = App::new(AppState::Menu);
        handle_key_events(KeyEvent { 
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            kind: crossterm::event::KeyEventKind::Press,
            state: KeyEventState::NONE
        }, &mut app)
            .expect("Error sending 'q' key event to test.");
        assert!(!app.running());
    }

    #[test]
    fn app_starts_with_blank_image() {
        let app = App::default();
        assert_eq!(app.image, [[' ';40];40]);
    }
}
