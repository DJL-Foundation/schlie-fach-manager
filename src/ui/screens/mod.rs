// Screen modules will be implemented in future phases
// For now, this is a placeholder to allow compilation

pub mod dashboard;
pub mod screensaver;

pub use dashboard::{DashboardScreen, format_currency};
pub use screensaver::ScreensaverScreen;
