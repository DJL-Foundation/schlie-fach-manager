use crate::model::BillingPeriod;
use anyhow::{Result, anyhow};
use directories::ProjectDirs;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "DJLFoundation";
const APPLICATION: &str = "SchliessfachManager";
const DB_PATH_ENV: &str = "SCHLIESSFACH_MANAGER_DB_PATH";
const EXPORT_DIR_ENV: &str = "SCHLIESSFACH_MANAGER_EXPORT_DIR";

/// Resolves platform-specific project directories.
fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .ok_or_else(|| anyhow!("unable to resolve platform-specific configuration directories"))
}

/// Returns (and creates if necessary) the platform-specific directory used to store all
/// persistent data for the Schließfach-Manager application.
pub fn data_directory() -> Result<PathBuf> {
    let dirs = project_dirs()?;
    let path = dirs.data_dir();
    fs::create_dir_all(path)?;
    Ok(path.to_path_buf())
}

/// Resolves the full path to the SQLite database file with the given `filename`.
/// The parent directory is created automatically so the caller can immediately
/// attempt to open the file.
pub fn database_path(filename: &str) -> Result<PathBuf> {
    if let Ok(override_path) = env::var(DB_PATH_ENV) {
        let path = PathBuf::from(override_path);
        ensure_parent_exists(&path)?;
        return Ok(path);
    }

    let mut path = data_directory()?;
    path.push(filename);
    Ok(path)
}

fn ensure_parent_exists(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// Resolves the export directory for backups and reports.
pub fn export_directory() -> Result<PathBuf> {
    if let Ok(override_path) = env::var(EXPORT_DIR_ENV) {
        let path = PathBuf::from(override_path);
        ensure_parent_exists(&path)?;
        return Ok(path);
    }

    let mut path = data_directory()?;
    path.push("exports");
    fs::create_dir_all(&path)?;
    Ok(path)
}

/// Application settings stored in the database `settings` table.
#[derive(Debug, Clone)]
pub struct AppSettings {
    pub deposit_cents: i64,
    pub yearly_fee_cents: i64,
    pub billing_period: BillingPeriod,
    pub currency: String,
    pub screensaver_timeout_seconds: u64,
    pub export_format: String,
    pub export_directory: PathBuf,
}

impl Default for AppSettings {
    fn default() -> Self {
        let export_directory = export_directory().unwrap_or_else(|_| PathBuf::from("./exports"));
        Self {
            deposit_cents: 1_000,
            yearly_fee_cents: 1_000,
            billing_period: BillingPeriod::Yearly,
            currency: "EUR".to_string(),
            screensaver_timeout_seconds: 60,
            export_format: "toml".to_string(),
            export_directory,
        }
    }
}

impl AppSettings {
    /// Applies a setting value retrieved from the database.
    pub fn apply_setting(&mut self, key: &str, value: &str) {
        match key {
            "deposit_cents" => {
                if let Ok(parsed) = value.parse::<i64>() {
                    self.deposit_cents = parsed;
                }
            }
            "yearly_fee_cents" => {
                if let Ok(parsed) = value.parse::<i64>() {
                    self.yearly_fee_cents = parsed;
                }
            }
            "billing_period" => {
                self.billing_period = BillingPeriod::from_str(value);
            }
            "currency" => {
                if !value.trim().is_empty() {
                    self.currency = value.to_string();
                }
            }
            "screensaver_timeout_seconds" => {
                if let Ok(parsed) = value.parse::<u64>() {
                    self.screensaver_timeout_seconds = parsed;
                }
            }
            "export_format" => {
                if !value.trim().is_empty() {
                    self.export_format = value.to_string();
                }
            }
            "export_directory" => {
                if !value.trim().is_empty() {
                    self.export_directory = PathBuf::from(value);
                }
            }
            _ => {}
        }
    }

    /// Converts the settings into key-value-description tuples for persistence.
    pub fn as_settings_entries(&self) -> Vec<(String, String, String)> {
        vec![
            (
                "deposit_cents".to_string(),
                self.deposit_cents.to_string(),
                "Pfandbetrag in Cents (Standard: 10.00€)".to_string(),
            ),
            (
                "yearly_fee_cents".to_string(),
                self.yearly_fee_cents.to_string(),
                "Jahresgebühr in Cents (Standard: 10.00€)".to_string(),
            ),
            (
                "billing_period".to_string(),
                self.billing_period.as_str().to_string(),
                "Berechnungszeitraum: monthly oder yearly".to_string(),
            ),
            (
                "currency".to_string(),
                self.currency.clone(),
                "Währung (ISO 4217 Code)".to_string(),
            ),
            (
                "screensaver_timeout_seconds".to_string(),
                self.screensaver_timeout_seconds.to_string(),
                "Sekunden Inaktivität bis Screensaver (Standard: 60)".to_string(),
            ),
            (
                "export_format".to_string(),
                self.export_format.clone(),
                "Standard-Exportformat (toml, json, csv)".to_string(),
            ),
            (
                "export_directory".to_string(),
                self.export_directory.to_string_lossy().to_string(),
                "Export-Verzeichnis".to_string(),
            ),
            (
                "app_version".to_string(),
                "2.1.0".to_string(),
                "Anwendungsversion".to_string(),
            ),
        ]
    }
}
