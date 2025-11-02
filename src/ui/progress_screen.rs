use crate::domain::{Achievement, AchievementId, MasteryTier, Progress};
use anyhow::Result;
use chrono::Utc;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};
use std::time::Duration;

pub struct ProgressScreen;

impl ProgressScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&self, progress: &Progress, total_challenges: usize) -> Result<()> {
        let mut terminal = ratatui::init();
        terminal.clear()?;

        loop {
            terminal.draw(|frame| self.render(frame, progress, total_challenges))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter | KeyCode::Char(' ') => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        ratatui::restore();
        Ok(())
    }

    fn render(&self, frame: &mut Frame, progress: &Progress, total_challenges: usize) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Length(1),  // Spacing
                Constraint::Length(3),  // Overall progress bar
                Constraint::Length(1),  // Spacing
                Constraint::Length(11), // Stats box (increased for more stats)
                Constraint::Length(1),  // Spacing
                Constraint::Min(8),     // Achievements
                Constraint::Length(3),  // Footer
            ])
            .split(area);

        self.render_title(frame, chunks[0]);
        self.render_progress_bar(frame, chunks[2], progress, total_challenges);
        self.render_stats(frame, chunks[4], progress, total_challenges);
        self.render_achievements(frame, chunks[6], progress);
        self.render_footer(frame, chunks[7]);
    }

    fn render_title(&self, frame: &mut Frame, area: Rect) {
        let title = Paragraph::new("YOUR PROGRESS")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::BOTTOM));

        frame.render_widget(title, area);
    }

    fn render_progress_bar(
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

        let ratio = if total_challenges > 0 {
            completed as f64 / total_challenges as f64
        } else {
            0.0
        };

        let label = format!(
            "Overall: {}/{} challenges complete ({}%)",
            completed, total_challenges, percentage
        );

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::NONE))
            .gauge_style(Style::default().fg(Color::Green).bg(Color::DarkGray))
            .label(label)
            .ratio(ratio);

        frame.render_widget(gauge, area);
    }

    fn render_stats(
        &self,
        frame: &mut Frame,
        area: Rect,
        progress: &Progress,
        _total_challenges: usize,
    ) {
        let total_time = Self::format_duration(progress.total_practice_time());
        let avg_time = progress
            .average_solve_time()
            .map(Self::format_solve_time)
            .unwrap_or_else(|| "N/A".to_string());
        let avg_keystrokes = progress
            .average_keystrokes()
            .map(|k| format!("{}", k))
            .unwrap_or_else(|| "N/A".to_string());

        let today = Utc::now().date_naive();
        let current_streak = progress.calculate_current_streak(today);
        let streak_text = if current_streak > 0 {
            format!("{} days 🔥", current_streak)
        } else {
            "0 days".to_string()
        };
        let longest_streak = format!("{} days", progress.longest_streak());
        let total_attempts = progress.total_attempts();

        // Calculate mastery tier counts
        let mut gold_count = 0;
        let mut silver_count = 0;
        let mut bronze_count = 0;

        for stats in progress.all_challenge_stats().values() {
            if let Some(tier) = stats.mastery_tier() {
                match tier {
                    MasteryTier::Gold => gold_count += 1,
                    MasteryTier::Silver => silver_count += 1,
                    MasteryTier::Bronze => bronze_count += 1,
                }
            }
        }

        let mastery_text = format!("{} 🥇  {} 🥈  {} 🥉", gold_count, silver_count, bronze_count);
        let achievement_count = progress.achievement_count();
        let total_achievements = AchievementId::all().len();

        let lines = vec![
            Line::from("Stats:").style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Line::from(vec![
                Span::raw("  Total practice time:    "),
                Span::styled(total_time, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("  Average solve time:     "),
                Span::styled(avg_time, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("  Average keystrokes:     "),
                Span::styled(avg_keystrokes, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("  Current streak:         "),
                Span::styled(streak_text, Style::default().fg(Color::Magenta)),
            ]),
            Line::from(vec![
                Span::raw("  Longest streak:         "),
                Span::styled(longest_streak, Style::default().fg(Color::Magenta)),
            ]),
            Line::from(vec![
                Span::raw("  Total attempts:         "),
                Span::styled(format!("{}", total_attempts), Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("  Mastery tiers:          "),
                Span::styled(mastery_text, Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("  Achievements:           "),
                Span::styled(
                    format!("{}/{}", achievement_count, total_achievements),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
        ];

        let stats = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        frame.render_widget(stats, area);
    }

    fn render_achievements(&self, frame: &mut Frame, area: Rect, progress: &Progress) {
        let title_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        };

        let list_area = Rect {
            x: area.x,
            y: area.y + 1,
            width: area.width,
            height: area.height - 1,
        };

        let title = Paragraph::new("Achievements:")
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_widget(title, title_area);

        let unlocked = progress.unlocked_achievements();

        if unlocked.is_empty() {
            let placeholder = Paragraph::new("  No achievements unlocked yet. Complete challenges to earn achievements!")
                .style(Style::default().fg(Color::DarkGray))
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::DarkGray)),
                );
            frame.render_widget(placeholder, list_area);
            return;
        }

        // Show most recent achievements (up to what fits)
        let items: Vec<ListItem> = unlocked
            .iter()
            .rev() // Most recent first
            .map(|unlocked_achievement| {
                let achievement = Achievement::get(unlocked_achievement.id());
                let text = format!(
                    "{}  {}  {}",
                    achievement.badge(),
                    achievement.name(),
                    achievement.description()
                );
                ListItem::new(text).style(Style::default().fg(Color::Green))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        frame.render_widget(list, list_area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let footer = Paragraph::new("[ Press any key to return ]")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::TOP));

        frame.render_widget(footer, area);
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
            format!("{}s", total_secs)
        }
    }

    fn format_solve_time(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        if total_secs >= 60 {
            let minutes = total_secs / 60;
            let seconds = total_secs % 60;
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", total_secs)
        }
    }
}

impl Default for ProgressScreen {
    fn default() -> Self {
        Self::new()
    }
}
