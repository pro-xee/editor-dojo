use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::domain::{Achievement, Solution};

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
            terminal.draw(|frame| self.render(frame, solution, &[]))?;

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

    /// Displays results with achievement notifications
    pub fn show_with_achievements(&self, solution: &Solution, achievements: Vec<Achievement>) -> Result<()> {
        let mut terminal = ratatui::init();
        terminal.clear()?;

        loop {
            terminal.draw(|frame| self.render(frame, solution, &achievements))?;

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

    fn render(&self, frame: &mut Frame, solution: &Solution, achievements: &[Achievement]) {
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
        let mut content_lines = vec![
            Line::from(""),
            Line::from(format!("Time: {}:{:02}s", elapsed / 60, elapsed % 60)),
        ];

        // Add recording information if available
        if let Some(recording) = solution.recording() {
            content_lines.push(Line::from(format!("Keystrokes: {}", recording.keystroke_count())));
            content_lines.push(Line::from(""));
            content_lines.push(Line::from("Key sequence:"));

            // Display the key sequence in a formatted box
            let key_sequence_text = recording.key_sequence().format_for_display(80);
            content_lines.push(Line::from(format!("  {}", key_sequence_text)));
            content_lines.push(Line::from(""));

            // Show recording path (abbreviated for display)
            let path_display = Self::abbreviate_path(&recording.file_path_display());
            content_lines.push(Line::from(format!("Recording: {}", path_display)));
            content_lines.push(Line::from(format!("Replay: asciinema play {}", path_display))
                .style(Style::default().fg(Color::Cyan)));
        }

        // Add achievement notifications if any were unlocked
        if !achievements.is_empty() {
            content_lines.push(Line::from(""));
            content_lines.push(Line::from(""));
            content_lines.push(Line::from(vec![
                Span::styled("🎉 NEW ACHIEVEMENT", Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)),
                Span::styled(if achievements.len() > 1 { "S" } else { "" }, Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)),
                Span::styled(" UNLOCKED! 🎉", Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)),
            ]));
            content_lines.push(Line::from(""));

            for achievement in achievements {
                let achievement_line = Line::from(vec![
                    Span::raw("  "),
                    Span::raw(achievement.badge()),
                    Span::raw("  "),
                    Span::styled(achievement.name(), Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)),
                    Span::raw(" - "),
                    Span::styled(achievement.description(), Style::default().fg(Color::White)),
                ]);
                content_lines.push(achievement_line);
            }
        }

        content_lines.push(Line::from(""));

        let content = Paragraph::new(content_lines)
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

    /// Abbreviates a file path for display by replacing home directory with ~
    fn abbreviate_path(path: &str) -> String {
        if let Ok(home) = std::env::var("HOME") {
            if let Some(rest) = path.strip_prefix(&home) {
                return format!("~{}", rest);
            }
        }
        path.to_string()
    }
}

impl Default for ResultsScreen {
    fn default() -> Self {
        Self::new()
    }
}
