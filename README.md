# Schließfach-Manager

[![CI](https://github.com/DJL-Foundation/schlie-fach-manager/workflows/CI/badge.svg)](https://github.com/DJL-Foundation/schlie-fach-manager/actions)
[![Crates.io](https://img.shields.io/crates/v/schliessfach-manager.svg)](https://crates.io/crates/schliessfach-manager)
[![Documentation](https://docs.rs/schliessfach-manager/badge.svg)](https://docs.rs/schliessfach-manager)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive terminal-based locker management system (Schließfach-Manager) designed for schools and institutions.

![Dashboard Screenshot](docs/images/dashboard.png)

## Features

- 🔐 **Locker Management**: Track lockers across multiple locations with damage status
- 📋 **Rental System**: Manage locker rentals with automatic debt calculation (10€/year overdue)
- 💰 **Payment Tracking**: Record deposits (10€), extensions, and debt payments
- 📊 **Dashboard**: Real-time overview of occupancy, expiring rentals, and outstanding payments
- 📁 **Multi-Format Export/Import**: Export and import data in JSON, TOML, CSV, and Markdown
- 🖥️ **Terminal UI**: Beautiful, responsive TUI built with [ratatui](https://ratatui.rs)
- 💾 **SQLite Storage**: Reliable local database storage
- 🎬 **Screensaver**: Automatic ASCII animation screensaver after inactivity
- ⌨️ **Global Keybinds**: 3× Escape to Dashboard, Window Switcher, and more
- 📝 **Audit Logging**: Full change tracking for compliance

## What's New in v2.1.0

- **3× Escape to Dashboard**: Press Escape three times within 1 second to return to Dashboard from anywhere
- **Escape Indicator `[|||]`**: Visual feedback showing escape counter in status bar
- **Screensaver**: Auto-activates after 60s inactivity with 5 ASCII animations
- **Window Switcher (`^`)**: Quick switching between main screens
- **Import Module**: Full CSV import support for lockers and rentals
- **Comprehensive Documentation**: New KEYBINDINGS.md, USER_GUIDE.md, CHANGELOG.md

## Installation

### From crates.io

```bash
cargo install schliessfach-manager
```

### From source

```bash
git clone https://github.com/DJL-Foundation/schlie-fach-manager.git
cd schlie-fach-manager
cargo build --release
```

### Pre-built binaries

Download pre-built binaries from the [Releases](https://github.com/DJL-Foundation/schlie-fach-manager/releases) page.

| Platform | Architecture | Download |
|----------|-------------|----------|
| Linux | x86_64 | `schliessfach-manager-linux-x86_64.tar.gz` |
| Linux | ARM64 | `schliessfach-manager-linux-aarch64.tar.gz` |
| macOS | x86_64 | `schliessfach-manager-macos-x86_64.tar.gz` |
| macOS | ARM64 (Apple Silicon) | `schliessfach-manager-macos-aarch64.tar.gz` |
| Windows | x86_64 | `schliessfach-manager-windows-x86_64.zip` |

## Usage

### Running the Application

```bash
schliessfach-manager
```

### Keyboard Shortcuts

#### Global Keybinds (always available)

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Navigate between elements |
| `^` (Caret) | Open Window Switcher |
| `Shift+Q` | Quit application |
| `Esc` | Cancel/Go back |
| `Esc` ×3 | **Return to Dashboard** (within 1 second) |
| `Enter` | Confirm/Select |

#### Dashboard Shortcuts

| Key | Action |
|-----|--------|
| `1` | New rental search |
| `2` | Rental list |
| `3` | Extend rental |
| `4` | Return locker |
| `5` | Report damage |

#### Escape Indicator

The status bar shows `[|||]` on the right side:
- Each `|` lights up after pressing Escape
- After 3 escapes within 1 second: jump to Dashboard
- Resets after 1 second or any other key

For complete keybindings documentation, see [docs/KEYBINDINGS.md](docs/KEYBINDINGS.md).

### Screensaver

The screensaver activates automatically after inactivity:

- **Timeout**: 60 seconds (configurable in settings)
- **Countdown**: 15 seconds warning in status bar
- **Animations**: 5 different ASCII animations (random)
- **Exit**: Any key returns to **Dashboard**

### Screens

1. **Dashboard**: Overview of all statistics and quick actions
2. **Rental Management**: Search, list, extend, return lockers
3. **Finance**: Payment summary and debtor tracking
4. **Management**: Locker and location management, backup/restore

## Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
schliessfach-manager = "2.1"
```

### Example

```rust
use schliessfach_manager::models::{Locker, Rental, TenantType};
use schliessfach_manager::db::Database;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a locker
    let locker = Locker::new("A-001", "Hauptgebäude", 150);
    println!("Created locker: {} at {}", locker.label, locker.location);

    // Create a rental
    let today = chrono::Utc::now().date_naive();
    let end_date = today + chrono::Duration::days(365);
    let rental = Rental::new(
        locker.id,
        "max.mustermann",
        TenantType::Schüler,
        today,
        end_date,
    );
    
    println!("Rental for {} expires in {} days", 
        rental.tenant_username, 
        rental.days_until_expiration());

    Ok(())
}
```

## Export Formats

The system supports multiple export formats:

| Format | Export | Import | Use Case |
|--------|--------|--------|----------|
| **TOML** (default) | ✓ | ✓ | Human-readable backup/restore |
| JSON | ✓ | ✓ | Full backup/restore |
| CSV | ✓ | ✓ | Spreadsheet analysis, bulk import |
| Markdown | ✓ | ✗ | Reports and documentation |

### Export Example

```rust
use schliessfach_manager::export::{export_full_backup, export_full_backup_toml};
use std::path::Path;

// Export to TOML (recommended)
export_full_backup_toml(&lockers, &rentals, &payments, &locations, Path::new("backup.toml"))?;

// Export to JSON
export_full_backup(&lockers, &rentals, &payments, &locations, Path::new("backup.json"))?;
```

### Import Example

```rust
use schliessfach_manager::import::{import_full_backup_json, import_lockers_csv, ImportStats};
use schliessfach_manager::db::Database;
use std::path::Path;

let db = Database::open(Path::new("data.db"))?;

// Import from JSON
let stats: ImportStats = import_full_backup_json(&db.conn, Path::new("backup.json"))?;
println!("Imported {} lockers", stats.lockers_imported);

// Import lockers from CSV
let count = import_lockers_csv(&db.conn, Path::new("lockers.csv"))?;
```

## Domain Model

### German Terminology

| German | English | Description |
|--------|---------|-------------|
| Schließfach | Locker | Storage unit |
| Mieter | Tenant | Person renting |
| Schüler | Student | Student tenant type |
| Lehrer | Teacher | Teacher tenant type |
| Kaution | Deposit | Security deposit (10€) |
| Verlängerung | Extension | Rental extension |
| Schulden | Debt | Outstanding payment |

### Pricing

- **Deposit (Kaution)**: 10€
- **Yearly Extension**: 10€ per year
- **Overdue Fee**: 10€ per year (or partial year) overdue

### Debt Calculation

```rust
// Debt accumulates at 10€ per year overdue
let days_overdue = (today - rental.rental_end_date).num_days();
let years_overdue = (days_overdue as f64 / 365.25).ceil() as i32;
let debt = years_overdue * 1000; // cents
```

## Configuration

The database is stored in the user's data directory:

| Platform | Location |
|----------|----------|
| Linux | `~/.local/share/schliessfach-manager/data.db` |
| macOS | `~/Library/Application Support/schliessfach-manager/data.db` |
| Windows | `%APPDATA%\schliessfach-manager\data.db` |

## Development

### Prerequisites

- Rust 1.75 or later
- SQLite development libraries (included via `rusqlite` bundled feature)

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

### CI/CD

The project uses GitHub Actions for:

- **CI**: Build, test, lint (rustfmt, clippy), security audit
- **Release**: Automated publishing to crates.io and GitHub releases

## Documentation

- **[User Guide](docs/USER_GUIDE.md)**: Comprehensive user documentation
- **[Keybindings](docs/KEYBINDINGS.md)**: Complete keyboard shortcuts reference
- **[Changelog](docs/CHANGELOG.md)**: Version history and release notes
- **[API Documentation](https://docs.rs/schliessfach-manager)**: Rust API reference

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [ratatui](https://ratatui.rs) for terminal UI
- Uses [SQLite](https://sqlite.org) via [rusqlite](https://github.com/rusqlite/rusqlite)
- Serialization powered by [serde](https://serde.rs)
