//! Screensaver screen for fullscreen animation display.
//!
//! This module provides the screensaver rendering functionality for the TUI.
//! The screensaver displays a centered ASCII animation on a black background
//! with green (Matrix-style) text.

use ratatui::{
    layout::{Alignment, Rect},
    prelude::Frame,
    style::Style,
    widgets::{Block, Clear, Paragraph},
};

use crate::screensaver::ScreensaverScreen as AnimationScreen;
use crate::ui::theme::Theme;

/// Renders the screensaver screen.
///
/// The screensaver takes over the entire terminal:
/// - Black background fills the entire screen
/// - Animation is centered both horizontally and vertically
/// - Text is rendered in green (Matrix-style)
/// - No header, keybind bar, or status bar is shown
///
/// # Arguments
///
/// * `frame` - The ratatui frame to render into
/// * `area` - The area to render (typically the full terminal)
/// * `screen` - The screensaver screen state containing the animation player
pub fn render_screensaver(frame: &mut Frame, area: Rect, screen: &AnimationScreen) {
    // Clear the entire area with black background
    let background = Block::default().style(Style::default().bg(Theme::SCREENSAVER_BG));
    frame.render_widget(Clear, area);
    frame.render_widget(background, area);

    // Get animation dimensions
    let player = screen.player();
    let anim_width = player.width();
    let anim_height = player.height();

    // Calculate center position
    let x = area.x + (area.width.saturating_sub(anim_width)) / 2;
    let y = area.y + (area.height.saturating_sub(anim_height)) / 2;

    // Create centered area for the animation
    let center_area = Rect {
        x,
        y,
        width: anim_width.min(area.width),
        height: anim_height.min(area.height),
    };

    // Render the current animation frame
    let frame_content = player.current_frame();
    let paragraph = Paragraph::new(frame_content)
        .style(Theme::screensaver_fg())
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, center_area);
}

/// Creates a new screensaver screen with a random animation.
pub fn create_screensaver_screen() -> AnimationScreen {
    let animation = crate::screensaver::get_random_animation();
    AnimationScreen::new(animation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screensaver::SpinningClock;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn test_create_screensaver_screen() {
        let screen = create_screensaver_screen();
        assert!(!screen.player().current_frame().is_empty());
    }

    #[test]
    fn test_render_screensaver_does_not_panic() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let screen = AnimationScreen::new(Box::new(SpinningClock));

        terminal
            .draw(|frame| {
                let area = frame.size();
                render_screensaver(frame, area, &screen);
            })
            .unwrap();
    }

    #[test]
    fn test_render_screensaver_small_terminal() {
        let backend = TestBackend::new(10, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        let screen = AnimationScreen::new(Box::new(SpinningClock));

        terminal
            .draw(|frame| {
                let area = frame.size();
                render_screensaver(frame, area, &screen);
            })
            .unwrap();
    }

    #[test]
    fn test_render_screensaver_large_terminal() {
        let backend = TestBackend::new(200, 60);
        let mut terminal = Terminal::new(backend).unwrap();

        let screen = AnimationScreen::new(Box::new(SpinningClock));

        terminal
            .draw(|frame| {
                let area = frame.size();
                render_screensaver(frame, area, &screen);
            })
            .unwrap();
    }
}
