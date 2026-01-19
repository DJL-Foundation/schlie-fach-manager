use crate::ui::theme::Theme;
use crate::ui::widgets::{Keybind, KeybindScope};
use color_eyre::eyre::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Widget},
};
use rusqlite::Connection;
use std::collections::HashMap;

/// Dashboard data loaded from the database
/// Based on v2.1 spec section 6.4
#[derive(Debug, Clone)]
pub struct DashboardData {
    pub total_lockers: usize,
    pub occupied_lockers: usize,
    pub occupancy_percent: f64,
    
    pub by_size: HashMap<String, (usize, usize)>, // size -> (occupied, total)
    pub by_location: HashMap<String, (usize, usize)>, // location -> (occupied, total)
    
    pub overdue_returns: usize,
    pub expiring_soon: usize,  // within 30 days
    pub damaged_lockers: usize,
    
    pub locations: Vec<LocationSummary>,
    pub pending_payments_cents: i64,
    pub revenue_30d_cents: i64,
    
    pub occupancy_history: Vec<(String, f64)>, // (month label, percent)
}

/// Summary information for a location
#[derive(Debug, Clone)]
pub struct LocationSummary {
    pub name: String,
    pub occupied: usize,
    pub total: usize,
}

impl DashboardData {
    /// Load dashboard data from the database
    pub fn load(conn: &Connection) -> Result<Self> {
        // Total lockers
        let total_lockers: usize = conn.query_row(
            "SELECT COUNT(*) FROM lockers",
            [],
            |row| row.get::<_, i64>(0).map(|n| n as usize),
        )?;
        
        // Currently occupied lockers (active rentals)
        let occupied_lockers: usize = conn.query_row(
            "SELECT COUNT(DISTINCT locker_id) FROM rentals 
             WHERE date('now') BETWEEN start_date AND end_date",
            [],
            |row| row.get::<_, i64>(0).map(|n| n as usize),
        )?;
        
        // Calculate occupancy percentage
        let occupancy_percent = if total_lockers > 0 {
            (occupied_lockers as f64 / total_lockers as f64) * 100.0
        } else {
            0.0
        };
        
        // Breakdown by size
        // TODO: Query actual data from rentals JOIN lockers
        // NOTE: This mock data is temporary and will not match the real total/occupied counts
        // Once the rental system is fully implemented, replace with proper aggregation queries
        let mut by_size = HashMap::new();
        by_size.insert("Klein".to_string(), (5, 20));
        by_size.insert("Mittel".to_string(), (8, 15));
        by_size.insert("Groß".to_string(), (3, 10));
        
        // Breakdown by location
        // TODO: Query actual data from rentals JOIN lockers
        // NOTE: This mock data is temporary and will not match the real total/occupied counts
        // Once the rental system is fully implemented, replace with proper aggregation queries
        let mut by_location = HashMap::new();
        by_location.insert("Hauptgebäude".to_string(), (12, 30));
        by_location.insert("Nebengebäude".to_string(), (3, 10));
        by_location.insert("Keller".to_string(), (1, 5));
        
        // Overdue returns (end_date < today and rental still active)
        let overdue_returns: usize = conn.query_row(
            "SELECT COUNT(*) FROM rentals 
             WHERE date('now') > end_date AND deposit_returned = 0",
            [],
            |row| row.get::<_, i64>(0).map(|n| n as usize),
        )?;
        
        // Expiring soon (within 30 days)
        let expiring_soon: usize = conn.query_row(
            "SELECT COUNT(*) FROM rentals 
             WHERE date('now') <= end_date 
             AND date('now', '+30 days') >= end_date 
             AND deposit_returned = 0",
            [],
            |row| row.get::<_, i64>(0).map(|n| n as usize),
        )?;
        
        // Damaged lockers
        let damaged_lockers: usize = conn.query_row(
            "SELECT COUNT(*) FROM lockers WHERE is_damaged = 1",
            [],
            |row| row.get::<_, i64>(0).map(|n| n as usize),
        )?;
        
        // Location summaries
        // TODO: Query actual data with proper aggregation
        let locations = vec![
            LocationSummary {
                name: "Hauptgebäude".to_string(),
                occupied: 12,
                total: 30,
            },
            LocationSummary {
                name: "Nebengebäude".to_string(),
                occupied: 3,
                total: 10,
            },
            LocationSummary {
                name: "Keller".to_string(),
                occupied: 1,
                total: 5,
            },
        ];
        
        // Pending payments
        // TODO: Implement proper payment tracking when rental system is complete
        let pending_payments_cents: i64 = 0;
        
        // Revenue last 30 days
        // COALESCE ensures this never returns NULL, so we can safely unwrap
        let revenue_30d_cents: i64 = conn.query_row(
            "SELECT COALESCE(SUM(amount_cents), 0) FROM payments 
             WHERE payment_date >= date('now', '-30 days')",
            [],
            |row| row.get(0),
        )?;
        
        // Occupancy history (last 12 months)
        let occupancy_history = Self::load_occupancy_history(conn)?;
        
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
    
    /// Load 12-month occupancy history
    fn load_occupancy_history(conn: &Connection) -> Result<Vec<(String, f64)>> {
        // Query the last 12 months of history from occupancy_history table
        let mut stmt = conn.prepare(
            "SELECT snapshot_date, occupancy_percent 
             FROM occupancy_history 
             WHERE snapshot_date >= date('now', '-12 months')
             ORDER BY snapshot_date ASC"
        )?;
        
        let mut history = Vec::new();
        let rows = stmt.query_map([], |row| {
            let date: String = row.get(0)?;
            let percent: f64 = row.get(1)?;
            Ok((date, percent))
        })?;
        
        for row in rows {
            let (date, percent) = row?;
            // Convert date to month label (e.g., "Jan 24")
            let label = Self::format_month_label(&date);
            history.push((label, percent));
        }
        
        // If no history, generate sample data
        if history.is_empty() {
            history = Self::generate_sample_history();
        }
        
        Ok(history)
    }
    
    /// Format date string to month label
    fn format_month_label(date: &str) -> String {
        // Parse YYYY-MM-DD and convert to "Mon YY" format
        let parts: Vec<&str> = date.split('-').collect();
        if parts.len() >= 2 {
            let year = parts[0].chars().skip(2).take(2).collect::<String>();
            let month_num = parts[1].parse::<u32>().unwrap_or(1);
            let month_abbr = match month_num {
                1 => "Jan", 2 => "Feb", 3 => "Mär", 4 => "Apr",
                5 => "Mai", 6 => "Jun", 7 => "Jul", 8 => "Aug",
                9 => "Sep", 10 => "Okt", 11 => "Nov", 12 => "Dez",
                _ => "???"
            };
            format!("{} {}", month_abbr, year)
        } else {
            date.to_string()
        }
    }
    
    /// Generate sample history for demo purposes
    fn generate_sample_history() -> Vec<(String, f64)> {
        vec![
            ("Jan 24".to_string(), 45.0),
            ("Feb 24".to_string(), 52.0),
            ("Mär 24".to_string(), 58.0),
            ("Apr 24".to_string(), 61.0),
            ("Mai 24".to_string(), 65.0),
            ("Jun 24".to_string(), 70.0),
            ("Jul 24".to_string(), 68.0),
            ("Aug 24".to_string(), 72.0),
            ("Sep 24".to_string(), 75.0),
            ("Okt 24".to_string(), 71.0),
            ("Nov 24".to_string(), 69.0),
            ("Dez 24".to_string(), 66.0),
        ]
    }
}

/// Format currency in cents to EUR string
pub fn format_currency(cents: i64) -> String {
    let is_negative = cents < 0;
    let abs_cents = cents.abs();
    let euros = abs_cents / 100;
    let cent_part = abs_cents % 100;
    
    if is_negative {
        format!("-{},{:02}€", euros, cent_part)
    } else {
        format!("{},{:02}€", euros, cent_part)
    }
}

/// Main dashboard screen
/// Based on v2.1 spec section 6
#[derive(Debug)]
pub struct DashboardScreen {
    data: Option<DashboardData>,
}

impl DashboardScreen {
    pub fn new() -> Self {
        Self { data: None }
    }
    
