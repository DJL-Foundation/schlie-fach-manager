//! Animation rendering logic for the screensaver module.
//!
//! This module provides the [`AnimationPlayer`] for managing animation state
//! and the [`ScreensaverScreen`] for rendering fullscreen animations.

use std::time::Instant;

use super::animations::Animation;

/// Manages the playback of an animation.
///
/// Tracks the current frame and handles timing for frame transitions.
///
/// # Example
///
/// ```
/// use schliessfach_manager::screensaver::{AnimationPlayer, SpinningClock};
///
/// let mut player = AnimationPlayer::new(Box::new(SpinningClock));
///
/// // Get the current frame
/// let frame = player.current_frame();
///
/// // Update the animation (call periodically)
/// player.update();
/// ```
pub struct AnimationPlayer {
    animation: Box<dyn Animation>,
    current_frame: usize,
    last_frame_time: Instant,
}

impl AnimationPlayer {
    /// Creates a new animation player with the given animation.
    pub fn new(animation: Box<dyn Animation>) -> Self {
        Self {
            animation,
            current_frame: 0,
            last_frame_time: Instant::now(),
        }
    }

    /// Updates the animation state, advancing frames if enough time has passed.
    ///
    /// This method should be called regularly (e.g., in the main event loop)
    /// to ensure smooth animation playback.
    pub fn update(&mut self) {
        let elapsed = self.last_frame_time.elapsed().as_millis() as u64;

        if elapsed >= self.animation.frame_delay_ms() {
            self.current_frame = (self.current_frame + 1) % self.animation.frames().len();
            self.last_frame_time = Instant::now();
        }
    }

    /// Returns the current frame content.
    pub fn current_frame(&self) -> &str {
        self.animation.frames()[self.current_frame]
    }

    /// Returns the animation being played.
    pub fn animation(&self) -> &dyn Animation {
        self.animation.as_ref()
    }

    /// Returns the width of the animation in characters.
    pub fn width(&self) -> u16 {
        self.animation.width()
    }

    /// Returns the height of the animation in lines.
    pub fn height(&self) -> u16 {
        self.animation.height()
    }

    /// Resets the animation to the first frame.
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.last_frame_time = Instant::now();
    }
}

/// A fullscreen screensaver screen that displays a centered animation.
///
/// The screen has a black background and renders the animation text in green
/// (Matrix-style) in the center of the terminal.
pub struct ScreensaverScreen {
    player: AnimationPlayer,
}

impl ScreensaverScreen {
    /// Creates a new screensaver screen with the given animation.
    pub fn new(animation: Box<dyn Animation>) -> Self {
        Self {
            player: AnimationPlayer::new(animation),
        }
    }

    /// Updates the animation state.
    pub fn update(&mut self) {
        self.player.update();
    }

    /// Returns a reference to the animation player.
    pub fn player(&self) -> &AnimationPlayer {
        &self.player
    }

    /// Returns a mutable reference to the animation player.
    pub fn player_mut(&mut self) -> &mut AnimationPlayer {
        &mut self.player
    }

    /// Resets the screensaver with a new animation.
    pub fn set_animation(&mut self, animation: Box<dyn Animation>) {
        self.player = AnimationPlayer::new(animation);
    }
}

impl Default for ScreensaverScreen {
    fn default() -> Self {
        Self::new(Box::new(super::animations::SpinningClock))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screensaver::animations::{LoadingSpinner, SpinningClock};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_animation_player_new() {
        let player = AnimationPlayer::new(Box::new(SpinningClock));
        assert_eq!(player.current_frame, 0);
        assert!(!player.current_frame().is_empty());
    }

    #[test]
    fn test_animation_player_current_frame() {
        let player = AnimationPlayer::new(Box::new(SpinningClock));
        let frame = player.current_frame();
        assert!(SpinningClock.frames().contains(&frame));
    }

    #[test]
    fn test_animation_player_update_advances_frame() {
        let mut player = AnimationPlayer::new(Box::new(SpinningClock));
        let initial_frame = player.current_frame;

        // Wait longer than the frame delay
        thread::sleep(Duration::from_millis(SpinningClock.frame_delay_ms() + 50));
        player.update();

        // Frame should have advanced
        assert_ne!(player.current_frame, initial_frame);
    }

    #[test]
    fn test_animation_player_wraps_around() {
        let mut player = AnimationPlayer::new(Box::new(SpinningClock));
        let total_frames = SpinningClock.frames().len();

        // Manually set to last frame
        player.current_frame = total_frames - 1;
        player.last_frame_time = Instant::now() - Duration::from_millis(200);

        player.update();

        // Should wrap to first frame
        assert_eq!(player.current_frame, 0);
    }

    #[test]
    fn test_animation_player_reset() {
        let mut player = AnimationPlayer::new(Box::new(SpinningClock));
        player.current_frame = 5;

        player.reset();

        assert_eq!(player.current_frame, 0);
    }

    #[test]
    fn test_animation_player_width_height() {
        let player = AnimationPlayer::new(Box::new(SpinningClock));
        assert_eq!(player.width(), SpinningClock.width());
        assert_eq!(player.height(), SpinningClock.height());
    }

    #[test]
    fn test_screensaver_screen_new() {
        let screen = ScreensaverScreen::new(Box::new(SpinningClock));
        assert!(!screen.player.current_frame().is_empty());
    }

    #[test]
    fn test_screensaver_screen_default() {
        let screen = ScreensaverScreen::default();
        assert_eq!(screen.player.animation().name(), "Spinning Clock");
    }

    #[test]
    fn test_screensaver_screen_update() {
        let mut screen = ScreensaverScreen::new(Box::new(SpinningClock));

        // Wait and update
        thread::sleep(Duration::from_millis(SpinningClock.frame_delay_ms() + 50));
        screen.update();

        // Should not panic
    }

    #[test]
    fn test_screensaver_screen_set_animation() {
        let mut screen = ScreensaverScreen::new(Box::new(SpinningClock));
        assert_eq!(screen.player.animation().name(), "Spinning Clock");

        screen.set_animation(Box::new(LoadingSpinner));
        assert_eq!(screen.player.animation().name(), "Loading Spinner");
    }
}
