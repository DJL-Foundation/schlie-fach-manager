//! Export and import functionality for locker management data.
//!
//! This module provides functions to export and import data in various formats:
//!
//! - **JSON**: Full backup/restore with all data types
//! - **TOML**: Human-readable backup format, excellent for manual editing
//! - **CSV**: Tabular data export for spreadsheet compatibility
//! - **Markdown**: Formatted reports for documentation and printing
//!
//! # Supported Formats
//!
//! | Format   | Export | Import | Use Case                           |
//! |----------|--------|--------|-------------------------------------|
//! | JSON     | ✓      | ✓      | Full backup/restore                 |
//! | TOML     | ✓      | ✓      | Human-readable configuration        |
//! | CSV      | ✓      | ✗      | Spreadsheet export                  |
//! | Markdown | ✓      | ✗      | Reports and documentation           |
//!
//! # Example
//!
//! ```rust,no_run
//! use schliessfach_manager::export::{
//!     export_full_backup, import_full_backup,
//!     export_full_backup_toml, import_full_backup_toml,
//! };
//! use std::path::Path;
//!
//! // Export to JSON
//! export_full_backup(&[], &[], &[], &[], Path::new("backup.json")).unwrap();
//!
//! // Export to TOML
//! export_full_backup_toml(&[], &[], &[], &[], Path::new("backup.toml")).unwrap();
//! ```

pub mod csv;
pub mod json;
pub mod markdown;
pub mod toml;

pub use self::csv::{export_active_rentals_csv, export_debtors_csv, export_lockers_csv};
pub use self::json::{export_full_backup, import_full_backup};
pub use self::markdown::{export_debtors_markdown, export_finance_overview_markdown};
pub use self::toml::{
    export_full_backup_toml, export_lockers_toml, export_rentals_toml, import_full_backup_toml,
};
