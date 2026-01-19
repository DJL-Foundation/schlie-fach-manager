use crate::ui::theme::Theme;
use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

/// Chat-style wizard message used for dialog workflows.
#[derive(Debug, Clone)]
pub struct WizardMessage {
    pub sender: MessageSender,
    pub content: MessageContent,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone)]
pub enum MessageSender {
    System,
    User,
}

#[derive(Debug, Clone)]
pub enum MessageContent {
    Question {
        text: String,
        options: Vec<WizardOption>,
    },
    Answer {
        text: String,
    },
    Info {
        text: String,
    },
}

#[derive(Debug, Clone)]
pub struct WizardOption {
    pub label: String,
    pub value: String,
    pub metadata: Option<String>,
}

/// Renders wizard dialogs and tracks the current selection.
#[derive(Debug, Clone, Default)]
pub struct WizardRenderer {
    pub messages: Vec<WizardMessage>,
    pub current_question_index: usize,
    pub selected_option_index: usize,
}

impl WizardRenderer {
    /// Render the wizard into the given area.
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let history_height = area.height.saturating_sub(10);
        let question_height = 10;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(history_height),
                Constraint::Length(question_height),
            ])
            .split(area);

        self.render_history(chunks[0], buf, theme);
        if let Some(current_msg) = self.messages.get(self.current_question_index) {
            self.render_question(current_msg, chunks[1], buf, theme);
        }
    }

    /// Handle navigation key presses within the wizard.
    pub fn handle_key(&mut self, key: KeyCode) -> WizardAction {
        match key {
            KeyCode::Up => {
                if self.selected_option_index > 0 {
                    self.selected_option_index -= 1;
                }
                WizardAction::None
            }
            KeyCode::Down => {
                self.selected_option_index = self.selected_option_index.saturating_add(1);
                self.clamp_selection();
                WizardAction::None
            }
            KeyCode::Enter => WizardAction::Confirm(self.selected_option_index),
            KeyCode::Esc => WizardAction::Cancel,
            _ => WizardAction::None,
        }
    }

    /// Renders the message history portion.
    fn render_history(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let mut lines = Vec::new();
        for (idx, msg) in self.messages.iter().enumerate() {
            if idx >= self.current_question_index {
                break;
            }
            let sender = match msg.sender {
                MessageSender::System => "System",
                MessageSender::User => "Benutzer",
            };
            let content = match &msg.content {
                MessageContent::Question { text, .. } => text,
                MessageContent::Answer { text } => text,
                MessageContent::Info { text } => text,
            };
            lines.push(Line::from(vec![
                Span::styled(sender.to_string(), Style::default().fg(theme.text_dim)),
                Span::raw(": "),
                Span::styled(content.clone(), Style::default().fg(theme.text)),
            ]));
        }

        Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Verlauf"))
            .render(area, buf);
    }

    /// Renders the current question and options.
    fn render_question(&self, msg: &WizardMessage, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let block = Block::default()
            .title("System")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.wizard_system));

        if let MessageContent::Question { text, options } = &msg.content {
            let mut items = Vec::new();
            for (idx, option) in options.iter().enumerate() {
                let prefix = if idx == self.selected_option_index {
                    "> "
                } else {
                    "  "
                };
                let meta = option
                    .metadata
                    .as_ref()
                    .map(|m| format!(" {}", m))
                    .unwrap_or_default();
                let style = if idx == self.selected_option_index {
                    Style::default().fg(theme.wizard_option_selected)
                } else {
                    Style::default().fg(theme.wizard_option_normal)
                };
                items.push(ListItem::new(Line::from(vec![Span::styled(
                    format!("{}{}{}", prefix, option.label, meta),
                    style,
                )])));
            }

            let list = List::new(items).block(block);
            list.render(area, buf);

            let question_area = Rect {
                x: area.x + 1,
                y: area.y + 1,
                width: area.width.saturating_sub(2),
                height: 2,
            };
            Paragraph::new(text.as_str())
                .style(Style::default().fg(theme.text))
                .render(question_area, buf);
        } else if let MessageContent::Info { text } = &msg.content {
            Paragraph::new(text.as_str()).block(block).render(area, buf);
        }
    }

    /// Clamps the selection index to the current question option count.
    fn clamp_selection(&mut self) {
        if let Some(WizardMessage {
            content: MessageContent::Question { options, .. },
            ..
        }) = self.messages.get(self.current_question_index)
        {
            if options.is_empty() {
                self.selected_option_index = 0;
            } else if self.selected_option_index >= options.len() {
                self.selected_option_index = options.len() - 1;
            }
        }
    }
}

/// Action returned from wizard input handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardAction {
    None,
    Confirm(usize),
    Cancel,
}
