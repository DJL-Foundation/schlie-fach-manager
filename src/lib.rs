pub mod config;
pub mod db;
pub mod model;
pub mod screensaver;

// Re-export commonly used types
pub use db::Database;
pub use model::{Locker, LockerStatus};
