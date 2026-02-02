//! UI rendering components
//!
//! Provides ratatui-based rendering for the terminal UI.

use crate::App;
use matw_ai::AIProvider;
use matw_core::{Content, Role};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// UI renderer
pub struct UI;

impl UI {
    /// Draw the complete UI
    pub fn draw<P: AIProvider>(f: &mut Frame, app: &App<P>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(f.area());

        Self::draw_messages(f, app, chunks[0]);
        Self::draw_input(f, app, chunks[1]);
    }

    /// Draw messages area
    fn draw_messages<P: AIProvider>(f: &mut Frame, app: &App<P>, area: Rect) {
        let mut lines = Vec::new();

        for msg in &app.messages {
            let (role_color, role_name) = match msg.role() {
                Role::User => (Color::Green, "User"),
                Role::Assistant => (Color::Blue, "Assistant"),
                Role::System => (Color::Yellow, "System"),
                Role::Tool => (Color::Cyan, "Tool"),
            };

            lines.push(Line::from(vec![
                Span::styled(
                    format!("[{}]", role_name),
                    Style::default().fg(role_color).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
            ]));

            // Handle content
            match msg.content() {
                Content::Text(text) => {
                    for line in text.lines() {
                        lines.push(Line::from(vec![Span::raw("  "), Span::raw(line)]));
                    }
                }
                Content::ToolUse { name, .. } => {
                    lines.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            format!("Using tool: {}", name),
                            Style::default().fg(Color::Cyan),
                        ),
                    ]));
                }
                Content::ToolResult { content, is_error, .. } => {
                    let color = if *is_error { Color::Red } else { Color::Gray };
                    for line in content.lines() {
                        lines.push(Line::from(vec![
                            Span::raw("  "),
                            Span::styled(line, Style::default().fg(color)),
                        ]));
                    }
                }
            }

            lines.push(Line::from(""));
        }

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Conversation")
            )
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    /// Draw input area
    fn draw_input<P: AIProvider>(f: &mut Frame, app: &App<P>, area: Rect) {
        let input = Paragraph::new(app.input.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Input | Status: {}", app.status))
            );

        f.render_widget(input, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_creation() {
        // UI is just a marker type, this test verifies it compiles
        let _ = UI;
    }
}
