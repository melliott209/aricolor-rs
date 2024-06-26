use rand::{thread_rng, Rng};
use std::{
    char, error,
    fs::File,
    io::{self, Read},
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    state: AppState,
    image: [[Tile; 40]; 20],
}

#[derive(Debug, Clone, Copy)]
pub enum AppState {
    Menu,
    Draw,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            state: AppState::Menu,
            image: [[Tile::default(); 40]; 20],
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(state: AppState) -> Self {
        App {
            running: true,
            state,
            image: [[Tile::default(); 40]; 20],
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn image(&self) -> [[Tile; 40]; 20] {
        self.image
    }

    pub fn state(&self) -> AppState {
        self.state
    }

    pub fn image_as_string(&self) -> String {
        let mut text = String::new();
        for row in self.image {
            for tile in row {
                if tile.hidden {
                    text.push('.')
                } else {
                    text.push(tile.glyph());
                }
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

    pub fn load_random_image(&mut self) {
        self.load_image("triforce.txt")
            .expect("Failed to load image");
    }

    pub fn reveal_part(&mut self) {
        let mut hidden = Vec::new();
        for (i, row) in self.image.iter().enumerate() {
            if row[0].hidden {
                hidden.push(i);
            }
        }
        if hidden.is_empty() {
            self.change_state();
            return;
        }
        let row = thread_rng().gen_range(0..hidden.len());
        self.image[hidden[row]]
            .iter_mut()
            .for_each(|t| t.hidden = false);
    }

    pub fn load_image(&mut self, filepath: &str) -> Result<(), io::Error> {
        let mut file = File::open(filepath)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        for (row, line) in contents.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                self.image[row][col].set_glyph(c);
                self.image[row][col].set_hidden(true);
            }
        }
        Ok(())
    }

    pub fn new_image(&mut self) {
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

    pub fn set_glyph(&mut self, c: char) {
        self.glyph = c;
    }

    pub fn set_hidden(&mut self, hid: bool) {
        self.hidden = hid;
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyEventState, KeyModifiers};

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
        handle_key_events(
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                kind: crossterm::event::KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
            &mut app,
        )
        .expect("Error sending 'q' key event to test.");
        assert!(!app.running());
    }

    #[test]
    fn app_starts_with_blank_image() {
        let app = App::default();
        assert_eq!(
            app.image,
            [[Tile {
                glyph: '.',
                hidden: true
            }; 40]; 20]
        );
    }

    #[test]
    fn expected_initial_image() {
        let app = App::default();
        assert_eq!(
            app.image_as_string(),
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

    #[test]
    fn load_image_correctly() {
        let mut app = App::default();
        app.load_image("triforce.txt")
            .expect("Error loading test image");
        assert_eq!(
            app.image_as_string(),
            "+======================================+\n\
             |                                      |\n\
             |                                      |\n\
             |                                      |\n\
             |                                      |\n\
             |                                      |\n\
             |                   /\\                 |\n\
             |                  /  \\                |\n\
             |                 /    \\               |\n\
             |                /------\\              |\n\
             |               / \\    / \\             |\n\
             |              /   \\  /   \\            |\n\
             |             /_____\\/_____\\           |\n\
             |                                      |\n\
             |                                      |\n\
             |                                      |\n\
             |                                      |\n\
             |                                      |\n\
             |                                      |\n\
             +======================================+\n"
        )
    }
}
