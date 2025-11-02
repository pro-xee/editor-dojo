use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::domain::Challenge;

pub struct ChallengeMode {
    pub practice_mode: bool,
}

/// Renders the challenge brief screen and waits for user to press Enter
pub struct ChallengeScreen {
    practice_mode: bool,
    show_hints: bool,
}

impl ChallengeScreen {
    pub fn new() -> Self {
        Self {
            practice_mode: false,
            show_hints: false,
        }
    }

    /// Displays the challenge and waits for Enter key
    /// Returns Some(ChallengeMode) if user wants to start, None if they quit
    pub fn show(&mut self, challenge: &Challenge) -> Result<Option<ChallengeMode>> {
        let mut terminal = ratatui::init();
        terminal.clear()?;

        let result = loop {
            terminal.draw(|frame| self.render(frame, challenge))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        break Ok(Some(ChallengeMode {
                            practice_mode: self.practice_mode,
                        }));
                    }
                    KeyCode::Esc | KeyCode::Char('q') => break Ok(None),
                    KeyCode::Char('p') => {
                        self.practice_mode = !self.practice_mode;
                    }
                    KeyCode::Char('h') if challenge.has_progressive_hints() => {
                        self.show_hints = !self.show_hints;
                    }
                    _ => {}
                }
            }
        };

        ratatui::restore();
        result
    }

    fn render(&self, frame: &mut Frame, challenge: &Challenge) {
        let area = frame.area();

        if self.show_hints && challenge.has_progressive_hints() {
            self.render_hints_overlay(frame, area, challenge);
        } else {
            self.render_main_screen(frame, area, challenge);
        }
    }

    fn render_main_screen(&self, frame: &mut Frame, area: ratatui::layout::Rect, challenge: &Challenge) {
        // Create vertical layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(10),   // Content
                Constraint::Length(4), // Footer
            ])
            .split(area);

        // Title with mode indicator
        let mode_indicator = if self.practice_mode { " [PRACTICE MODE]" } else { " [CHALLENGE MODE]" };
        let title = Paragraph::new(format!("Challenge: {}{}", challenge.title(), mode_indicator))
            .style(Style::default().fg(if self.practice_mode { Color::Yellow } else { Color::Cyan }))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, chunks[0]);

        // Content
        let mut content_text = vec![
            Line::from(""),
            Line::from(challenge.description()),
            Line::from(""),
        ];

        // Mode explanation
        if self.practice_mode {
            content_text.push(Line::from(vec![
                Span::styled("📝 Practice Mode: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("This won't count toward your records."),
            ]));
            content_text.push(Line::from(""));
        } else {
            content_text.push(Line::from(vec![
                Span::styled("⚡ Challenge Mode: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw("Your time and keystrokes will be recorded."),
            ]));
            content_text.push(Line::from(""));
        }

        // Add tags if present
        if !challenge.tags().is_empty() {
            let tags_str = challenge.tags().join(", ");
            content_text.push(Line::from(vec![
                Span::styled("Tags: ", Style::default().fg(Color::Cyan)),
                Span::raw(tags_str),
            ]));
            content_text.push(Line::from(""));
        }

        content_text.extend(vec![
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
        ]);

        // Add optimal solution info if available
        if let (Some(solution), Some(keystrokes)) = (challenge.optimal_solution(), challenge.optimal_keystrokes()) {
            content_text.push(Line::from(""));
            content_text.push(Line::from(vec![
                Span::styled("⭐ Optimal: ", Style::default().fg(Color::Green)),
                Span::raw(format!("{} keystrokes: \"{}\"", keystrokes, solution)),
            ]));
        }

        content_text.push(Line::from(""));
        content_text.push(Line::from("Editor closes automatically when complete."));

        let content = Paragraph::new(content_text)
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(content, chunks[1]);

        // Footer with all options
        let mut footer_lines = vec![
            Line::from("p: Toggle Practice Mode  Enter: Begin  Esc/q: Quit"),
        ];
        if challenge.has_progressive_hints() {
            footer_lines.push(Line::from("h: View Progressive Hints"));
        }

        let footer = Paragraph::new(footer_lines)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, chunks[2]);
    }

    fn render_hints_overlay(&self, frame: &mut Frame, area: ratatui::layout::Rect, challenge: &Challenge) {
        // Create centered box for hints
        let hints_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(area)[1];

        let hints_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(hints_area)[1];

        let mut hint_lines = vec![
            Line::from(vec![
                Span::styled("💡 Progressive Hints", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
        ];

        for (i, hint) in challenge.progressive_hints().iter().enumerate() {
            hint_lines.push(Line::from(vec![
                Span::styled(format!("{}. ", i + 1), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(hint),
            ]));
            hint_lines.push(Line::from(""));
        }

        if let (Some(solution), Some(keystrokes)) = (challenge.optimal_solution(), challenge.optimal_keystrokes()) {
            hint_lines.push(Line::from(""));
            hint_lines.push(Line::from(vec![
                Span::styled("Optimal Solution: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]));
            hint_lines.push(Line::from(vec![
                Span::raw(format!("  {} keystrokes: \"{}\"", keystrokes, solution)),
            ]));
        }

        let hints_widget = Paragraph::new(hint_lines)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(" Progressive Hints ")
                .title_alignment(Alignment::Center));

        frame.render_widget(hints_widget, hints_area);

        // Footer
        let footer_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area)[1];

        let footer = Paragraph::new("Press 'h' to close hints")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, footer_area);
    }
}

impl Default for ChallengeScreen {
    fn default() -> Self {
        Self::new()
    }
}
