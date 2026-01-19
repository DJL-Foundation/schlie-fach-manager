use crate::{db, model::Locker};
use anyhow::Result;

/// Creates multiple lockers with a sequential numbering scheme.
pub fn create_bulk_lockers(
    db: &db::connection::Database,
    start_id: i64,
    prefix: &str,
    count: usize,
    location: &str,
    size: &str,
) -> Result<Vec<Locker>> {
    let mut created = Vec::new();
    for idx in 0..count {
        let id = start_id + idx as i64;
        let number = format!("{}-{:03}", prefix, idx + 1);
        let locker = Locker::new(id, number, location.to_string(), size.to_string());
        db::lockers::upsert_locker(db.connection(), &locker)?;
        created.push(locker);
    }
    Ok(created)
}
