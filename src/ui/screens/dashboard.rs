use crate::{
    config::AppSettings,
    db,
    model::{LocationSummary, PaymentType},
    ui::{theme::Theme, widgets::graph::OccupancyGraph},
};
use anyhow::Result;
use chrono::{Duration, Utc};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};
use std::collections::{HashMap, HashSet};

/// Data model for the dashboard screen.
pub struct DashboardData {
    pub total_lockers: usize,
    pub occupied_lockers: usize,
    pub occupancy_percent: f64,
    pub by_size: HashMap<String, (usize, usize)>,
    pub by_location: HashMap<String, (usize, usize)>,
    pub overdue_returns: usize,
    pub expiring_soon: usize,
    pub damaged_lockers: usize,
    pub locations: Vec<LocationSummary>,
    pub pending_payments_cents: i64,
    pub revenue_30d_cents: i64,
    pub occupancy_history: Vec<(String, f64)>,
}

impl DashboardData {
    /// Loads dashboard metrics from the database.
    pub fn load(conn: &db::connection::Database, settings: &AppSettings) -> Result<Self> {
        let lockers = db::lockers::list_lockers(conn.connection())?;
        let rentals = db::rentals::list_rentals(conn.connection())?;
        let payments = db::payments::list_payments(conn.connection())?;
        let total_lockers = lockers.len();

        let today = Utc::now().date_naive();
        let active_rentals: HashMap<i64, _> = rentals
            .iter()
            .filter(|rental| rental.is_active_on(today))
            .map(|rental| (rental.locker_id, rental))
            .collect();

        let occupied_lockers = active_rentals.len();
        let occupancy_percent = if total_lockers == 0 {
            0.0
        } else {
            (occupied_lockers as f64 / total_lockers as f64) * 100.0
        };

        let mut by_size: HashMap<String, (usize, usize)> = HashMap::new();
        let mut by_location: HashMap<String, (usize, usize)> = HashMap::new();
        let mut location_totals: HashMap<String, LocationSummary> = HashMap::new();

        for locker in &lockers {
            let occupied = active_rentals.contains_key(&locker.id);
            let size_entry = by_size.entry(locker.size.clone()).or_insert((0, 0));
            size_entry.1 += 1;
            if occupied {
                size_entry.0 += 1;
            }

            let location_entry = by_location.entry(locker.location.clone()).or_insert((0, 0));
            location_entry.1 += 1;
            if occupied {
                location_entry.0 += 1;
            }

            let summary =
                location_totals
                    .entry(locker.location.clone())
                    .or_insert(LocationSummary {
                        name: locker.location.clone(),
                        occupied: 0,
                        total: 0,
                    });
            summary.total += 1;
            if occupied {
                summary.occupied += 1;
            }
        }

        let overdue_returns = rentals
            .iter()
            .filter(|rental| rental.end_date < today && !rental.deposit_returned)
            .count();
        let expiring_soon = rentals
            .iter()
            .filter(|rental| {
                rental.end_date >= today && rental.end_date <= (today + Duration::days(30))
            })
            .count();
        let damaged_lockers = lockers.iter().filter(|locker| locker.is_damaged).count();

        let mut pending_payments_cents = 0;
        let mut revenue_30d_cents = 0;
        let mut fee_paid: HashSet<i64> = HashSet::new();

        for payment in &payments {
            if payment.payment_date >= today - Duration::days(30) {
                revenue_30d_cents += payment.amount_cents;
            }
            if matches!(payment.payment_type, PaymentType::YearlyFee) {
                fee_paid.insert(payment.rental_id);
            }
        }

        for rental in &rentals {
            if !rental.deposit_paid {
                pending_payments_cents += settings.deposit_cents;
            }
            let days_active = (today - rental.start_date).num_days();
            let period_days = match settings.billing_period {
                crate::model::BillingPeriod::Monthly => 30,
                crate::model::BillingPeriod::Yearly => 365,
            };
            if days_active >= period_days && !fee_paid.contains(&rental.id) {
                pending_payments_cents += settings.yearly_fee_cents;
            }
        }

        let occupancy_history =
            db::history::get_occupancy_history(conn.connection(), total_lockers)?;

        let mut locations: Vec<LocationSummary> = location_totals.into_values().collect();
        locations.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Self {
            total_lockers,
            occupied_lockers,
            occupancy_percent,
            by_size,
            by_location,
            overdue_returns,
            expiring_soon,
            damaged_lockers,
            locations,
            pending_payments_cents,
            revenue_30d_cents,
            occupancy_history,
        })
    }
}

/// Layout helper for the dashboard main content.
pub fn layout_dashboard(area: Rect) -> Vec<Rect> {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),
            Constraint::Min(6),
            Constraint::Length(7),
            Constraint::Min(8),
        ])
        .split(area);

    let middle_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[2]);

    vec![
        main_chunks[0],
        main_chunks[1],
        middle_row[0],
        middle_row[1],
        main_chunks[3],
    ]
}

