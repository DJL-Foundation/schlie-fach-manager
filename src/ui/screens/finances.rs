use crate::{db, ui::theme::Theme, ui::widgets::table::TableWidget};
use anyhow::Result;
use chrono::Utc;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Render the finances screen.
pub fn render(
    area: Rect,
    buf: &mut Buffer,
    db: &db::connection::Database,
    theme: &Theme,
) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(1)])
        .split(area);

    render_overview(chunks[0], buf, db, theme)?;
    render_payments(chunks[1], buf, db, theme)?;
    Ok(())
}

/// Render the finance overview summary.
fn render_overview(
    area: Rect,
    buf: &mut Buffer,
    db: &db::connection::Database,
    theme: &Theme,
) -> Result<()> {
    let payments = db::payments::list_payments(db.connection())?;
    let today = Utc::now().date_naive();
    let mut revenue_30d = 0;

    for payment in &payments {
        if payment.payment_date >= today - chrono::Duration::days(30) {
            revenue_30d += payment.amount_cents;
        }
    }

    let lines = vec![
        format!("Ausstehende Zahlungen: {}", format_currency(0)),
        format!("Einnahmen (30 Tage): {}", format_currency(revenue_30d)),
    ];
    Paragraph::new(lines.join("\n"))
        .block(Block::default().borders(Borders::ALL).title("Übersicht"))
        .style(Style::default().fg(theme.text))
        .render(area, buf);
    Ok(())
}

/// Render the payments table.
fn render_payments(
    area: Rect,
    buf: &mut Buffer,
    db: &db::connection::Database,
    theme: &Theme,
) -> Result<()> {
    let payments = db::payments::list_payments(db.connection())?;
    let rentals = db::rentals::list_rentals(db.connection())?;

    let rows = payments
        .iter()
        .map(|payment| {
            let renter = rentals
                .iter()
                .find(|rental| rental.id == payment.rental_id)
                .map(|rental| rental.renter_name.as_str())
                .unwrap_or("-");
            vec![
                payment.payment_date.to_string(),
                renter.to_string(),
                format_currency(payment.amount_cents),
                payment.payment_type.as_str().to_string(),
            ]
        })
        .collect::<Vec<_>>();

    let table = TableWidget {
        title: "Zahlungsverlauf",
        headers: vec!["Datum", "Verleiher", "Betrag", "Typ"],
        rows,
        widths: vec![
            Constraint::Length(12),
            Constraint::Percentage(35),
            Constraint::Length(12),
            Constraint::Percentage(35),
        ],
    };
    table.render(area, buf, theme);
    Ok(())
}

/// Formats currency values in cents to Euro display.
fn format_currency(cents: i64) -> String {
    let euros = cents as f64 / 100.0;
    format!("€{:.2}", euros)
}
