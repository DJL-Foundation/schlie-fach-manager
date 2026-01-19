//! Import functionality for locker management data.
//!
//! This module provides functions to import data from various formats:
//!
//! - **JSON**: Full backup restoration from JSON files
//! - **TOML**: Human-readable backup restoration
//! - **CSV**: Tabular data import for lockers and rentals
//!
//! # Supported Formats
//!
//! | Format | Import | Use Case                           |
//! |--------|--------|-------------------------------------|
//! | JSON   | ✓      | Full backup restoration             |
//! | TOML   | ✓      | Human-readable configuration import |
//! | CSV    | ✓      | Spreadsheet data import             |
//!
//! # Example
//!
//! ```rust,no_run
//! use schliessfach_manager::import::{
//!     import_full_backup_json, import_full_backup_toml,
//!     import_lockers_csv, import_rentals_csv,
//! };
//! use schliessfach_manager::db::Database;
//! use std::path::Path;
//!
//! // Open database connection
//! let db = Database::open(Path::new("data.db")).unwrap();
//!
//! // Import from JSON
//! let stats = import_full_backup_json(&db.conn, Path::new("backup.json")).unwrap();
//! println!("Imported {} lockers", stats.lockers_imported);
//!
//! // Import from TOML
//! let stats = import_full_backup_toml(&db.conn, Path::new("backup.toml")).unwrap();
//!
//! // Import lockers from CSV
//! let count = import_lockers_csv(&db.conn, Path::new("lockers.csv")).unwrap();
//! println!("Imported {} lockers", count);
//! ```

pub mod csv;
pub mod json;
pub mod toml;

pub use self::csv::{import_lockers_csv, import_rentals_csv};
pub use self::json::{import_full_backup_json, ImportStats};
pub use self::toml::import_full_backup_toml;
