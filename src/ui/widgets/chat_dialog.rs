use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Frame,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// Chat step types.
#[derive(Debug, Clone)]
pub enum ChatStep {
    Question {
        prompt: String,
        subtitle: Option<String>,
        input_type: InputType,
    },
    Confirmation {
        prompt: String,
        details: Vec<(String, String)>,
    },
    Info {
        message: String,
    },
    Completed {
        message: String,
    },
}

/// Input types for chat steps.
#[derive(Debug, Clone)]
pub enum InputType {
    Text,
    TextWithSuffix(String),
    Choice(Vec<String>),
    YesNo,
    Money,
}

/// Chat message in history.
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub prompt: String,
    pub response: String,
    pub is_completed: bool,
}

/// Interactive chat dialog widget.
#[derive(Debug)]
pub struct ChatDialog {
    pub title: String,
    pub steps: Vec<ChatStep>,
    pub current_step: usize,
    pub history: Vec<ChatMessage>,
    pub input: String,
    pub cursor_position: usize,
    pub selected_option: usize,
    pub is_complete: bool,
}

impl ChatDialog {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            steps: Vec::new(),
            current_step: 0,
            history: Vec::new(),
            input: String::new(),
            cursor_position: 0,
            selected_option: 0,
            is_complete: false,
        }
    }

    pub fn add_step(mut self, step: ChatStep) -> Self {
        self.steps.push(step);
        self
    }

    pub fn current_step(&self) -> Option<&ChatStep> {
        self.steps.get(self.current_step)
    }

    pub fn push_char(&mut self, ch: char) {
        self.input.insert(self.cursor_position, ch);
        self.cursor_position += 1;
    }

    pub fn pop_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input.remove(self.cursor_position);
        }
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn next_option(&mut self) {
        if let Some(ChatStep::Question { input_type, .. }) = self.current_step() {
            let max = match input_type {
                InputType::Choice(options) => options.len().saturating_sub(1),
                InputType::YesNo => 1,
                _ => 0,
            };
            self.selected_option = (self.selected_option + 1).min(max);
        }
    }

    pub fn prev_option(&mut self) {
        self.selected_option = self.selected_option.saturating_sub(1);
    }

    pub fn confirm_step(&mut self) -> Option<String> {
        if let Some(step) = self.current_step().cloned() {
            let response = match &step {
                ChatStep::Question { prompt, input_type, .. } => {
                    let resp = match input_type {
                        InputType::Text | InputType::TextWithSuffix(_) | InputType::Money => {
                            self.input.clone()
                        }
                        InputType::Choice(options) => {
                            options.get(self.selected_option).cloned().unwrap_or_default()
                        }
                        InputType::YesNo => {
                            if self.selected_option == 0 {
                                "Ja".to_string()
                            } else {
                                "Nein".to_string()
                            }
                        }
                    };
                    self.history.push(ChatMessage {
                        prompt: prompt.clone(),
                        response: resp.clone(),
                        is_completed: true,
                    });
                    Some(resp)
                }
                ChatStep::Confirmation { prompt, .. } => {
                    self.history.push(ChatMessage {
                        prompt: prompt.clone(),
                        response: "✓".to_string(),
                        is_completed: true,
                    });
                    Some("confirmed".to_string())
                }
                ChatStep::Info { .. } | ChatStep::Completed { .. } => {
                    None
                }
            };

            self.current_step += 1;
            self.clear_input();
            self.selected_option = 0;

            if self.current_step >= self.steps.len() {
                self.is_complete = true;
            }

            return response;
        }
        None
    }

    pub fn go_back(&mut self) {
        if self.current_step > 0 {
            self.current_step -= 1;
            self.history.pop();
            self.clear_input();
            self.selected_option = 0;
        }
    }
}

