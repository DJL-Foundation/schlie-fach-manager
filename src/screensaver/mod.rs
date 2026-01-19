pub mod animations;
pub mod renderer;

use animations::{Animation, BouncingBox, LoadingSpinner, MatrixRain, SpinningClock, WavingText};
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns a randomly selected animation for the screensaver.
pub fn get_random_animation() -> Box<dyn Animation> {
    let animations: Vec<Box<dyn Animation>> = vec![
        Box::new(SpinningClock),
        Box::new(BouncingBox),
        Box::new(MatrixRain),
        Box::new(LoadingSpinner),
        Box::new(WavingText),
    ];

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.subsec_nanos())
        .unwrap_or(0);
    let index = (nanos as usize) % animations.len();
    animations
        .into_iter()
        .nth(index)
        .unwrap_or_else(|| Box::new(SpinningClock))
}
