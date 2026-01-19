use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

/// Keybind bar component for displaying global and context-specific keybinds
/// Based on v2.1 spec section 4.3
#[derive(Debug, Clone)]
pub struct KeybindBar {
    global_binds: Vec<Keybind>,
    context_binds: Vec<Keybind>,
    context_message: Option<String>,
}

/// A single keybind
#[derive(Debug, Clone)]
pub struct Keybind {
    pub key: String,
    pub description: String,
    pub scope: KeybindScope,
}

/// Scope of a keybind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeybindScope {
    Global,
    Context,
}

impl KeybindBar {
    pub fn new() -> Self {
        Self {
            global_binds: Self::default_global_binds(),
            context_binds: Vec::new(),
            context_message: None,
        }
    }
    
    /// Default global keybinds that are always visible
    fn default_global_binds() -> Vec<Keybind> {
        vec![
            Keybind {
                key: "Tab".to_string(),
                description: "Nächster".to_string(),
                scope: KeybindScope::Global,
            },
            Keybind {
                key: "Shift+Tab".to_string(),
                description: "Vorheriger".to_string(),
                scope: KeybindScope::Global,
            },
            Keybind {
                key: "^".to_string(),
                description: "Window Switcher".to_string(),
                scope: KeybindScope::Global,
            },
            Keybind {
                key: "Shift+Q".to_string(),
                description: "Beenden".to_string(),
                scope: KeybindScope::Global,
            },
            Keybind {
                key: "Esc".to_string(),
                description: "Zurück (3x=Dashboard)".to_string(),
                scope: KeybindScope::Global,
            },
        ]
    }
    
    /// Set context-specific keybinds
    pub fn set_context_binds(&mut self, binds: Vec<Keybind>) {
        self.context_binds = binds;
        self.context_message = None;
    }
    
    /// Set a context message (replaces context keybinds temporarily)
    pub fn set_context_message(&mut self, message: String) {
        self.context_message = Some(message);
    }
    
    /// Clear the context message
    pub fn clear_context_message(&mut self) {
        self.context_message = None;
    }
    
    /// Render the keybind bar
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        // Split into two lines
        let line_height = (area.height / 2).max(1);
        let line1_y = area.y;
        let line2_y = area.y + line_height;
        
        let line1_area = Rect {
            x: area.x,
            y: line1_y,
            width: area.width,
            height: line_height,
        };
        
        let line2_area = Rect {
            x: area.x,
            y: line2_y,
            width: area.width,
            height: area.height.saturating_sub(line_height),
        };
        
        // Line 1: Global binds (accent color)
        let global_line = self.format_keybinds(&self.global_binds, theme.keybind_global);
        let global_para = Paragraph::new(global_line)
            .alignment(Alignment::Left);
        global_para.render(line1_area, buf);
        
        // Line 2: Context message OR context binds
        if let Some(msg) = &self.context_message {
            let context_para = Paragraph::new(msg.as_str())
                .style(theme.status_info)
                .alignment(Alignment::Left);
            context_para.render(line2_area, buf);
        } else {
            let context_line = self.format_keybinds(&self.context_binds, theme.keybind_context);
            let context_para = Paragraph::new(context_line)
                .alignment(Alignment::Left);
            context_para.render(line2_area, buf);
        }
    }
    
    /// Format keybinds as a line with [Key] Description pattern
    fn format_keybinds<'a>(&self, binds: &'a [Keybind], color: ratatui::style::Color) -> Line<'a> {
        let mut spans = Vec::new();
        
        for (i, bind) in binds.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw("  "));
            }
            
            // [Key]
            spans.push(Span::styled(
                format!("[{}]", bind.key),
                Style::default().fg(color),
            ));
            
            // Description
            spans.push(Span::raw(" "));
            spans.push(Span::styled(
                &bind.description,
                Style::default().fg(color),
            ));
        }
        
        Line::from(spans)
    }
}

impl Default for KeybindBar {
    fn default() -> Self {
        Self::new()
    }
}
