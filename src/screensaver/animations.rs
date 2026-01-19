/// Trait for screensaver animations
pub trait Animation {
    /// Get the name of the animation
    fn name(&self) -> &str;
    
    /// Get the frames of the animation
    fn frames(&self) -> &[&str];
    
    /// Get the delay between frames in milliseconds
    fn frame_delay_ms(&self) -> u64;
    
    /// Get the width of the animation in characters
    fn width(&self) -> u16;
    
    /// Get the height of the animation in characters
    fn height(&self) -> u16;
}

/// 1. Spinning Clock Animation - Emoji clocks rotating
pub struct SpinningClock;

impl Animation for SpinningClock {
    fn name(&self) -> &str {
        "Spinning Clock"
    }
    
    fn frames(&self) -> &[&str] {
        &[
            "🕐", "🕑", "🕒", "🕓", "🕔", "🕕",
            "🕖", "🕗", "🕘", "🕙", "🕚", "🕛",
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

/// 2. Bouncing Box Animation - ASCII box with moving square
pub struct BouncingBox;

impl Animation for BouncingBox {
    fn name(&self) -> &str {
        "Bouncing Box"
    }
    
    fn frames(&self) -> &[&str] {
        &[
            r"┌─────────────┐
│             │
│   ┌─────┐   │
│   │  ■  │   │
│   └─────┘   │
│             │
└─────────────┘",
            r"┌─────────────┐
│  ┌─────┐    │
│  │  ■  │    │
│  └─────┘    │
│             │
│             │
└─────────────┘",
            r"┌─────────────┐
│┌─────┐      │
││  ■  │      │
│└─────┘      │
│             │
│             │
└─────────────┘",
            r"┌─────────────┐
│             │
│┌─────┐      │
││  ■  │      │
│└─────┘      │
│             │
└─────────────┘",
            r"┌─────────────┐
│             │
│             │
│┌─────┐      │
││  ■  │      │
│└─────┘      │
└─────────────┘",
            r"┌─────────────┐
│             │
│             │
│             │
│┌─────┐      │
│└─────┘      │
└─────────────┘",
            r"┌─────────────┐
│             │
│             │
│             │
│  ┌─────┐    │
│  └─────┘    │
└─────────────┘",
            r"┌─────────────┐
│             │
│             │
│             │
│    ┌─────┐  │
│    └─────┘  │
└─────────────┘",
            r"┌─────────────┐
│             │
│             │
│             │
│      ┌─────┐│
│      └─────┘│
└─────────────┘",
            r"┌─────────────┐
│             │
│             │
│      ┌─────┐│
│      │  ■  ││
│      └─────┘│
└─────────────┘",
            r"┌─────────────┐
│             │
│      ┌─────┐│
│      │  ■  ││
│      └─────┘│
│             │
└─────────────┘",
            r"┌─────────────┐
│      ┌─────┐│
│      │  ■  ││
│      └─────┘│
│             │
│             │
└─────────────┘",
            r"┌─────────────┐
│    ┌─────┐  │
│    │  ■  │  │
│    └─────┘  │
│             │
│             │
└─────────────┘",
            r"┌─────────────┐
│  ┌─────┐    │
│  │  ■  │    │
│  └─────┘    │
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

/// 3. Matrix Rain Animation - 0s and 1s falling
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
  1   1       ",
            r"    0   1     
  1       0   
      1       
0       1   1 
    1       0 
        0   1 
  1   1       
      0       ",
            r"  1       0   
      1       
0       1   1 
    1       0 
        0   1 
  1   1       
      0       
    1     0   ",
            r"      1       
0       1   1 
    1       0 
        0   1 
  1   1       
      0       
    1     0   
  0       1   ",
            r"0       1   1 
    1       0 
        0   1 
  1   1       
      0       
    1     0   
  0       1   
      1     0 ",
            r"    1       0 
        0   1 
  1   1       
      0       
    1     0   
  0       1   
      1     0 
1       0     ",
            r"        0   1 
  1   1       
      0       
    1     0   
  0       1   
      1     0 
1       0     
    0       1 ",
            r"  1   1       
      0       
    1     0   
  0       1   
      1     0 
1       0     
    0       1 
        1     ",
        ]
    }
    
    fn frame_delay_ms(&self) -> u64 {
        200
    }
    
    fn width(&self) -> u16 {
        20
    }
    
    fn height(&self) -> u16 {
        8
    }
}

/// 4. Loading Spinner Animation - Braille spinner
pub struct LoadingSpinner;

impl Animation for LoadingSpinner {
    fn name(&self) -> &str {
        "Loading Spinner"
    }
    
    fn frames(&self) -> &[&str] {
        &[
            r"    ╔════════════╗
    ║ ⠋ Loading  ║
    ╚════════════╝",
            r"    ╔════════════╗
    ║ ⠙ Loading  ║
    ╚════════════╝",
            r"    ╔════════════╗
    ║ ⠹ Loading  ║
    ╚════════════╝",
            r"    ╔════════════╗
    ║ ⠸ Loading  ║
    ╚════════════╝",
            r"    ╔════════════╗
    ║ ⠼ Loading  ║
    ╚════════════╝",
            r"    ╔════════════╗
    ║ ⠴ Loading  ║
    ╚════════════╝",
            r"    ╔════════════╗
    ║ ⠦ Loading  ║
    ╚════════════╝",
            r"    ╔════════════╗
    ║ ⠧ Loading  ║
    ╚════════════╝",
            r"    ╔════════════╗
    ║ ⠇ Loading  ║
    ╚════════════╝",
            r"    ╔════════════╗
    ║ ⠏ Loading  ║
    ╚════════════╝",
        ]
    }
    
    fn frame_delay_ms(&self) -> u64 {
        100
    }
    
    fn width(&self) -> u16 {
        18
    }
    
    fn height(&self) -> u16 {
        3
    }
}

/// 5. Waving Text Animation - Text moving up and down
pub struct WavingText;

impl Animation for WavingText {
    fn name(&self) -> &str {
        "Waving Text"
    }
    
    fn frames(&self) -> &[&str] {
        &[
            r"    ╔═══════════════════════╗
    ║                       ║
    ║   Schließfach-Manager ║
    ║                       ║
    ╚═══════════════════════╝",
            r"    ╔═══════════════════════╗
    ║  Schließfach-Manager  ║
    ║                       ║
    ║                       ║
    ╚═══════════════════════╝",
            r"    ╔═══════════════════════╗
    ║                       ║
    ║  Schließfach-Manager  ║
    ║                       ║
    ╚═══════════════════════╝",
            r"    ╔═══════════════════════╗
    ║                       ║
    ║                       ║
    ║  Schließfach-Manager  ║
    ╚═══════════════════════╝",
            r"    ╔═══════════════════════╗
    ║                       ║
    ║  Schließfach-Manager  ║
    ║                       ║
    ╚═══════════════════════╝",
            r"    ╔═══════════════════════╗
    ║  Schließfach-Manager  ║
    ║                       ║
    ║                       ║
    ╚═══════════════════════╝",
        ]
    }
    
    fn frame_delay_ms(&self) -> u64 {
        300
    }
    
    fn width(&self) -> u16 {
        27
    }
    
    fn height(&self) -> u16 {
        5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinning_clock_has_frames() {
        let anim = SpinningClock;
        assert_eq!(anim.frames().len(), 12);
        assert!(anim.frame_delay_ms() > 0);
        assert!(anim.width() > 0);
        assert!(anim.height() > 0);
    }

    #[test]
    fn test_bouncing_box_has_frames() {
        let anim = BouncingBox;
        assert!(!anim.frames().is_empty());
        assert!(anim.frame_delay_ms() > 0);
        assert!(anim.width() > 0);
        assert!(anim.height() > 0);
    }

    #[test]
    fn test_matrix_rain_has_frames() {
        let anim = MatrixRain;
        assert!(!anim.frames().is_empty());
        assert!(anim.frame_delay_ms() > 0);
        assert!(anim.width() > 0);
        assert!(anim.height() > 0);
    }

    #[test]
    fn test_loading_spinner_has_frames() {
        let anim = LoadingSpinner;
        assert_eq!(anim.frames().len(), 10);
        assert!(anim.frame_delay_ms() > 0);
        assert!(anim.width() > 0);
        assert!(anim.height() > 0);
    }

    #[test]
    fn test_waving_text_has_frames() {
        let anim = WavingText;
        assert!(!anim.frames().is_empty());
        assert!(anim.frame_delay_ms() > 0);
        assert!(anim.width() > 0);
        assert!(anim.height() > 0);
    }

    #[test]
    fn test_all_animations_have_valid_properties() {
        let animations: Vec<Box<dyn Animation>> = vec![
            Box::new(SpinningClock),
            Box::new(BouncingBox),
            Box::new(MatrixRain),
            Box::new(LoadingSpinner),
            Box::new(WavingText),
        ];

        for anim in animations {
            assert!(!anim.name().is_empty());
            assert!(!anim.frames().is_empty());
            assert!(anim.frame_delay_ms() > 0);
            assert!(anim.width() > 0);
            assert!(anim.height() > 0);
        }
    }
}