    /// Load data from database
    pub fn load_data(&mut self, conn: &Connection) -> Result<()> {
        self.data = Some(DashboardData::load(conn)?);
        Ok(())
    }
    
    /// Get dashboard-specific keybinds
    pub fn get_keybinds() -> Vec<Keybind> {
        vec![
            Keybind {
                key: "1".to_string(),
                description: "Suchen".to_string(),
                scope: KeybindScope::Context,
            },
            Keybind {
                key: "2".to_string(),
                description: "Liste".to_string(),
                scope: KeybindScope::Context,
            },
            Keybind {
                key: "3".to_string(),
                description: "Verlängern".to_string(),
                scope: KeybindScope::Context,
            },
            Keybind {
                key: "4".to_string(),
                description: "Rückgabe".to_string(),
                scope: KeybindScope::Context,
            },
            Keybind {
                key: "5".to_string(),
                description: "Defekt melden".to_string(),
                scope: KeybindScope::Context,
            },
        ]
    }
    
    /// Render the dashboard screen
    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let data = match &self.data {
            Some(d) => d,
            None => {
                // Render loading state
                self.render_loading(area, buf, theme);
                return;
            }
        };
        
        // Create fixed-height layout according to spec section 6.2
        let chunks = Self::create_layout(area);
        
        // Render each section
        self.render_occupancy(chunks[0], buf, theme, data);
        self.render_actions(chunks[1], buf, theme, data);
        self.render_locations(chunks[2], buf, theme, data);
        self.render_finances(chunks[3], buf, theme, data);
        self.render_trend(chunks[4], buf, theme, data);
    }
    
