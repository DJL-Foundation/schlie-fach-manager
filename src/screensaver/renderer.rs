use std::time::Instant;
use super::animations::Animation;

/// Animation player that manages frame updates and timing
pub struct AnimationPlayer {
    animation: Box<dyn Animation>,
    current_frame: usize,
    last_frame_time: Instant,
}

impl AnimationPlayer {
    /// Create a new animation player with the given animation
    pub fn new(animation: Box<dyn Animation>) -> Self {
        Self {
            animation,
            current_frame: 0,
            last_frame_time: Instant::now(),
        }
    }
    
    /// Update the animation frame based on elapsed time
    pub fn update(&mut self) {
        let elapsed = self.last_frame_time.elapsed().as_millis() as u64;
        
        if elapsed >= self.animation.frame_delay_ms() {
            self.current_frame = (self.current_frame + 1) % self.animation.frames().len();
            self.last_frame_time = Instant::now();
        }
    }
    
    /// Get the current frame content
    pub fn current_frame(&self) -> &str {
        self.animation.frames()[self.current_frame]
    }
    
    /// Get the animation width
    pub fn width(&self) -> u16 {
        self.animation.width()
    }
    
    /// Get the animation height
    pub fn height(&self) -> u16 {
        self.animation.height()
    }
    
    /// Get the animation name
    pub fn name(&self) -> &str {
        self.animation.name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screensaver::animations::SpinningClock;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_animation_player_creation() {
        let anim = Box::new(SpinningClock);
        let player = AnimationPlayer::new(anim);
        
        assert_eq!(player.current_frame, 0);
        assert!(!player.current_frame().is_empty());
    }

    #[test]
    fn test_animation_player_cycles_frames() {
        let anim = Box::new(SpinningClock);
        let frame_count = anim.frames().len();
        let mut player = AnimationPlayer::new(anim);
        
        // Initially at frame 0
        assert_eq!(player.current_frame, 0);
        
        // Wait long enough for frame to advance
        thread::sleep(Duration::from_millis(200));
        player.update();
        
        // Should have advanced to next frame
        assert_eq!(player.current_frame, 1);
        
        // Advance through all frames
        for _ in 0..(frame_count - 1) {
            thread::sleep(Duration::from_millis(200));
            player.update();
        }
        
        // Should wrap back to 0
        assert_eq!(player.current_frame, 0);
    }

    #[test]
    fn test_animation_player_timing() {
        let anim = Box::new(SpinningClock);
        let mut player = AnimationPlayer::new(anim);
        
        let initial_frame = player.current_frame;
        
        // Update immediately - should not advance (not enough time elapsed)
        player.update();
        assert_eq!(player.current_frame, initial_frame);
        
        // Wait for frame delay
        thread::sleep(Duration::from_millis(200));
        player.update();
        
        // Should have advanced
        assert_ne!(player.current_frame, initial_frame);
    }

    #[test]
    fn test_animation_player_properties() {
        let anim = Box::new(SpinningClock);
        let player = AnimationPlayer::new(anim);
        
        assert_eq!(player.name(), "Spinning Clock");
        assert!(player.width() > 0);
        assert!(player.height() > 0);
    }
}
