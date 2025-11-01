use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::domain::Challenge;

/// Renders the challenge brief screen and waits for user to press Enter
pub struct ChallengeScreen;

impl ChallengeScreen {
    pub fn new() -> Self {
        Self
    }

    /// Displays the challenge and waits for Enter key
    /// Returns true if user pressed Enter, false if they quit
    pub fn show(&self, challenge: &Challenge) -> Result<bool> {
        let mut terminal = ratatui::init();
        terminal.clear()?;

        let result = loop {
            terminal.draw(|frame| self.render(frame, challenge))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => break Ok(true),
                    KeyCode::Esc | KeyCode::Char('q') => break Ok(false),
                    _ => {}
                }
            }
        };

        ratatui::restore();
        result
    }

    fn render(&self, frame: &mut Frame, challenge: &Challenge) {
        let area = frame.area();

        // Create vertical layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(10),   // Content
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // Title
        let title = Paragraph::new(format!("Challenge: {}", challenge.title()))
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, chunks[0]);

        // Content
        let content_text = vec![
            Line::from(""),
            Line::from(challenge.description()),
            Line::from(""),
            Line::from(vec![
                Span::styled("Starting: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("\"{}\"", challenge.starting_content())),
            ]),
            Line::from(vec![
                Span::styled("Target:   ", Style::default().fg(Color::Green)),
                Span::raw(format!("\"{}\"", challenge.target_content())),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("💡 Hint: "),
                Span::styled(challenge.hint(), Style::default().fg(Color::Magenta)),
            ]),
            Line::from(""),
            Line::from("Editor closes automatically when complete."),
        ];

        let content = Paragraph::new(content_text)
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(content, chunks[1]);

        // Footer
        let footer = Paragraph::new("[ Press Enter to Begin | Esc/q to Quit ]")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, chunks[2]);
    }
}

impl Default for ChallengeScreen {
    fn default() -> Self {
        Self::new()
    }
}
