//! Animation definitions for the screensaver module.
//!
//! This module provides the [`Animation`] trait and implementations of various
//! ASCII animations for the screensaver feature.

/// A trait for screensaver animations.
///
/// Implementations must be thread-safe (`Send + Sync`) to allow for
/// potential async operations.
///
/// # Example
///
/// ```
/// use schliessfach_manager::screensaver::Animation;
///
/// struct MyAnimation;
///
/// impl Animation for MyAnimation {
///     fn name(&self) -> &str { "My Animation" }
///     fn frames(&self) -> &[&str] { &["Frame 1", "Frame 2"] }
///     fn frame_delay_ms(&self) -> u64 { 100 }
///     fn width(&self) -> u16 { 10 }
///     fn height(&self) -> u16 { 1 }
/// }
/// ```
pub trait Animation: Send + Sync {
    /// Returns the name of the animation.
    fn name(&self) -> &str;

    /// Returns all frames of the animation.
    fn frames(&self) -> &[&str];

    /// Returns the delay between frames in milliseconds.
    fn frame_delay_ms(&self) -> u64;

    /// Returns the width of the animation in characters.
    fn width(&self) -> u16;

    /// Returns the height of the animation in lines.
    fn height(&self) -> u16;
}

/// A spinning clock animation using clock emoji faces.
///
/// Shows clock faces from 🕐 through 🕛, creating a rotating effect.
#[derive(Debug, Clone, Copy, Default)]
pub struct SpinningClock;

impl Animation for SpinningClock {
    fn name(&self) -> &str {
        "Spinning Clock"
    }

    fn frames(&self) -> &[&str] {
        &[
            "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛",
        ]
    }

    fn frame_delay_ms(&self) -> u64 {
        150
    }

    fn width(&self) -> u16 {
        2
    }

    fn height(&self) -> u16 {
        1
    }
}

/// A bouncing box animation.
///
/// Shows a small box moving around inside a larger container.
#[derive(Debug, Clone, Copy, Default)]
pub struct BouncingBox;

impl Animation for BouncingBox {
    fn name(&self) -> &str {
        "Bouncing Box"
    }

    fn frames(&self) -> &[&str] {
        &[
            r"┌─────────────┐
│             │
│   ┌───┐     │
│   │ ■ │     │
│   └───┘     │
│             │
└─────────────┘",
            r"┌─────────────┐
│  ┌───┐      │
│  │ ■ │      │
│  └───┘      │
│             │
│             │
└─────────────┘",
            r"┌─────────────┐
│┌───┐        │
││ ■ │        │
│└───┘        │
│             │
│             │
└─────────────┘",
            r"┌─────────────┐
│             │
│┌───┐        │
││ ■ │        │
│└───┘        │
│             │
└─────────────┘",
            r"┌─────────────┐
│             │
│             │
│┌───┐        │
││ ■ │        │
│└───┘        │
└─────────────┘",
            r"┌─────────────┐
│             │
│             │
│  ┌───┐      │
│  │ ■ │      │
│  └───┘      │
└─────────────┘",
            r"┌─────────────┐
│             │
│             │
│     ┌───┐   │
│     │ ■ │   │
│     └───┘   │
└─────────────┘",
            r"┌─────────────┐
│             │
│             │
│       ┌───┐ │
│       │ ■ │ │
│       └───┘ │
└─────────────┘",
            r"┌─────────────┐
│             │
│       ┌───┐ │
│       │ ■ │ │
│       └───┘ │
│             │
└─────────────┘",
            r"┌─────────────┐
│       ┌───┐ │
│       │ ■ │ │
│       └───┘ │
│             │
│             │
└─────────────┘",
            r"┌─────────────┐
│     ┌───┐   │
│     │ ■ │   │
│     └───┘   │
│             │
│             │
└─────────────┘",
            r"┌─────────────┐
│   ┌───┐     │
│   │ ■ │     │
│   └───┘     │
│             │
│             │
└─────────────┘",
        ]
    }

    fn frame_delay_ms(&self) -> u64 {
        100
    }

    fn width(&self) -> u16 {
        15
    }

    fn height(&self) -> u16 {
        7
    }
}

/// A Matrix-style rain animation with falling 0s and 1s.
#[derive(Debug, Clone, Copy, Default)]
pub struct MatrixRain;

impl Animation for MatrixRain {
    fn name(&self) -> &str {
        "Matrix Rain"
    }

    fn frames(&self) -> &[&str] {
        &[
            r"  1   0     1  
    0   1     0
  1       0    
      1       1
0       1      
    1       0  
        0   1  
  1   1        ",
            r"    0   1      
  1       0    
      1        
0       1   1  
    1       0  
        0   1  
  1   1        
      0        ",
            r"  1       0    
      1        
0       1   1  
    1       0  
        0   1  
  1   1        
      0        
    0     1    ",
            r"      1        
0       1   1  
    1       0  
        0   1  
  1   1        
      0        
    0     1    
  1   0     1  ",
            r"0       1   1  
    1       0  
        0   1  
  1   1        
      0        
    0     1    
  1   0     1  
    0   1     0",
            r"    1       0  
        0   1  
  1   1        
      0        
    0     1    
  1   0     1  
    0   1     0
  1       0    ",
            r"        0   1  
  1   1        
      0        
    0     1    
  1   0     1  
    0   1     0
  1       0    
      1       1",
            r"  1   1        
      0        
    0     1    
  1   0     1  
    0   1     0
  1       0    
      1       1
0       1      ",
        ]
    }