/// Renders the chat dialog.
pub fn render_chat_dialog(frame: &mut Frame, area: Rect, dialog: &ChatDialog) {
    let block = Block::default()
        .title(format!(" {} ", dialog.title))
        .borders(Borders::ALL)
        .border_style(Theme::title());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(5),    // History
            Constraint::Length(6), // Current step
            Constraint::Length(1), // Help
        ])
        .split(inner);

    // Render history
    let mut history_lines: Vec<Line> = Vec::new();
    for msg in &dialog.history {
        history_lines.push(Line::from(vec![
            Span::styled("✓ ", Theme::success()),
            Span::styled(&msg.prompt, Theme::dim()),
        ]));
        history_lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(&msg.response, Theme::normal()),
        ]));
        history_lines.push(Line::from(""));
    }

    let history = Paragraph::new(history_lines);
    frame.render_widget(history, chunks[0]);

    // Render current step
    if let Some(step) = dialog.current_step() {
        render_chat_step(frame, chunks[1], step, dialog);
    } else if dialog.is_complete {
        let complete = Paragraph::new(Line::from(vec![
            Span::styled("✓ ", Theme::success()),
            Span::styled("Vorgang abgeschlossen", Theme::success()),
        ]));
        frame.render_widget(complete, chunks[1]);
    }

    // Render help
    let help = Paragraph::new(Line::from(vec![
        Span::styled("[Enter] Bestätigen | [↑↓] Auswählen | [ESC] Abbrechen", Theme::dim()),
    ]));
    frame.render_widget(help, chunks[2]);
}

fn render_chat_step(frame: &mut Frame, area: Rect, step: &ChatStep, dialog: &ChatDialog) {
    match step {
        ChatStep::Question { prompt, subtitle, input_type } => {
            let mut lines: Vec<Line> = Vec::new();

            lines.push(Line::from(vec![
                Span::styled("> ", Theme::primary_style()),
                Span::styled(prompt, Theme::normal()),
            ]));

            if let Some(sub) = subtitle {
                lines.push(Line::from(Span::styled(format!("  {}", sub), Theme::dim())));
            }

            lines.push(Line::from(""));

            match input_type {
                InputType::Text => {
                    lines.push(Line::from(vec![
                        Span::raw("  ["),
                        Span::styled(&dialog.input, Theme::input()),
                        Span::raw("]"),
                    ]));
                }
                InputType::TextWithSuffix(suffix) => {
                    lines.push(Line::from(vec![
                        Span::raw("  ["),
                        Span::styled(&dialog.input, Theme::input()),
                        Span::raw("]"),
                        Span::styled(suffix, Theme::dim()),
                    ]));
                }
                InputType::Choice(options) => {
                    let mut option_spans: Vec<Span> = vec![Span::raw("  ")];
                    for (i, opt) in options.iter().enumerate() {
                        let style = if i == dialog.selected_option {
                            Theme::button_selected()
                        } else {
                            Theme::button()
                        };
                        option_spans.push(Span::styled(format!("[{}]", opt), style));
                        option_spans.push(Span::raw(" "));
                    }
                    lines.push(Line::from(option_spans));
                }
                InputType::YesNo => {
                    let yes_style = if dialog.selected_option == 0 {
                        Theme::button_selected()
                    } else {
                        Theme::button()
                    };
                    let no_style = if dialog.selected_option == 1 {
                        Theme::button_selected()
                    } else {
                        Theme::button()
                    };
                    lines.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled("[Ja]", yes_style),
                        Span::raw(" "),
                        Span::styled("[Nein]", no_style),
                    ]));
                }
                InputType::Money => {
                    lines.push(Line::from(vec![
                        Span::raw("  ["),
                        Span::styled(&dialog.input, Theme::input()),
                        Span::raw("] €"),
                    ]));
                }
            }

            let paragraph = Paragraph::new(lines);
            frame.render_widget(paragraph, area);
        }
        ChatStep::Confirmation { prompt, details } => {
            let mut lines: Vec<Line> = Vec::new();

            lines.push(Line::from(vec![
                Span::styled("> ", Theme::primary_style()),
                Span::styled(prompt, Theme::normal()),
            ]));
            lines.push(Line::from(""));

            for (key, value) in details {
                lines.push(Line::from(vec![
                    Span::raw("  • "),
                    Span::styled(format!("{}: ", key), Theme::dim()),
                    Span::styled(value, Theme::normal()),
                ]));
            }

            let paragraph = Paragraph::new(lines);
            frame.render_widget(paragraph, area);
        }
        ChatStep::Info { message } => {
            let lines = vec![
                Line::from(vec![
                    Span::styled("ℹ ", Theme::info_style()),
                    Span::styled(message, Theme::normal()),
                ]),
            ];
            let paragraph = Paragraph::new(lines);
            frame.render_widget(paragraph, area);
        }
        ChatStep::Completed { message } => {
            let lines = vec![
                Line::from(vec![
                    Span::styled("✓ ", Theme::success()),
                    Span::styled(message, Theme::success()),
                ]),
            ];
            let paragraph = Paragraph::new(lines);
            frame.render_widget(paragraph, area);
        }
    }
}
