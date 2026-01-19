use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

/// Wizard renderer for dialog-style interactions
/// Based on v2.1 spec section 7.3
#[derive(Debug)]
pub struct WizardRenderer {
    messages: Vec<WizardMessage>,
    current_question_index: usize,
    selected_option_index: usize,
}

/// A message in the wizard conversation
#[derive(Debug, Clone)]
pub struct WizardMessage {
    pub sender: MessageSender,
    pub content: MessageContent,
    pub timestamp: Option<String>,
}

/// Sender of a message
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageSender {
    System,
    User,
}

/// Content of a message
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

/// An option in a wizard question
#[derive(Debug, Clone)]
pub struct WizardOption {
    pub label: String,
    pub value: String,
    pub metadata: Option<String>,
}

/// Action result from wizard interaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WizardAction {
    None,
    NextQuestion,
    Complete(Vec<String>),
    Cancel,
}

impl WizardRenderer {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            current_question_index: 0,
            selected_option_index: 0,
        }
    }
    
    /// Add a new message to the wizard
    pub fn add_message(&mut self, message: WizardMessage) {
        self.messages.push(message);
    }
    
    /// Set the current question
    pub fn set_current_question(&mut self, message: WizardMessage) {
        self.current_question_index = self.messages.len();
        self.messages.push(message);
        self.selected_option_index = 0;
    }
    
    /// Get the currently selected option value
    pub fn selected_option_value(&self) -> Option<String> {
        if let Some(msg) = self.messages.get(self.current_question_index) {
            if let MessageContent::Question { options, .. } = &msg.content {
                return options.get(self.selected_option_index)
                    .map(|opt| opt.value.clone());
            }
        }
        None
    }
    
    /// Move to the next option
    pub fn select_next_option(&mut self) {
        if let Some(msg) = self.messages.get(self.current_question_index) {
            if let MessageContent::Question { options, .. } = &msg.content {
                if !options.is_empty() {
                    self.selected_option_index = (self.selected_option_index + 1) % options.len();
                }
            }
        }
    }
    
    /// Move to the previous option
    pub fn select_previous_option(&mut self) {
        if let Some(msg) = self.messages.get(self.current_question_index) {
            if let MessageContent::Question { options, .. } = &msg.content {
                if !options.is_empty() {
                    if self.selected_option_index == 0 {
                        self.selected_option_index = options.len() - 1;
                    } else {
                        self.selected_option_index -= 1;
                    }
                }
            }
        }
    }
    
    /// Confirm the current selection
    pub fn confirm_selection(&mut self) -> WizardAction {
        if let Some(value) = self.selected_option_value() {
            // Add user's answer to messages
            self.add_message(WizardMessage {
                sender: MessageSender::User,
                content: MessageContent::Answer { text: value.clone() },
                timestamp: None,
            });
            WizardAction::NextQuestion
        } else {
            WizardAction::None
        }
    }
    
    /// Render the wizard
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        // Calculate heights
        let history_height = area.height.saturating_sub(12).max(5);
        let question_height = area.height.saturating_sub(history_height);
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(history_height),
                Constraint::Length(question_height),
            ])
            .split(area);
        
        // Render message history
        self.render_history(chunks[0], buf, theme);
        
        // Render current question
        if let Some(current_msg) = self.messages.get(self.current_question_index) {
            self.render_question(current_msg, chunks[1], buf, theme);
        }
    }
    
    fn render_history(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        // Only show messages before the current question
        let history_messages: Vec<&WizardMessage> = self.messages
            .iter()
            .take(self.current_question_index)
            .collect();
        
        let items: Vec<ListItem> = history_messages
            .iter()
            .map(|msg| {
                let color = match msg.sender {
                    MessageSender::System => theme.wizard_system,
                    MessageSender::User => theme.wizard_user,
                };
                
                let text = match &msg.content {
                    MessageContent::Question { text, .. } => format!("System: {}", text),
                    MessageContent::Answer { text } => format!("→ {}", text),
                    MessageContent::Info { text } => format!("ℹ {}", text),
                };
                
                ListItem::new(text).style(Style::default().fg(color))
            })
            .collect();
        
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Verlauf"));
        
        Widget::render(list, area, buf);
    }
    
    fn render_question(&self, msg: &WizardMessage, area: Rect, buf: &mut Buffer, theme: &Theme) {
        if let MessageContent::Question { text, options } = &msg.content {
            let block = Block::default()
                .title(format!("System [Schritt {}/{}]", 
                    self.current_question_index + 1,
                    self.messages.len()))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.wizard_system));
            
            let inner = block.inner(area);
            block.render(area, buf);
            
            // Split into text area and options area
            let text_height = (text.lines().count() as u16 + 2).min(inner.height / 2);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(text_height),
                    Constraint::Min(3),
                ])
                .split(inner);
            
            // Render question text
            let question_para = Paragraph::new(text.as_str())
                .style(Style::default().fg(theme.text))
                .alignment(Alignment::Left);
            question_para.render(chunks[0], buf);
            
            // Render options
            let option_items: Vec<ListItem> = options
                .iter()
                .enumerate()
                .map(|(i, opt)| {
                    let is_selected = i == self.selected_option_index;
                    let prefix = if is_selected { "  > " } else { "    " };
                    
                    let text = if let Some(metadata) = &opt.metadata {
                        format!("{}{} ({})", prefix, opt.label, metadata)
                    } else {
                        format!("{}{}", prefix, opt.label)
                    };
                    
                    let style = if is_selected {
                        Style::default()
                            .fg(theme.wizard_option_selected)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme.wizard_option_normal)
                    };
                    
                    ListItem::new(text).style(style)
                })
                .collect();
            
            let options_list = List::new(option_items);
            options_list.render(chunks[1], buf);
        }
    }
}

impl Default for WizardRenderer {
    fn default() -> Self {
        Self::new()
    }
}
