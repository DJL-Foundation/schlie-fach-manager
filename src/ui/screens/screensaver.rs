use crate::{
    screensaver::{animations::AnimationPlayer, get_random_animation, renderer::render_animation},
    ui::theme::Theme,
};
use ratatui::Frame;

/// Screensaver screen displaying animated ASCII art.
pub struct ScreensaverScreen {
    pub player: AnimationPlayer,
}

impl ScreensaverScreen {
    /// Creates a new screensaver screen with a random animation.
    pub fn new() -> Self {
        let animation = get_random_animation();
        Self {
            player: AnimationPlayer::new(animation),
        }
    }

    /// Updates the animation frame.
    pub fn update(&mut self) {
        self.player.update();
    }

    /// Renders the screensaver.
    pub fn render(&self, frame: &mut Frame<'_>, theme: &Theme) {
        render_animation(frame, &self.player, theme);
    }
}
