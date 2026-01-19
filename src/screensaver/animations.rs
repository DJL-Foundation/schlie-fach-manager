use std::time::Instant;

/// Trait that defines a screensaver animation.
pub trait Animation {
    fn name(&self) -> &str;
    fn frames(&self) -> &[&str];
    fn frame_delay_ms(&self) -> u64;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}

/// Animation player that cycles through frames.
pub struct AnimationPlayer {
    animation: Box<dyn Animation>,
    current_frame: usize,
    last_frame_time: Instant,
}

impl AnimationPlayer {
    /// Creates a new animation player.
    pub fn new(animation: Box<dyn Animation>) -> Self {
        Self {
            animation,
            current_frame: 0,
            last_frame_time: Instant::now(),
        }
    }

    /// Updates the current frame if enough time has elapsed.
    pub fn update(&mut self) {
        let elapsed = self.last_frame_time.elapsed().as_millis() as u64;
        if elapsed >= self.animation.frame_delay_ms() {
            self.current_frame = (self.current_frame + 1) % self.animation.frames().len();
            self.last_frame_time = Instant::now();
        }
    }

    /// Returns the current frame contents.
    pub fn current_frame(&self) -> &str {
        self.animation.frames()[self.current_frame]
    }

    /// Returns the underlying animation.
    pub fn animation(&self) -> &dyn Animation {
        self.animation.as_ref()
    }
}

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

pub struct BouncingBox;

impl Animation for BouncingBox {
    fn name(&self) -> &str {
        "Bouncing Box"
    }

    fn frames(&self) -> &[&str] {
        &[
            r"
┌─────────────┐
│             │
│   ┌─────┐   │
│   │  ■  │   │
│   └─────┘   │
│             │
└─────────────┘",
            r"
┌─────────────┐
│  ┌─────┐    │
│  │  ■  │    │
│  └─────┘    │
│             │
│             │
└─────────────┘",
            r"
┌─────────────┐
│┌─────┐      │
││  ■  │      │
│└─────┘      │
│             │
│             │
└─────────────┘",
            r"
┌─────────────┐
│             │
│             │
│      ┌─────┐│
│      │  ■  ││
│      └─────┘│
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

pub struct MatrixRain;

impl Animation for MatrixRain {
    fn name(&self) -> &str {
        "Matrix Rain"
    }

    fn frames(&self) -> &[&str] {
        &[
            r"
  1   0     1
    0   1     0
  1       0
      1       1
0       1
    1       0
        0   1
  1   1       ",
            r"
    0   1
  1       0
      1
0       1   1
    1       0
        0   1
  1   1
      0       ",
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

pub struct LoadingSpinner;

impl Animation for LoadingSpinner {
    fn name(&self) -> &str {
        "Loading Spinner"
    }

    fn frames(&self) -> &[&str] {
        &[
            r"
    ╔════════════╗
    ║ ⠋ Loading  ║
    ╚════════════╝",
            r"
    ╔════════════╗
    ║ ⠙ Loading  ║
    ╚════════════╝",
            r"
    ╔════════════╗
    ║ ⠹ Loading  ║
    ╚════════════╝",
            r"
    ╔════════════╗
    ║ ⠸ Loading  ║
    ╚════════════╝",
            r"
    ╔════════════╗
    ║ ⠼ Loading  ║
    ╚════════════╝",
            r"
    ╔════════════╗
    ║ ⠴ Loading  ║
    ╚════════════╝",
            r"
    ╔════════════╗
    ║ ⠦ Loading  ║
    ╚════════════╝",
            r"
    ╔════════════╗
    ║ ⠧ Loading  ║
    ╚════════════╝",
            r"
    ╔════════════╗
    ║ ⠇ Loading  ║
    ╚════════════╝",
            r"
    ╔════════════╗
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

pub struct WavingText;

impl Animation for WavingText {
    fn name(&self) -> &str {
        "Waving Text"
    }

    fn frames(&self) -> &[&str] {
        &[
            r"
    ╔═══════════════════════╗
    ║                       ║
    ║   Schließfach-Manager ║
    ║                       ║
    ╚═══════════════════════╝",
            r"
    ╔═══════════════════════╗
    ║  Schließfach-Manager  ║
    ║                       ║
    ║                       ║
    ╚═══════════════════════╝",
            r"
    ╔═══════════════════════╗
    ║                       ║
    ║  Schließfach-Manager  ║
    ║                       ║
    ╚═══════════════════════╝",
            r"
    ╔═══════════════════════╗
    ║                       ║
    ║                       ║
    ║  Schließfach-Manager  ║
    ╚═══════════════════════╝",
            r"
    ╔═══════════════════════╗
    ║                       ║
    ║  Schließfach-Manager  ║
    ║                       ║
    ╚═══════════════════════╝",
            r"
    ╔═══════════════════════╗
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
    fn spinning_clock_has_frames() {
        let animation = SpinningClock;
        assert!(!animation.frames().is_empty());
        assert_eq!(animation.width(), 2);
    }

    #[test]
    fn animation_player_cycles_frames() {
        let mut player = AnimationPlayer::new(Box::new(SpinningClock));
        let first = player.current_frame().to_string();
        player.update();
        let second = player.current_frame().to_string();
        assert!(!first.is_empty());
        assert!(!second.is_empty());
    }
}
