//! Screensaver module for the Schließfach-Manager
//! 
//! Provides ASCII animations that activate after a period of inactivity.

pub mod animations;
pub mod renderer;

pub use animations::{
    Animation, SpinningClock, BouncingBox, MatrixRain, LoadingSpinner, WavingText
};
pub use renderer::AnimationPlayer;

use std::time::SystemTime;

/// Get a random animation using system time as seed
/// 
/// Since the `rand` crate is not in dependencies, we use SystemTime to select
/// a pseudo-random animation from the available options.
pub fn get_random_animation() -> Box<dyn Animation> {
    let animations: [fn() -> Box<dyn Animation>; 5] = [
        || Box::new(SpinningClock),
        || Box::new(BouncingBox),
        || Box::new(MatrixRain),
        || Box::new(LoadingSpinner),
        || Box::new(WavingText),
    ];
    
    // Use system time to pick a pseudo-random animation
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    let index = (timestamp % animations.len() as u128) as usize;
    animations[index]()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_animation() {
        // Get multiple animations and ensure they're valid
        for _ in 0..10 {
            let anim = get_random_animation();
            assert!(!anim.name().is_empty());
            assert!(!anim.frames().is_empty());
            assert!(anim.frame_delay_ms() > 0);
            assert!(anim.width() > 0);
            assert!(anim.height() > 0);
        }
    }

    #[test]
    fn test_all_animation_types_can_be_created() {
        let animations: Vec<Box<dyn Animation>> = vec![
            Box::new(SpinningClock),
            Box::new(BouncingBox),
            Box::new(MatrixRain),
            Box::new(LoadingSpinner),
            Box::new(WavingText),
        ];

        assert_eq!(animations.len(), 5);
        
        for anim in animations {
            assert!(!anim.name().is_empty());
        }
    }
}
