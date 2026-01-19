use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph},
};
use crate::screensaver::{get_random_animation, AnimationPlayer};

/// Screensaver screen that displays animated ASCII art
/// 
/// This screen has no header, keybind bar, or status bar - it's fullscreen.
/// The animation is centered on a black background with Matrix-style green text.
pub struct ScreensaverScreen {
    player: AnimationPlayer,
}

impl ScreensaverScreen {
    /// Create a new screensaver screen with a random animation
    pub fn new() -> Self {
        let animation = get_random_animation();
        Self {
            player: AnimationPlayer::new(animation),
        }
    }
    
    /// Update the animation (advance frames based on timing)
    pub fn update(&mut self) {
        self.player.update();
    }
    
    /// Render the screensaver to the terminal
    pub fn render(&self, f: &mut Frame) {
        let area = f.area();
        
        // Clear background (all black)
        f.render_widget(
            Block::default().style(Style::default().bg(Color::Black)),
            area,
        );
        
        // Calculate center position
        let anim_width = self.player.width();
        let anim_height = self.player.height();
        
        let x = (area.width.saturating_sub(anim_width)) / 2;
        let y = (area.height.saturating_sub(anim_height)) / 2;
        
        // Create centered area
        let center_area = Rect {
            x: area.x + x,
            y: area.y + y,
            width: anim_width,
            height: anim_height,
        };
        
        // Render animation frame
        let frame_content = self.player.current_frame();
        let paragraph = Paragraph::new(Text::raw(frame_content))
            .style(Style::default().fg(Color::Green)) // Matrix-style green
            .alignment(Alignment::Left);
        
        f.render_widget(paragraph, center_area);
    }
    
    /// Get the name of the current animation
    #[allow(dead_code)]
    pub fn animation_name(&self) -> &str {
        self.player.name()
    }
}

impl Default for ScreensaverScreen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screensaver_screen_creation() {
        let screen = ScreensaverScreen::new();
        assert!(!screen.animation_name().is_empty());
    }

    #[test]
    fn test_screensaver_screen_update() {
        let mut screen = ScreensaverScreen::new();
        // Should not panic
        screen.update();
    }

    #[test]
    fn test_screensaver_screen_default() {
        let screen = ScreensaverScreen::default();
        assert!(!screen.animation_name().is_empty());
    }
}
