use std::io;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use crate::domain::Challenge;

pub struct ChallengeListScreen {
    challenges: Vec<Challenge>,
    selected_index: usize,
}

impl ChallengeListScreen {
    pub fn new(challenges: Vec<Challenge>) -> Self {
        Self {
            challenges,
            selected_index: 0,
        }
    }

    /// Shows the challenge list and returns the selected challenge
    pub fn show(mut self) -> Result<Option<Challenge>> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run(&mut terminal);

        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        result
    }

    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<Option<Challenge>> {
        loop {
            terminal.draw(|f| self.render(f))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        return Ok(None);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if self.selected_index > 0 {
                            self.selected_index -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if self.selected_index < self.challenges.len() - 1 {
                            self.selected_index += 1;
                        }
                    }
                    KeyCode::Enter => {
                        return Ok(Some(self.challenges[self.selected_index].clone()));
                    }
                    _ => {}
                }
            }
        }
    }

    fn render(&self, f: &mut ratatui::Frame) {
        let size = f.area();

        // Create main layout
        let chunks = Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(size);

        // Render title
        self.render_title(f, chunks[0]);

        // Render challenge list
        self.render_list(f, chunks[1]);

        // Render help footer
        self.render_footer(f, chunks[2]);
    }

    fn render_title(&self, f: &mut ratatui::Frame, area: Rect) {
        let title = Paragraph::new("CHALLENGE SELECTION")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::BOTTOM));

        f.render_widget(title, area);
    }

    fn render_list(&self, f: &mut ratatui::Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .challenges
            .iter()
            .enumerate()
            .map(|(i, challenge)| {
                let difficulty_tag = challenge
                    .difficulty()
                    .map(|d| format!(" [{}]", d))
                    .unwrap_or_default();

                let number = format!("{:2}. ", i + 1);
                let title = challenge.title();
                let content = format!("{}{}{}", number, title, difficulty_tag);

                let style = if i == self.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if i == self.selected_index { "> " } else { "  " };
                ListItem::new(Line::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(content, style),
                ]))
            })
            .collect();

        let list = List::new(items).block(Block::default().borders(Borders::NONE));

        f.render_widget(list, area);
    }

    fn render_footer(&self, f: &mut ratatui::Frame, area: Rect) {
        let help_text = "↑/↓: Navigate  Enter: Select  q/Esc: Quit";
        let footer = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::TOP));

        f.render_widget(footer, area);
    }
}
