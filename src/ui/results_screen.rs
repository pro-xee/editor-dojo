use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::domain::Solution;

/// Renders the results screen after challenge completion
pub struct ResultsScreen;

impl ResultsScreen {
    pub fn new() -> Self {
        Self
    }

    /// Displays the results and waits for any key press
    pub fn show(&self, solution: &Solution) -> Result<()> {
        let mut terminal = ratatui::init();
        terminal.clear()?;

        loop {
            terminal.draw(|frame| self.render(frame, solution))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(_) | KeyCode::Enter | KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        ratatui::restore();
        Ok(())
    }

    fn render(&self, frame: &mut Frame, solution: &Solution) {
        let area = frame.area();

        // Create vertical layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(5),    // Content
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // Title
        let title_text = if solution.is_completed() {
            "✓ CHALLENGE COMPLETE!"
        } else {
            "✗ CHALLENGE INCOMPLETE"
        };

        let title_color = if solution.is_completed() {
            Color::Green
        } else {
            Color::Red
        };

        let title = Paragraph::new(title_text)
            .style(Style::default().fg(title_color))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, chunks[0]);

        // Content
        let elapsed = solution.elapsed_seconds();
        let content_text = vec![
            Line::from(""),
            Line::from(format!("Time: {}:{}s", elapsed / 60, elapsed % 60)),
            Line::from(""),
        ];

        let content = Paragraph::new(content_text)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(content, chunks[1]);

        // Footer
        let footer = Paragraph::new("[ Press any key to exit ]")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, chunks[2]);
    }
}

impl Default for ResultsScreen {
    fn default() -> Self {
        Self::new()
    }
}
