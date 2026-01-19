//! # Schließfach-Manager
//!
//! A comprehensive terminal-based locker management system designed for schools
//! and institutions. This crate provides both a ready-to-use TUI application and
//! a library for building custom locker management solutions.
//!
//! ## Features
//!
//! - **Locker Management**: Track lockers across multiple locations with damage status
//! - **Rental System**: Manage locker rentals with automatic debt calculation
//! - **Payment Tracking**: Record deposits, extensions, and debt payments
//! - **Multi-Format Export**: Export data to JSON, TOML, CSV, and Markdown
//! - **Terminal UI**: Beautiful, responsive TUI built with [ratatui]
//! - **SQLite Storage**: Reliable local database storage
//!
//! ## Quick Start
//!
//! ### Using as a Binary
//!
//! Install from crates.io:
//! ```bash
//! cargo install schliessfach-manager
//! ```
//!
//! Run the application:
//! ```bash
//! schliessfach-manager
//! ```
//!
//! ### Using as a Library
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! schliessfach-manager = "2.0"
//! ```
//!
//! Example usage:
//! ```rust,no_run
//! use schliessfach_manager::models::{Locker, Rental, TenantType};
//! use schliessfach_manager::db::Database;
//!
//! // Create a locker
//! let locker = Locker::new("A-001", "Hauptgebäude", 150);
//! println!("Created locker: {} at {}", locker.label, locker.location);
//! ```
//!
//! ## Architecture
//!
//! The crate is organized into several modules:
//!
//! - [`models`]: Domain models (Locker, Rental, Payment, etc.)
//! - [`db`]: Database layer with SQLite backend
//! - [`export`]: Export functionality (JSON, TOML, CSV, Markdown)
//! - [`workflows`]: State machines for complex operations
//! - [`ui`]: Terminal user interface components
//!
//! ## Domain Model
//!
//! The core domain consists of:
//!
//! ### Lockers
//!
//! Physical storage units that can be rented. Each locker has:
//! - A unique label (e.g., "A-001")
//! - A location (e.g., "Hauptgebäude", "Turnhalle")
//! - A height in centimeters
//! - Damage status
//!
//! ### Rentals
//!
//! Rental contracts linking tenants to lockers. Features:
//! - Tenant information (username, type: Schüler/Lehrer)
//! - Rental period (start and end dates)
//! - Automatic debt calculation (10€/year overdue)
//! - Deposit tracking (10€ standard deposit)
//!
//! ### Payments
//!
//! Financial transactions including:
//! - Deposits (Kaution)
//! - Extensions (Verlängerung)
//! - Deposit returns (Kautionsrückgabe)
//! - Debt payments (Schuldenbegleichung)
//!
//! ## Export Formats
//!
//! The system supports multiple export formats:
//!
//! | Format   | Export | Import | Best For                    |
//! |----------|--------|--------|------------------------------|
//! | JSON     | ✓      | ✓      | Full backup/restore          |
//! | TOML     | ✓      | ✓      | Human-readable configuration |
//! | CSV      | ✓      | ✗      | Spreadsheet analysis         |
//! | Markdown | ✓      | ✗      | Reports and documentation    |
//!
//! ## Examples
//!
//! ### Creating and Managing Lockers
//!
//! ```rust
//! use schliessfach_manager::models::Locker;
//!
//! // Create a new locker
//! let mut locker = Locker::new("B-042", "Turnhalle", 120);
//!
//! // Check locker status
//! assert!(!locker.is_damaged);
//!
//! // Mark as damaged
//! locker.is_damaged = true;
//! ```
//!
//! ### Working with Rentals
//!
//! ```rust
//! use schliessfach_manager::models::{Rental, TenantType};
//! use chrono::{Duration, Utc};
//!
//! // Create a rental for a student
//! let start_date = Utc::now().date_naive();
//! let end_date = start_date + Duration::days(365);
//!
//! let rental = Rental::new(
//!     1,                          // locker_id
//!     "max.mustermann",           // username
//!     TenantType::Schüler,        // tenant type
//!     start_date,
//!     end_date,
//! );
//!
//! // Check days until expiration
//! let days_left = rental.days_until_expiration();
//! println!("Rental expires in {} days", days_left);
//! ```
//!
//! ### Exporting Data
//!
//! ```rust,no_run
//! use schliessfach_manager::export::{export_full_backup, export_full_backup_toml};
//! use schliessfach_manager::models::Locker;
//! use std::path::Path;
//!
//! let lockers = vec![Locker::new("A-001", "Main", 100)];
//!
//! // Export to JSON
//! export_full_backup(&lockers, &[], &[], &[], Path::new("backup.json")).unwrap();
//!
//! // Export to TOML (human-readable)
//! export_full_backup_toml(&lockers, &[], &[], &[], Path::new("backup.toml")).unwrap();
//! ```
//!
//! ## German Domain Terminology
//!
//! This system uses German terminology for its domain model:
//!
//! | German | English | Description |
//! |--------|---------|-------------|
//! | Schließfach | Locker | Storage unit |
//! | Mieter | Tenant | Person renting |
//! | Schüler | Student | Student tenant type |
//! | Lehrer | Teacher | Teacher tenant type |
//! | Kaution | Deposit | Security deposit (10€) |
//! | Verlängerung | Extension | Rental extension |
//! | Schulden | Debt | Outstanding payment |
//!
//! ## Configuration
//!
//! The application stores its database in the user's data directory:
//! - Linux: `~/.local/share/schliessfach-manager/`
//! - macOS: `~/Library/Application Support/schliessfach-manager/`
//! - Windows: `%APPDATA%\schliessfach-manager\`
//!
//! ## Security Considerations
//!
//! - Data is stored locally in SQLite (no network transmission)
//! - No personal data beyond usernames is stored
//! - Email addresses are generated from usernames (not stored)
//!
//! ## License
//!
//! This project is licensed under the MIT License.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

pub mod app;
pub mod config;
pub mod db;
pub mod export;
pub mod import;
pub mod models;
pub mod screensaver;
pub mod ui;
pub mod workflows;

// Re-export commonly used types at crate root for convenience
pub use models::{Locker, Payment, PaymentType, Rental, TenantType};
pub use db::Database;
