//! Screensaver module for the Schließfach-Manager application.
//!
//! This module provides ASCII animations for the screensaver feature,
//! which activates after a configurable period of inactivity.
//!
//! # Features
//!
//! - Multiple pre-built animations
//! - Random animation selection
//! - Thread-safe animation types
//! - Configurable frame delays
//!
//! # Usage
//!
//! ```
//! use schliessfach_manager::screensaver::{get_random_animation, AnimationPlayer};
//!
//! // Get a random animation
//! let animation = get_random_animation();
//!
//! // Create a player to manage the animation
//! let mut player = AnimationPlayer::new(animation);
//!
//! // Update the animation (call periodically)
//! player.update();
//!
//! // Get the current frame content
//! let frame = player.current_frame();
//! ```
//!
//! # Available Animations
//!
//! - [`SpinningClock`] - Clock emoji faces rotating (🕐 through 🕛)
//! - [`BouncingBox`] - A box bouncing inside a container
//! - [`MatrixRain`] - Matrix-style falling 0s and 1s
//! - [`LoadingSpinner`] - Braille character spinner
//! - [`WavingText`] - "Schließfach-Manager" text moving up and down

pub mod animations;
pub mod renderer;

pub use animations::{
    all_animations, Animation, BouncingBox, LoadingSpinner, MatrixRain, SpinningClock, WavingText,
};
pub use renderer::{AnimationPlayer, ScreensaverScreen};

/// Returns a randomly selected animation from the available animations.
///
/// Uses a simple deterministic selection based on the current time to avoid
/// requiring the `rand` crate dependency.
///
/// # Example
///
/// ```
/// use schliessfach_manager::screensaver::get_random_animation;
///
/// let animation = get_random_animation();
/// println!("Selected animation: {}", animation.name());
/// ```
pub fn get_random_animation() -> Box<dyn Animation> {
    let animations = all_animations();

    // Use current time as a simple pseudo-random source
    // This avoids adding a rand dependency just for this feature
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as usize)
        .unwrap_or(0);

    let index = now % animations.len();

    // We need to create a new boxed animation since we can't move out of a Vec
    match index {
        0 => Box::new(SpinningClock),
        1 => Box::new(BouncingBox),
        2 => Box::new(MatrixRain),
        3 => Box::new(LoadingSpinner),
        _ => Box::new(WavingText),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_animation_returns_valid_animation() {
        let anim = get_random_animation();
        assert!(!anim.name().is_empty());
        assert!(!anim.frames().is_empty());
        assert!(anim.frame_delay_ms() > 0);
        assert!(anim.width() > 0);
        assert!(anim.height() > 0);
    }

    #[test]
    fn test_get_random_animation_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        let anim = get_random_animation();
        // The trait object must be Send + Sync
        fn check_animation(_: &(dyn Animation + Send + Sync)) {}
        check_animation(anim.as_ref());
    }

    #[test]
    fn test_module_exports() {
        // Verify all expected types are exported
        let _: Box<dyn Animation> = Box::new(SpinningClock);
        let _: Box<dyn Animation> = Box::new(BouncingBox);
        let _: Box<dyn Animation> = Box::new(MatrixRain);
        let _: Box<dyn Animation> = Box::new(LoadingSpinner);
        let _: Box<dyn Animation> = Box::new(WavingText);
    }

    #[test]
    fn test_all_animations_exported() {
        let anims = all_animations();
        assert_eq!(anims.len(), 5);
    }
}
