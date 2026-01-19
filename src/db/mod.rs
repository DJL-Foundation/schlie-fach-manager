//! Database layer for the Schließfach-Manager system.
//!
//! This module provides all database functionality using SQLite as the backend.
//! It includes:
//!
//! - [`connection`]: Database connection management
//! - [`lockers`]: CRUD operations for lockers
//! - [`rentals`]: CRUD operations for rentals
//! - [`payments`]: CRUD operations for payments
//! - [`queries`]: Complex queries for statistics and reports
//! - [`migrations`]: Database schema migrations
//! - [`seed`]: Test data seeding
//!
//! # Database Schema
//!
//! The database consists of five main tables:
//!
//! ```text
//! ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
//! │   lockers   │◄────│   rentals   │────►│  payments   │
//! └─────────────┘     └─────────────┘     └─────────────┘
//!                            │
//!                            ▼
//!                     ┌─────────────┐
//!                     │  audit_log  │
//!                     └─────────────┘
//! ```
//!
//! # Example
//!
//! ```rust,no_run
//! use schliessfach_manager::db::Database;
//!
//! // Open or create the database
//! let db = Database::open_default().expect("Failed to open database");
//!
//! // Database is now ready to use
//! ```
//!
//! # Data Storage Location
//!
//! By default, the database is stored in the user's data directory:
//!
//! - **Linux**: `~/.local/share/schliessfach-manager/data.db`
//! - **macOS**: `~/Library/Application Support/schliessfach-manager/data.db`
//! - **Windows**: `%APPDATA%\schliessfach-manager\data.db`

pub mod connection;
pub mod lockers;
pub mod migrations;
pub mod payments;
pub mod queries;
pub mod rentals;
pub mod seed;

pub use connection::Database;
