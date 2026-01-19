//! Schließfach-Manager v2.1-Tauri
//! 
//! Main entry point for the Tauri application.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod services;
mod error;

use db::connection::{establish_connection, DbConnection};
use db::migrations::run_migrations;
use std::sync::Arc;
use tokio::sync::Mutex;

fn main() {
    // Establish database connection
    let conn = establish_connection().expect("Failed to establish database connection");
    
    // Run migrations
    run_migrations(&conn).expect("Failed to run database migrations");
    
    // Create thread-safe connection wrapper
    let db = DbConnection(Arc::new(Mutex::new(conn)));
    
    // Build and run Tauri app
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            // Dashboard commands
            commands::dashboard::get_dashboard_data,
            commands::dashboard::get_status_bar_data,
            // Locker commands
            commands::lockers::get_all_lockers,
            commands::lockers::get_available_lockers,
            commands::lockers::get_locker,
            commands::lockers::create_locker,
            commands::lockers::update_locker,
            commands::lockers::delete_locker,
            // Rental commands
            commands::rentals::get_all_rentals,
            commands::rentals::get_active_rentals,
            commands::rentals::get_overdue_rentals,
            commands::rentals::get_rental,
            commands::rentals::create_rental,
            commands::rentals::extend_rental,
            commands::rentals::return_rental,
            // Settings commands
            commands::settings::get_settings,
            commands::settings::update_setting,
            commands::settings::get_audit_log,
            commands::settings::run_database_migrations,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