/// Render the dashboard main content.
pub fn render(area: Rect, buf: &mut Buffer, data: &DashboardData, theme: &Theme) {
    let chunks = layout_dashboard(area);

    render_occupancy(chunks[0], buf, data, theme);
    render_actions(chunks[1], buf, data, theme);
    render_locations(chunks[2], buf, data, theme);
    render_finances(chunks[3], buf, data, theme);
    render_trend(chunks[4], buf, data, theme);
}

/// Render the occupancy card.
fn render_occupancy(area: Rect, buf: &mut Buffer, data: &DashboardData, theme: &Theme) {
    let percent = format!("{:.1}%", data.occupancy_percent);
    let progress = progress_bar(data.occupancy_percent, 20);
    let lines = vec![
        Line::from(format!(
            "Gesamt: {} / {} ({})",
            data.occupied_lockers, data.total_lockers, percent
        )),
        Line::from(progress),
        Line::from("Nach Größe:"),
    ];
    let mut occupancy_lines = lines;
    let mut sizes: Vec<_> = data.by_size.iter().collect();
    sizes.sort_by(|a, b| a.0.cmp(b.0));
    for (size, (occupied, total)) in sizes {
        occupancy_lines.push(Line::from(format!("  {}: {}/{}", size, occupied, total)));
    }
    occupancy_lines.push(Line::from("Nach Standort:"));
    let mut locations: Vec<_> = data.by_location.iter().collect();
    locations.sort_by(|a, b| a.0.cmp(b.0));
    for (location, (occupied, total)) in locations {
        occupancy_lines.push(Line::from(format!(
            "  {}: {}/{}",
            location, occupied, total
        )));
    }

    Paragraph::new(occupancy_lines)
        .block(Block::default().borders(Borders::ALL).title("Belegung"))
        .style(Style::default().fg(theme.text))
        .render(area, buf);
}

/// Render the actions-required card.
fn render_actions(area: Rect, buf: &mut Buffer, data: &DashboardData, theme: &Theme) {
    let lines = vec![
        Line::from(format!("Überfällige Rückgaben: {}", data.overdue_returns)),
        Line::from(format!(
            "Auslaufende Verträge (30 Tage): {}",
            data.expiring_soon
        )),
        Line::from(format!("Defekte Schließfächer: {}", data.damaged_lockers)),
    ];
    Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Aktionen erforderlich"),
        )
        .style(Style::default().fg(theme.text))
        .render(area, buf);
}

/// Render the locations summary card.
fn render_locations(area: Rect, buf: &mut Buffer, data: &DashboardData, theme: &Theme) {
    let mut lines = Vec::new();
    for summary in &data.locations {
        lines.push(Line::from(format!(
            "{}: {}/{}",
            summary.name, summary.occupied, summary.total
        )));
    }
    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Standorte"))
        .style(Style::default().fg(theme.text))
        .render(area, buf);
}

/// Render the finance summary card.
fn render_finances(area: Rect, buf: &mut Buffer, data: &DashboardData, theme: &Theme) {
    let pending = format_currency(data.pending_payments_cents);
    let revenue = format_currency(data.revenue_30d_cents);
    let lines = vec![
        Line::from(format!("Ausstehende Zahlungen: {}", pending)),
        Line::from(format!("Einnahmen (30 Tage): {}", revenue)),
    ];
    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Finanzen"))
        .style(Style::default().fg(theme.text))
        .render(area, buf);
}

/// Render the occupancy trend graph.
fn render_trend(area: Rect, buf: &mut Buffer, data: &DashboardData, theme: &Theme) {
    let graph = OccupancyGraph {
        data: data.occupancy_history.clone(),
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Trend / Belegungsverlauf");
    let inner = block.inner(area);
    block.render(area, buf);
    graph.render(inner, buf, theme);
}

/// Formats currency values in cents to Euro display.
fn format_currency(cents: i64) -> String {
    let euros = cents as f64 / 100.0;
    format!("€{:.2}", euros)
}

/// Builds a simple text-based progress bar.
fn progress_bar(percent: f64, width: usize) -> String {
    let filled = ((percent / 100.0) * width as f64).round() as usize;
    let filled = filled.min(width);
    let empty = width.saturating_sub(filled);
    format!(
        "[{}{}] {:.1}%",
        "█".repeat(filled),
        "░".repeat(empty),
        percent
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use chrono::NaiveDate;

    #[test]
    fn pending_payments_include_missing_deposit() -> Result<()> {
        let db = db::connection::Database::open_in_memory()?;
        let locker = crate::model::Locker::new(1, "A-01", "Hauptgebäude", "Klein");
        db::lockers::upsert_locker(db.connection(), &locker)?;
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let rental = crate::model::Rental::new(1, locker.id, "Max", start, end);
        db::rentals::upsert_rental(db.connection(), &rental)?;

        let settings = AppSettings::default();
        let data = DashboardData::load(&db, &settings)?;
        assert!(data.pending_payments_cents >= settings.deposit_cents);
        Ok(())
    }
}
