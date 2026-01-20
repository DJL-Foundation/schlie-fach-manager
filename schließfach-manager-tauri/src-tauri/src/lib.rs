//! Schließfach-Manager v2.1-Tauri
//! 
//! A modern desktop application for locker management.

pub mod commands;
pub mod db;
pub mod services;
pub mod error;

pub use error::{AppError, Result};
