//! Database module for Schließfach-Manager
//! 
//! Contains connection handling, migrations, models, and queries.

pub mod connection;
pub mod migrations;
pub mod models;

pub use connection::DbConnection;
pub use models::*;
