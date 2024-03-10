use ratatui::{
    layout::{Alignment, Layout, Direction, Constraint},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph, Padding},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ])
        .split(frame.size());

    frame.render_widget(
        Paragraph::new(
                "Space: New Picture\n".to_string() +
                "    q: Quit"
        )
        .block(
            Block::bordered()
                .title("Menu")
                .title_alignment(Alignment::Center)
                .padding(Padding::new(1, 0, 1, 0))
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow).bg(Color::Black)),
        layout[0],
    );

    let mut text = String::new();
    for row in app.image() {
        for tile in row {
            text.push(tile.glyph());
        }
        text.push('\n');
    }
    let ascii_grid = Paragraph::new(text);

    frame.render_widget(
        ascii_grid.block(
            Block::bordered()
            .title("Ari-Color v0.1")
            .title_alignment(Alignment::Center)
            .padding(Padding::new(0, 0, layout[1].height / 2 - 20 / 2, 0))
            .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow).bg(Color::Black))
        .centered(),
        layout[1],
    );
}
