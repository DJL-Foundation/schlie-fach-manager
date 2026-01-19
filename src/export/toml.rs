use crate::export::ExportData;
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Writes export data to a TOML file.
pub fn export(path: &Path, data: &ExportData) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(data)?;
    fs::write(path, content)?;
    Ok(())
}