    fn frame_delay_ms(&self) -> u64 {
        200
    }

    fn width(&self) -> u16 {
        15
    }

    fn height(&self) -> u16 {
        8
    }
}

/// A loading spinner using braille characters.
#[derive(Debug, Clone, Copy, Default)]
pub struct LoadingSpinner;

impl Animation for LoadingSpinner {
    fn name(&self) -> &str {
        "Loading Spinner"
    }

    fn frames(&self) -> &[&str] {
        &[
            r"╔════════════╗
║ ⠋ Loading  ║
╚════════════╝",
            r"╔════════════╗
║ ⠙ Loading  ║
╚════════════╝",
            r"╔════════════╗
║ ⠹ Loading  ║
╚════════════╝",
            r"╔════════════╗
║ ⠸ Loading  ║
╚════════════╝",
            r"╔════════════╗
║ ⠼ Loading  ║
╚════════════╝",
            r"╔════════════╗
║ ⠴ Loading  ║
╚════════════╝",
            r"╔════════════╗
║ ⠦ Loading  ║
╚════════════╝",
            r"╔════════════╗
║ ⠧ Loading  ║
╚════════════╝",
            r"╔════════════╗
║ ⠇ Loading  ║
╚════════════╝",
            r"╔════════════╗
║ ⠏ Loading  ║
╚════════════╝",
        ]
    }

    fn frame_delay_ms(&self) -> u64 {
        100
    }

    fn width(&self) -> u16 {
        14
    }

    fn height(&self) -> u16 {
        3
    }
}

/// A waving text animation showing "Schließfach-Manager" moving up and down.
#[derive(Debug, Clone, Copy, Default)]
pub struct WavingText;

impl Animation for WavingText {
    fn name(&self) -> &str {
        "Waving Text"
    }

    fn frames(&self) -> &[&str] {
        &[
            r"╔═══════════════════════╗
║                       ║
║ Schließfach-Manager   ║
║                       ║
╚═══════════════════════╝",
            r"╔═══════════════════════╗
║ Schließfach-Manager   ║
║                       ║
║                       ║
╚═══════════════════════╝",
            r"╔═══════════════════════╗
║                       ║
║ Schließfach-Manager   ║
║                       ║
╚═══════════════════════╝",
            r"╔═══════════════════════╗
║                       ║
║                       ║
║ Schließfach-Manager   ║
╚═══════════════════════╝",
            r"╔═══════════════════════╗
║                       ║
║ Schließfach-Manager   ║
║                       ║
╚═══════════════════════╝",
            r"╔═══════════════════════╗
║ Schließfach-Manager   ║
║                       ║
║                       ║
╚═══════════════════════╝",
        ]
    }

    fn frame_delay_ms(&self) -> u64 {
        300
    }

    fn width(&self) -> u16 {
        25
    }

    fn height(&self) -> u16 {
        5
    }
}

/// Returns all available animations.
pub fn all_animations() -> Vec<Box<dyn Animation>> {
    vec![
        Box::new(SpinningClock),
        Box::new(BouncingBox),
        Box::new(MatrixRain),
        Box::new(LoadingSpinner),
        Box::new(WavingText),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinning_clock_properties() {
        let anim = SpinningClock;
        assert_eq!(anim.name(), "Spinning Clock");
        assert_eq!(anim.frames().len(), 12);
        assert_eq!(anim.frame_delay_ms(), 150);
        assert_eq!(anim.width(), 2);
        assert_eq!(anim.height(), 1);
    }

    #[test]
    fn test_bouncing_box_properties() {
        let anim = BouncingBox;
        assert_eq!(anim.name(), "Bouncing Box");
        assert!(!anim.frames().is_empty());
        assert_eq!(anim.frame_delay_ms(), 100);
        assert_eq!(anim.width(), 15);
        assert_eq!(anim.height(), 7);
    }

    #[test]
    fn test_matrix_rain_properties() {
        let anim = MatrixRain;
        assert_eq!(anim.name(), "Matrix Rain");
        assert!(!anim.frames().is_empty());
        assert_eq!(anim.frame_delay_ms(), 200);
        assert_eq!(anim.width(), 15);
        assert_eq!(anim.height(), 8);
    }

    #[test]
    fn test_loading_spinner_properties() {
        let anim = LoadingSpinner;
        assert_eq!(anim.name(), "Loading Spinner");
        assert_eq!(anim.frames().len(), 10);
        assert_eq!(anim.frame_delay_ms(), 100);
        assert_eq!(anim.width(), 14);
        assert_eq!(anim.height(), 3);
    }

    #[test]
    fn test_waving_text_properties() {
        let anim = WavingText;
        assert_eq!(anim.name(), "Waving Text");
        assert!(!anim.frames().is_empty());
        assert_eq!(anim.frame_delay_ms(), 300);
        assert_eq!(anim.width(), 25);
        assert_eq!(anim.height(), 5);
    }

    #[test]
    fn test_all_animations() {
        let anims = all_animations();
        assert_eq!(anims.len(), 5);
    }

    #[test]
    fn test_animation_frames_not_empty() {
        for anim in all_animations() {
            assert!(
                !anim.frames().is_empty(),
                "Animation '{}' has no frames",
                anim.name()
            );
        }
    }

    #[test]
    fn test_animation_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SpinningClock>();
        assert_send_sync::<BouncingBox>();
        assert_send_sync::<MatrixRain>();
        assert_send_sync::<LoadingSpinner>();
        assert_send_sync::<WavingText>();
    }
}
