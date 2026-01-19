use crate::export::ExportData;
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Imports export data from a JSON file.
pub fn import(path: &Path) -> Result<ExportData> {
    let content = fs::read_to_string(path)?;
    let data = serde_json::from_str(&content)?;
    Ok(data)
}
