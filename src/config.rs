use color_eyre::eyre::{eyre, Result};
use directories::ProjectDirs;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "DJLFoundation";
const APPLICATION: &str = "SchliessfachManager";
const DB_PATH_ENV: &str = "SCHLIESSFACH_MANAGER_DB_PATH";

fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .ok_or_else(|| eyre!("unable to resolve platform-specific configuration directories"))
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
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}
