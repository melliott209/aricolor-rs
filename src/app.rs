use std::{error, char};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    state: AppState,
    image: [[Tile;40];20],
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
            image: [[Tile::default();40];20],
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(state: AppState) -> Self {
        App {
            running: true,
            state,
            image: [[Tile::default();40];20],
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    pub fn running(&self) -> bool { self.running }

    pub fn image(&self) -> [[Tile;40];20] { self.image }

    pub fn image_as_string(&self) -> String {
        let mut text = String::new();
        for row in self.image {
            for tile in row {
                text.push(tile.glyph());
            }
            text.push('\n');
        }
        text
    }

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

/// A single character-tile on the drawing surface. If hidden
/// it doesn't get drawn.

#[derive(Debug, Copy, PartialEq)]
pub struct Tile {
    glyph: char,
    hidden: bool,
}

impl Clone for Tile {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            glyph: '.',
            hidden: true,
        }
    }

}

impl Tile {
    pub fn glyph(&self) -> char {
        self.glyph
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
        assert_eq!(app.image, [[Tile{glyph: '.', hidden: true};40];20]);
    }

    #[test]
    fn expected_initial_image() {
        let app = App::default();
        assert_eq!(app.image_as_string(),
            "........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n\
             ........................................\n"
            );
    }
}
