use crate::domain::Progress;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Stylize,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuAction {
    StartTraining,
    ViewProgress,
    BrowseChallenges,
    Settings,
    Quit,
}

pub struct MainMenuScreen {
    selected_index: usize,
    options: Vec<(&'static str, MenuAction)>,
}

impl MainMenuScreen {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            options: vec![
                ("Start Training", MenuAction::StartTraining),
                ("View Progress", MenuAction::ViewProgress),
                ("Browse Challenges", MenuAction::BrowseChallenges),
                ("Settings", MenuAction::Settings),
                ("Quit", MenuAction::Quit),
            ],
        }
    }

    pub fn show(&mut self, progress: &Progress, total_challenges: usize) -> Result<MenuAction> {
        let mut terminal = ratatui::init();
        terminal.clear()?;

        let result = loop {
            terminal.draw(|frame| self.render(frame, progress, total_challenges))?;

            if let Event::Key(key) = event::read()? {
                match self.handle_key(key) {
                    Some(action) => break action,
                    None => continue,
                }
            }
        };

        ratatui::restore();
        Ok(result)
    }

    fn render(&self, frame: &mut Frame, progress: &Progress, total_challenges: usize) {
        let area = frame.area();

        // Create main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Length(1),  // Spacing
                Constraint::Length(6),  // Progress summary
                Constraint::Length(1),  // Spacing
                Constraint::Min(8),     // Menu options
                Constraint::Length(3),  // Controls
            ])
            .split(area);

        // Title
        self.render_title(frame, chunks[0]);

        // Progress summary
        self.render_progress_summary(frame, chunks[2], progress, total_challenges);

        // Menu options
        self.render_menu(frame, chunks[4]);

        // Controls
        self.render_controls(frame, chunks[5]);
    }

    fn render_title(&self, frame: &mut Frame, area: Rect) {
        let title = Paragraph::new(vec![
            Line::from("EDITOR DOJO").style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Line::from("Master Your Text Editor").style(Style::default().fg(Color::Gray)),
        ])
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));

        frame.render_widget(title, area);
    }

    fn render_progress_summary(
        &self,
        frame: &mut Frame,
        area: Rect,
        progress: &Progress,
        total_challenges: usize,
    ) {
        let completed = progress.total_completed();
        let percentage = if total_challenges > 0 {
            (completed as f64 / total_challenges as f64 * 100.0) as u32
        } else {
            0
        };

        let editor = progress
            .editor_preference()
            .unwrap_or("Not set")
            .to_string();

        let current_streak = progress.calculate_current_streak(chrono::Utc::now().date_naive());
        let streak_text = if current_streak > 0 {
            format!("{} days 🔥", current_streak)
        } else {
            "0 days".to_string()
        };

        let total_time = progress.total_practice_time();
        let time_text = Self::format_duration(total_time);

        let lines = vec![
            Line::from(vec![
                Span::raw("Editor: "),
                Span::styled(
                    editor,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("                    [Change]").fg(Color::DarkGray),
            ]),
            Line::from(vec![
                Span::raw("Progress: "),
                Span::styled(
                    format!("{}/{} challenges complete ({}%)", completed, total_challenges, percentage),
                    Style::default().fg(Color::Green),
                ),
            ]),
            Line::from(vec![
                Span::raw("Current streak: "),
                Span::styled(streak_text, Style::default().fg(Color::Magenta)),
            ]),
            Line::from(vec![
                Span::raw("Total practice time: "),
                Span::styled(time_text, Style::default().fg(Color::Blue)),
            ]),
        ];

        let summary = Paragraph::new(lines)
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::NONE)
                    .style(Style::default()),
            );

        frame.render_widget(summary, area);
    }

    fn render_menu(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .options
            .iter()
            .enumerate()
            .map(|(i, (label, _))| {
                let prefix = if i == self.selected_index {
                    "> "
                } else {
                    "  "
                };

                let style = if i == self.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                ListItem::new(format!("{}{}", prefix, label)).style(style)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        frame.render_widget(list, area);
    }

    fn render_controls(&self, frame: &mut Frame, area: Rect) {
        let controls = Paragraph::new("↑/↓: Navigate  Enter: Select  q: Quit")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::TOP));

        frame.render_widget(controls, area);
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<MenuAction> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Some(MenuAction::Quit),
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
                None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_index < self.options.len() - 1 {
                    self.selected_index += 1;
                }
                None
            }
            KeyCode::Enter => Some(self.options[self.selected_index].1),
            _ => None,
        }
    }

    fn format_duration(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;

        if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else if minutes > 0 {
            format!("{}m", minutes)
        } else {
            "0m".to_string()
        }
    }
}

impl Default for MainMenuScreen {
    fn default() -> Self {
        Self::new()
    }
}
