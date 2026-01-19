use crate::ui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Describes whether a keybind is global or context-specific.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeybindScope {
    Global,
    Context,
}

/// Single keybind entry displayed in the keybind bar.
#[derive(Debug, Clone)]
pub struct Keybind {
    pub key: String,
    pub description: String,
    pub scope: KeybindScope,
}

/// Widget state for the keybind bar.
#[derive(Debug, Clone, Default)]
pub struct KeybindBar {
    pub global_binds: Vec<Keybind>,
    pub context_binds: Vec<Keybind>,
    pub context_message: Option<String>,
}

impl KeybindBar {
    /// Render the keybind bar into the buffer.
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let mut lines = Vec::new();
        let global_line = self.format_keybinds(&self.global_binds, theme);
        lines.push(global_line);

        if let Some(message) = &self.context_message {
            lines.push(Line::from(vec![Span::styled(
                message.clone(),
                theme.status_info,
            )]));
        } else {
            let context_line = self.format_keybinds(&self.context_binds, theme);
            lines.push(context_line);
        }

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::TOP))
            .style(Style::default().bg(theme.background));
        paragraph.render(area, buf);
    }

    /// Formats a list of keybinds into a line of spans.
    fn format_keybinds(&self, binds: &[Keybind], theme: &Theme) -> Line<'static> {
        let mut spans: Vec<Span<'static>> = Vec::new();
        for (idx, bind) in binds.iter().enumerate() {
            if idx > 0 {
                spans.push(Span::raw("  "));
            }

            let color = match bind.scope {
                KeybindScope::Global => theme.keybind_global,
                KeybindScope::Context => theme.keybind_context,
            };
            let key_style = Style::default().fg(color).add_modifier(Modifier::BOLD);
            spans.push(Span::styled(format!("[{}]", bind.key), key_style));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(
                bind.description.clone(),
                Style::default().fg(color),
            ));
        }
        Line::from(spans)
    }
}