    /// Create the fixed-height layout
    /// Based on v2.1 spec section 6.2
    fn create_layout(area: Rect) -> Vec<Rect> {
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(9),     // Belegung
                Constraint::Min(6),        // Aktionen (wächst)
                Constraint::Length(7),     // Standorte + Finanzen row
                Constraint::Min(8),        // Trend (wächst)
            ])
            .split(area);
        
        // Split Standorte + Finanzen horizontally
        let middle_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(main_chunks[2]);
        
        vec![
            main_chunks[0], // Belegung
            main_chunks[1], // Aktionen
            middle_row[0],  // Standorte
            middle_row[1],  // Finanzen
            main_chunks[3], // Trend
        ]
    }
    
    /// Render loading state
    fn render_loading(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title("Dashboard");
        
        let text = Paragraph::new("Lade Daten...")
            .alignment(Alignment::Center)
            .block(block);
        
        text.render(area, buf);
    }
    
    /// Render occupancy section (Belegung)
    fn render_occupancy(&self, area: Rect, buf: &mut Buffer, theme: &Theme, data: &DashboardData) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title("📊 Belegung");
        
        let inner = block.inner(area);
        block.render(area, buf);
        
        // Split into sections
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Summary line
                Constraint::Length(2), // Progress bar
                Constraint::Min(3),    // Breakdown
            ])
            .split(inner);
        
        // Summary line
        let summary = format!(
            "Gesamt: {} / {} ({:.1}% belegt)",
            data.occupied_lockers, data.total_lockers, data.occupancy_percent
        );
        let summary_para = Paragraph::new(summary)
            .style(Style::default().fg(theme.text).add_modifier(Modifier::BOLD));
        summary_para.render(chunks[0], buf);
        
        // Progress bar
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(theme.accent).bg(theme.background))
            .ratio(data.occupancy_percent / 100.0)
            .label(format!("{:.1}%", data.occupancy_percent));
        gauge.render(chunks[1], buf);
        
        // Breakdown by size and location
        let breakdown_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(chunks[2]);
        
        // By size
        let mut size_lines = vec![Line::from(
            Span::styled("Nach Größe:", Style::default().fg(theme.text_dim))
        )];
        for (size, (occupied, total)) in &data.by_size {
            let percent = if *total > 0 {
                (*occupied as f64 / *total as f64) * 100.0
            } else {
                0.0
            };
            size_lines.push(Line::from(format!(
                "  {}: {}/{} ({:.0}%)",
                size, occupied, total, percent
            )));
        }
        let size_para = Paragraph::new(size_lines);
        size_para.render(breakdown_chunks[0], buf);
        
        // By location
        let mut loc_lines = vec![Line::from(
            Span::styled("Nach Standort:", Style::default().fg(theme.text_dim))
        )];
        for (location, (occupied, total)) in &data.by_location {
            let percent = if *total > 0 {
                (*occupied as f64 / *total as f64) * 100.0
            } else {
                0.0
            };
            loc_lines.push(Line::from(format!(
                "  {}: {}/{} ({:.0}%)",
                location, occupied, total, percent
            )));
        }
        let loc_para = Paragraph::new(loc_lines);
        loc_para.render(breakdown_chunks[1], buf);
    }
    
    /// Render actions section (Aktionen erforderlich)
    fn render_actions(&self, area: Rect, buf: &mut Buffer, theme: &Theme, data: &DashboardData) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title("⚠ Aktionen erforderlich");
        
        let inner = block.inner(area);
        block.render(area, buf);
        
        // Create list of action items
        let items = vec![
            ListItem::new(Line::from(vec![
                Span::styled("Überfällige Rückgaben: ", Style::default().fg(theme.text)),
                Span::styled(
                    data.overdue_returns.to_string(),
                    if data.overdue_returns > 0 {
                        Style::default().fg(theme.error).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme.success)
                    },
                ),
            ])),
            ListItem::new(Line::from(vec![
                Span::styled("Auslaufende Verträge (30 Tage): ", Style::default().fg(theme.text)),
                Span::styled(
                    data.expiring_soon.to_string(),
                    if data.expiring_soon > 0 {
                        Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme.success)
                    },
                ),
            ])),
            ListItem::new(Line::from(vec![
                Span::styled("Defekte Schließfächer: ", Style::default().fg(theme.text)),
                Span::styled(
                    data.damaged_lockers.to_string(),
                    if data.damaged_lockers > 0 {
                        Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme.success)
                    },
                ),
            ])),
        ];
        
        let list = List::new(items)
            .style(Style::default().fg(theme.text));
        list.render(inner, buf);
    }
    
    /// Render locations section (Standorte)
    fn render_locations(&self, area: Rect, buf: &mut Buffer, theme: &Theme, data: &DashboardData) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title("📍 Standorte");
        
        let inner = block.inner(area);
        block.render(area, buf);
        
        // Create list of locations
        let items: Vec<ListItem> = data.locations.iter().map(|loc| {
            let percent = if loc.total > 0 {
                (loc.occupied as f64 / loc.total as f64) * 100.0
            } else {
                0.0
            };
            ListItem::new(format!(
                "{}: {}/{} ({:.0}%)",
                loc.name, loc.occupied, loc.total, percent
            ))
        }).collect();
        
        let list = List::new(items)
            .style(Style::default().fg(theme.text));
        list.render(inner, buf);
    }
    
    /// Render finances section (Finanzen)
    fn render_finances(&self, area: Rect, buf: &mut Buffer, theme: &Theme, data: &DashboardData) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title("💰 Finanzen");
        
        let inner = block.inner(area);
        block.render(area, buf);
        
        // Create financial summary
        let lines = vec![
            Line::from(vec![
                Span::styled("Ausstehende Zahlungen: ", Style::default().fg(theme.text)),
                Span::styled(
                    format_currency(data.pending_payments_cents),
                    if data.pending_payments_cents > 0 {
                        Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme.success)
                    },
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Einnahmen (30 Tage): ", Style::default().fg(theme.text)),
                Span::styled(
                    format_currency(data.revenue_30d_cents),
                    Style::default().fg(theme.success).add_modifier(Modifier::BOLD),
                ),
            ]),
        ];
        
        let para = Paragraph::new(lines);
        para.render(inner, buf);
    }
    
    /// Render trend section (Belegungsverlauf)
    fn render_trend(&self, area: Rect, buf: &mut Buffer, theme: &Theme, data: &DashboardData) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title("📈 Belegungsverlauf (12 Monate)");
        
        let inner = block.inner(area);
        block.render(area, buf);
        
        if data.occupancy_history.is_empty() {
            let text = Paragraph::new("Keine Verlaufsdaten verfügbar")
                .alignment(Alignment::Center);
            text.render(inner, buf);
            return;
        }
        
        // Use simplified bar chart rendering
        // Convert history data to bars
        let bar_data: Vec<(&str, u64)> = data.occupancy_history.iter()
            .map(|(label, percent)| (label.as_str(), *percent as u64))
            .collect();
        
        use ratatui::widgets::BarChart;
        
        let chart = BarChart::default()
            .data(&bar_data)
            .bar_width(4)
            .bar_gap(1)
            .bar_style(Style::default().fg(theme.accent))
            .value_style(Style::default().fg(theme.text_dim))
            .label_style(Style::default().fg(theme.text_dim))
            .max(100); // Max percentage is 100
        
        chart.render(inner, buf);
    }
}

impl Default for DashboardScreen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_currency_positive() {
        assert_eq!(format_currency(1000), "10,00€");
        assert_eq!(format_currency(1050), "10,50€");
        assert_eq!(format_currency(99), "0,99€");
        assert_eq!(format_currency(0), "0,00€");
    }
    
    #[test]
    fn test_format_currency_negative() {
        assert_eq!(format_currency(-1000), "-10,00€");
        assert_eq!(format_currency(-1050), "-10,50€");
        assert_eq!(format_currency(-99), "-0,99€");
    }
    
    #[test]
    fn test_format_currency_large_amounts() {
        assert_eq!(format_currency(100000), "1000,00€");
        assert_eq!(format_currency(123456), "1234,56€");
        assert_eq!(format_currency(-123456), "-1234,56€");
    }
}
