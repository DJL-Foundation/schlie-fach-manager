# Schließfach-Manager v2.0 - Implementierungsspezifikation Teil 2

## 6. Workflows & Business-Logik (Fortsetzung)

### 6.3 Workflow: Zurückgeben (Fortsetzung)

**Schulden-Logik**:
```rust
fn calculate_debt(rental: &Rental) -> i32 {
    let today = Utc::now().date_naive();
    if today <= rental.rental_end_date {
        return 0; // Nicht überfällig
    }
    
    let days_overdue = (today - rental.rental_end_date).num_days();
    let years_overdue = (days_overdue as f64 / 365.25).ceil() as i32;
    
    years_overdue * 1000 // 10€ pro Jahr in Cent
}

fn can_return_locker(debt_cents: i32) -> ReturnAction {
    match debt_cents {
        0 => ReturnAction::AllowedWithDeposit,
        1000 => ReturnAction::AllowedWithoutDeposit, // Genau 10€: Pfand einbehalten
        _ => ReturnAction::RequiresPayment(debt_cents),
    }
}
```

### 6.4 Workflow: Bulk-Erstellung von Schließfächern

```rust
pub struct BulkCreateWorkflow {
    location: String,
    prefix: String,
    start_num: i32,
    end_num: i32,
    height_mode: HeightMode,
}

pub enum HeightMode {
    Auto,           // Gleichmäßig verteilt 0-300cm
    Fixed(i32),     // Feste Höhe für alle
}

impl BulkCreateWorkflow {
    pub fn generate_lockers(&self) -> Vec<Locker> {
        let count = (self.end_num - self.start_num + 1) as usize;
        (self.start_num..=self.end_num)
            .enumerate()
            .map(|(i, num)| {
                let height = match self.height_mode {
                    HeightMode::Auto => {
                        // Gleichmäßige Verteilung
                        ((i as f64 / count as f64) * 300.0) as i32
                    }
                    HeightMode::Fixed(h) => h,
                };
                
                Locker {
                    id: 0, // Auto-increment
                    label: format!("{}{:03}", self.prefix, num),
                    location: self.location.clone(),
                    height,
                    is_damaged: false,
                    created_at: Utc::now(),
                }
            })
            .collect()
    }
}
```

---

## 7. Datenbank-Queries

### 7.1 Dashboard-Statistiken

```rust
// src/db/queries.rs

pub fn get_dashboard_stats(conn: &Connection) -> Result<DashboardStats> {
    // Gesamtzahlen
    let total_lockers: i32 = conn.query_row(
        "SELECT COUNT(*) FROM lockers",
        [],
        |row| row.get(0)
    )?;
    
    let occupied_lockers: i32 = conn.query_row(
        "SELECT COUNT(DISTINCT locker_id) FROM rentals WHERE returned_at IS NULL",
        [],
        |row| row.get(0)
    )?;
    
    let damaged_lockers: i32 = conn.query_row(
        "SELECT COUNT(*) FROM lockers WHERE is_damaged = 1",
        [],
        |row| row.get(0)
    )?;
    
    let damaged_and_occupied: i32 = conn.query_row(
        "SELECT COUNT(DISTINCT l.id) 
         FROM lockers l
         JOIN rentals r ON l.id = r.locker_id
         WHERE l.is_damaged = 1 AND r.returned_at IS NULL",
        [],
        |row| row.get(0)
    )?;
    
    // Standort-Statistiken
    let locations = get_location_stats(conn)?;
    
    // Ablaufende Verträge
    let expiring_soon = get_expiring_rentals(conn, 30)?;
    
    // Überfällige Verträge
    let overdue_rentals = get_overdue_rentals(conn)?;
    
    // Finanzen
    let total_revenue_cents = get_total_revenue(conn)?;
    let outstanding_payments_cents = get_outstanding_payments(conn)?;
    
    Ok(DashboardStats {
        total_lockers,
        occupied_lockers,
        damaged_lockers,
        damaged_and_occupied,
        locations,
        expiring_soon,
        overdue_rentals,
        total_revenue_cents,
        outstanding_payments_cents,
    })
}

pub fn get_location_stats(conn: &Connection) -> Result<Vec<LocationStats>> {
    let mut stmt = conn.prepare(
        "SELECT 
            l.location,
            COUNT(l.id) as total,
            COUNT(CASE WHEN r.returned_at IS NULL THEN 1 END) as occupied,
            SUM(l.is_damaged) as damaged
         FROM lockers l
         LEFT JOIN rentals r ON l.id = r.locker_id AND r.returned_at IS NULL
         GROUP BY l.location
         ORDER BY l.location"
    )?;
    
    let stats = stmt.query_map([], |row| {
        Ok(LocationStats {
            location: row.get(0)?,
            total: row.get(1)?,
            occupied: row.get(2)?,
            damaged: row.get(3)?,
        })
    })?;
    
    stats.collect()
}

pub fn get_expiring_rentals(conn: &Connection, days: i32) -> Result<Vec<RentalWithLocker>> {
    let cutoff_date = (Utc::now() + Duration::days(days as i64))
        .date_naive()
        .to_string();
    
    let mut stmt = conn.prepare(
        "SELECT 
            r.id, r.locker_id, r.tenant_username, r.tenant_type,
            r.rental_start_date, r.rental_end_date, r.deposit_paid,
            r.deposit_returned, r.created_at, r.returned_at,
            l.id, l.label, l.location, l.height, l.is_damaged, l.created_at
         FROM rentals r
         JOIN lockers l ON r.locker_id = l.id
         WHERE r.returned_at IS NULL
         AND r.rental_end_date <= ?
         ORDER BY r.rental_end_date ASC"
    )?;
    
    let rentals = stmt.query_map([cutoff_date], |row| {
        Ok(RentalWithLocker {
            rental: row_to_rental(row, 0)?,
            locker: row_to_locker(row, 10)?,
        })
    })?;
    
    rentals.collect()
}

pub fn get_overdue_rentals(conn: &Connection) -> Result<Vec<RentalWithLocker>> {
    let today = Utc::now().date_naive().to_string();
    
    let mut stmt = conn.prepare(
        "SELECT 
            r.id, r.locker_id, r.tenant_username, r.tenant_type,
            r.rental_start_date, r.rental_end_date, r.deposit_paid,
            r.deposit_returned, r.created_at, r.returned_at,
            l.id, l.label, l.location, l.height, l.is_damaged, l.created_at
         FROM rentals r
         JOIN lockers l ON r.locker_id = l.id
         WHERE r.returned_at IS NULL
         AND r.rental_end_date < ?
         ORDER BY r.rental_end_date ASC"
    )?;
    
    let rentals = stmt.query_map([today], |row| {
        Ok(RentalWithLocker {
            rental: row_to_rental(row, 0)?,
            locker: row_to_locker(row, 10)?,
        })
    })?;
    
    rentals.collect()
}
```

### 7.2 Finanz-Queries

```rust
pub fn get_payment_summary(
    conn: &Connection,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
) -> Result<PaymentSummary> {
    let end = end_date.unwrap_or_else(|| Utc::now().date_naive());
    
    let mut stmt = conn.prepare(
        "SELECT 
            payment_type,
            SUM(amount_cents) as total_cents,
            COUNT(*) as count
         FROM payments
         WHERE payment_date >= ? AND payment_date <= ?
         GROUP BY payment_type"
    )?;
    
    let rows = stmt.query_map([start_date.to_string(), end.to_string()], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i32>(1)?,
            row.get::<_, i32>(2)?,
        ))
    })?;
    
    let mut deposits = 0;
    let mut extensions = 0;
    let mut deposit_returns = 0;
    
    for result in rows {
        let (payment_type, amount, _count) = result?;
        match payment_type.as_str() {
            "Deposit" => deposits += amount,
            "Extension" => extensions += amount,
            "DepositReturn" => deposit_returns += amount,
            _ => {}
        }
    }
    
    Ok(PaymentSummary {
        deposits_cents: deposits,
        extensions_cents: extensions,
        deposit_returns_cents: deposit_returns,
        total_cents: deposits + extensions + deposit_returns,
        net_cents: deposits + extensions + deposit_returns, // deposit_returns ist bereits negativ
    })
}

pub fn get_outstanding_payments(conn: &Connection) -> Result<i32> {
    let today = Utc::now().date_naive().to_string();
    
    // Berechne Schulden für alle überfälligen Verträge
    let mut stmt = conn.prepare(
        "SELECT rental_end_date FROM rentals 
         WHERE returned_at IS NULL AND rental_end_date < ?"
    )?;
    
    let dates = stmt.query_map([today], |row| {
        row.get::<_, String>(0)
    })?;
    
    let mut total_debt = 0;
    for date_result in dates {
        let end_date_str = date_result?;
        let end_date = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d")?;
        let debt = calculate_debt_from_date(end_date);
        total_debt += debt;
    }
    
    Ok(total_debt)
}

fn calculate_debt_from_date(end_date: NaiveDate) -> i32 {
    let today = Utc::now().date_naive();
    if today <= end_date {
        return 0;
    }
    let days_overdue = (today - end_date).num_days();
    let years_overdue = (days_overdue as f64 / 365.25).ceil() as i32;
    years_overdue * 1000
}

pub fn get_debtors(conn: &Connection) -> Result<Vec<DebtorInfo>> {
    let today = Utc::now().date_naive().to_string();
    
    let mut stmt = conn.prepare(
        "SELECT 
            tenant_username,
            tenant_type,
            rental_end_date
         FROM rentals
         WHERE returned_at IS NULL AND rental_end_date < ?
         ORDER BY rental_end_date ASC"
    )?;
    
    let rentals = stmt.query_map([today], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    
    let mut debtors: HashMap<String, (TenantType, i32)> = HashMap::new();
    
    for result in rentals {
        let (username, tenant_type_str, end_date_str) = result?;
        let end_date = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d")?;
        let debt = calculate_debt_from_date(end_date);
        
        let tenant_type = match tenant_type_str.as_str() {
            "Lehrer" => TenantType::Lehrer,
            _ => TenantType::Schüler,
        };
        
        debtors.entry(username)
            .and_modify(|(_, total)| *total += debt)
            .or_insert((tenant_type, debt));
    }
    
    let mut result: Vec<DebtorInfo> = debtors
        .into_iter()
        .map(|(username, (tenant_type, debt))| DebtorInfo {
            username: username.clone(),
            email: format!("{}@athenetz.de", username),
            total_debt_cents: debt,
            tenant_type,
        })
        .collect();
    
    result.sort_by(|a, b| b.total_debt_cents.cmp(&a.total_debt_cents));
    
    Ok(result)
}
```

### 7.3 Historische Daten für Graphen

```rust
pub fn get_occupancy_history(conn: &Connection, months: i32) -> Result<Vec<(NaiveDate, i32)>> {
    // Erstelle monatliche Snapshots der Belegung
    let start_date = Utc::now().date_naive() - Duration::days(months as i64 * 30);
    
    let mut history = Vec::new();
    let mut current = start_date;
    let end = Utc::now().date_naive();
    
    while current <= end {
        let occupied = conn.query_row(
            "SELECT COUNT(DISTINCT locker_id) FROM rentals 
             WHERE rental_start_date <= ? 
             AND (returned_at IS NULL OR returned_at >= ?)",
            [current.to_string(), current.to_string()],
            |row| row.get(0)
        )?;
        
        history.push((current, occupied));
        current = current + Duration::days(30);
    }
    
    Ok(history)
}
```

---

## 8. Export-Funktionalität

### 8.1 JSON-Export

```rust
// src/export/json.rs

pub fn export_full_backup(db: &Database, path: &Path) -> Result<()> {
    let backup = FullBackup {
        version: "2.0".to_string(),
        exported_at: Utc::now(),
        lockers: db.get_all_lockers()?,
        rentals: db.get_all_rentals()?,
        payments: db.get_all_payments()?,
        locations: db.get_all_locations()?,
    };
    
    let json = serde_json::to_string_pretty(&backup)?;
    std::fs::write(path, json)?;
    
    Ok(())
}

pub fn export_finance_overview(
    summary: &PaymentSummary,
    debtors: &[DebtorInfo],
    path: &Path,
) -> Result<()> {
    let export = FinanceExport {
        summary: summary.clone(),
        debtors: debtors.to_vec(),
        exported_at: Utc::now(),
    };
    
    let json = serde_json::to_string_pretty(&export)?;
    std::fs::write(path, json)?;
    
    Ok(())
}
```

### 8.2 CSV-Export

```rust
// src/export/csv.rs

pub fn export_debtors_csv(debtors: &[DebtorInfo], path: &Path) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;
    
    wtr.write_record(&["Email", "Typ", "Schulden_EUR", "Ueberfaellig_Tage"])?;
    
    for debtor in debtors {
        let amount_eur = format!("{:.2}", debtor.total_debt_cents as f64 / 100.0);
        let tenant_type = match debtor.tenant_type {
            TenantType::Schüler => "Schüler",
            TenantType::Lehrer => "Lehrer",
        };
        
        wtr.write_record(&[
            &debtor.email,
            tenant_type,
            &amount_eur,
            &"", // TODO: Tage berechnen
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
}

pub fn export_lockers_csv(lockers: &[Locker], path: &Path) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;
    
    wtr.write_record(&["ID", "Label", "Standort", "Hoehe_cm", "Defekt", "Erstellt"])?;
    
    for locker in lockers {
        wtr.write_record(&[
            &locker.id.to_string(),
            &locker.label,
            &locker.location,
            &locker.height.to_string(),
            &(if locker.is_damaged { "Ja" } else { "Nein" }),
            &locker.created_at.to_rfc3339(),
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
}

pub fn export_active_rentals_csv(rentals: &[RentalWithLocker], path: &Path) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;
    
    wtr.write_record(&[
        "Schliessfach",
        "Standort",
        "Mieter",
        "Typ",
        "Beginn",
        "Ende",
        "Status",
    ])?;
    
    let today = Utc::now().date_naive();
    
    for item in rentals {
        let status = if item.rental.rental_end_date < today {
            "Überfällig"
        } else {
            "Aktiv"
        };
        
        let tenant_type = match item.rental.tenant_type {
            TenantType::Schüler => "Schüler",
            TenantType::Lehrer => "Lehrer",
        };
        
        wtr.write_record(&[
            &item.locker.label,
            &item.locker.location,
            &format!("{}@athenetz.de", item.rental.tenant_username),
            tenant_type,
            &item.rental.rental_start_date.to_string(),
            &item.rental.rental_end_date.to_string(),
            status,
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
}
```

### 8.3 Markdown-Export

```rust
// src/export/markdown.rs

pub fn export_debtors_markdown(debtors: &[DebtorInfo], path: &Path) -> Result<()> {
    let mut content = String::new();
    
    content.push_str("# Schuldentabelle\n\n");
    content.push_str(&format!("Stand: {}\n\n", Utc::now().format("%d.%m.%Y %H:%M")));
    
    content.push_str("| E-Mail | Typ | Schulden | Überfällig seit |\n");
    content.push_str("|--------|-----|----------|------------------|\n");
    
    let mut total = 0;
    for debtor in debtors {
        let amount_eur = debtor.total_debt_cents as f64 / 100.0;
        total += debtor.total_debt_cents;
        
        let tenant_type = match debtor.tenant_type {
            TenantType::Schüler => "Schüler",
            TenantType::Lehrer => "Lehrer",
        };
        
        content.push_str(&format!(
            "| {} | {} | {:.2} € | - |\n",
            debtor.email, tenant_type, amount_eur
        ));
    }
    
    content.push_str("\n");
    content.push_str(&format!(
        "**Gesamt: {} Schuldner, {:.2} € ausstehend**\n",
        debtors.len(),
        total as f64 / 100.0
    ));
    
    std::fs::write(path, content)?;
    Ok(())
}

pub fn export_finance_overview_markdown(
    summary: &PaymentSummary,
    debtors: &[DebtorInfo],
    path: &Path,
) -> Result<()> {
    let mut content = String::new();
    
    content.push_str("# Finanzübersicht\n\n");
    content.push_str(&format!("Stand: {}\n\n", Utc::now().format("%d.%m.%Y %H:%M")));
    
    content.push_str("## Einnahmen\n\n");
    content.push_str(&format!(
        "- Pfand-Einnahmen: {:.2} €\n",
        summary.deposits_cents as f64 / 100.0
    ));
    content.push_str(&format!(
        "- Verlängerungen: {:.2} €\n",
        summary.extensions_cents as f64 / 100.0
    ));
    content.push_str(&format!(
        "- Pfand-Rückgaben: {:.2} €\n",
        summary.deposit_returns_cents as f64 / 100.0
    ));
    content.push_str(&format!(
        "- **Gesamt-Einnahmen: {:.2} €**\n",
        summary.total_cents as f64 / 100.0
    ));
    content.push_str(&format!(
        "- **Netto (nach Pfand): {:.2} €**\n\n",
        summary.net_cents as f64 / 100.0
    ));
    
    content.push_str("## Ausstehende Zahlungen\n\n");
    let student_debtors: Vec<_> = debtors
        .iter()
        .filter(|d| d.tenant_type == TenantType::Schüler)
        .collect();
    let teacher_debtors: Vec<_> = debtors
        .iter()
        .filter(|d| d.tenant_type == TenantType::Lehrer)
        .collect();
    
    let student_debt: i32 = student_debtors.iter().map(|d| d.total_debt_cents).sum();
    let teacher_debt: i32 = teacher_debtors.iter().map(|d| d.total_debt_cents).sum();
    
    content.push_str(&format!(
        "- Schüler: {:.2} € ({} Verleih)\n",
        student_debt as f64 / 100.0,
        student_debtors.len()
    ));
    content.push_str(&format!(
        "- Lehrer: {:.2} € ({} Verleih)\n",
        teacher_debt as f64 / 100.0,
        teacher_debtors.len()
    ));
    content.push_str(&format!(
        "- **Gesamt: {:.2} € ({} Verleih)**\n",
        (student_debt + teacher_debt) as f64 / 100.0,
        debtors.len()
    ));
    
    std::fs::write(path, content)?;
    Ok(())
}
```

---

## 9. Testing-Strategie

### 9.1 Unit-Tests

```rust
// src/models/locker.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_locker_creation() {
        let locker = Locker {
            id: 1,
            label: "A-042".to_string(),
            location: "Hauptgebäude".to_string(),
            height: 100,
            is_damaged: false,
            created_at: Utc::now(),
        };
        
        assert_eq!(locker.label, "A-042");
        assert!(!locker.is_damaged);
    }
    
    #[test]
    fn test_locker_damage_flag() {
        let mut locker = Locker::new("A-042", "Test", 100);
        assert!(!locker.is_damaged);
        
        locker.mark_damaged();
        assert!(locker.is_damaged);
        
        locker.mark_repaired();
        assert!(!locker.is_damaged);
    }
}
```

```rust
// src/workflows/rent_locker.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rent_workflow_state_machine() {
        let mut workflow = RentLockerWorkflow::new();
        
        assert_eq!(workflow.state, RentState::ChooseLocation);
        
        workflow.next(WorkflowInput::Location("Hauptgebäude".to_string()));
        assert_eq!(workflow.state, RentState::ChooseHeight);
        
        workflow.next(WorkflowInput::Height(HeightPreference::Middle));
        assert!(matches!(workflow.state, RentState::SelectLocker { .. }));
    }
    
    #[test]
    fn test_debt_calculation() {
        let rental = Rental {
            rental_end_date: Utc::now().date_naive() - Duration::days(40),
            ..Default::default()
        };
        
        let debt = calculate_debt(&rental);
        assert_eq!(debt, 1000); // 10€ für 1 Jahr überfällig
    }
}
```

### 9.2 Integration-Tests

```rust
// tests/integration_test.rs

use schliessfach_manager::*;

#[test]
fn test_full_rental_workflow() {
    let db = Database::new_in_memory().unwrap();
    db.migrate().unwrap();
    
    // Schließfach anlegen
    let locker = db.create_locker("A-042", "Hauptgebäude", 100).unwrap();
    
    // Verleihen
    let rental = db.create_rental(
        locker.id,
        "max.mustermann",
        TenantType::Schüler,
        Utc::now().date_naive(),
        Utc::now().date_naive() + Duration::days(365),
    ).unwrap();
    
    // Zahlungen erfassen
    db.create_payment(rental.id, 1000, PaymentType::Deposit, Utc::now().date_naive()).unwrap();
    db.create_payment(rental.id, 1000, PaymentType::Extension, Utc::now().date_naive()).unwrap();
    
    // Verlängern
    let new_end_date = rental.rental_end_date + Duration::days(365);
    db.extend_rental(rental.id, new_end_date).unwrap();
    
    // Zurückgeben
    db.return_locker(rental.id, Utc::now()).unwrap();
    
    // Verifizieren
    let returned_rental = db.get_rental(rental.id).unwrap();
    assert!(returned_rental.returned_at.is_some());
}

#[test]
fn test_overdue_calculation() {
    let db = Database::new_in_memory().unwrap();
    db.migrate().unwrap();
    
    let locker = db.create_locker("A-042", "Hauptgebäude", 100).unwrap();
    
    // Verleih mit abgelaufenem Datum
    let rental = db.create_rental(
        locker.id,
        "max.mustermann",
        TenantType::Schüler,
        Utc::now().date_naive() - Duration::days(400),
        Utc::now().date_naive() - Duration::days(35),
    ).unwrap();
    
    let overdue = db.get_overdue_rentals().unwrap();
    assert_eq!(overdue.len(), 1);
    assert_eq!(overdue[0].rental.id, rental.id);
}

#[test]
fn test_dashboard_stats() {
    let db = Database::new_in_memory().unwrap();
    db.migrate().unwrap();
    
    // Setup: 3 Schließfächer, 2 verliehen, 1 defekt
    let l1 = db.create_locker("A-001", "Hauptgebäude", 50).unwrap();
    let l2 = db.create_locker("A-002", "Hauptgebäude", 100).unwrap();
    let l3 = db.create_locker("A-003", "Turnhalle", 150).unwrap();
    
    db.create_rental(l1.id, "user1", TenantType::Schüler, 
                     Utc::now().date_naive(), 
                     Utc::now().date_naive() + Duration::days(365)).unwrap();
    
    db.create_rental(l2.id, "user2", TenantType::Lehrer,
                     Utc::now().date_naive(),
                     Utc::now().date_naive() + Duration::days(365)).unwrap();
    
    db.mark_locker_damaged(l3.id).unwrap();
    
    let stats = db.get_dashboard_stats().unwrap();
    
    assert_eq!(stats.total_lockers, 3);
    assert_eq!(stats.occupied_lockers, 2);
    assert_eq!(stats.damaged_lockers, 1);
    assert_eq!(stats.locations.len(), 2); // Hauptgebäude, Turnhalle
}
```

### 9.3 UI-Tests (mit ratatui TestBackend)

```rust
// tests/ui_test.rs

use ratatui::backend::TestBackend;
use ratatui::Terminal;

#[test]
fn test_dashboard_rendering() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    
    let mut app = App::new_with_test_db();
    app.screen = AppScreen::Dashboard;
    
    terminal.draw(|frame| {
        ui::render(frame, &app);
    }).unwrap();
    
    let buffer = terminal.backend().buffer();
    
    // Verifiziere dass wichtige UI-Elemente vorhanden sind
    assert!(buffer_contains(buffer, "Schließfach-Manager"));
    assert!(buffer_contains(buffer, "Dashboard"));
    assert!(buffer_contains(buffer, "Gesamt:"));
}

fn buffer_contains(buffer: &ratatui::buffer::Buffer, text: &str) -> bool {
    buffer.content().iter().any(|cell| cell.symbol().contains(text))
}
```

---

## 10. Implementierungs-Roadmap

### Phase 1: Kern-Infrastruktur (Woche 1-2)
- [x] Projekt-Setup (Cargo.toml mit Dependencies)
- [ ] Datenbankschema & Migrationen
- [ ] Domain-Modelle (Locker, Rental, Payment)
- [ ] Basis-CRUD-Operationen
- [ ] Config-Management (DB-Pfad)
- [ ] Unit-Tests für Models & DB

### Phase 2: Business-Logik (Woche 3-4)
- [ ] Workflow-State-Machines
  - [ ] RentLockerWorkflow
  - [ ] ExtendRentalWorkflow
  - [ ] ReturnLockerWorkflow
- [ ] Schulden-Berechnung
- [ ] Dashboard-Statistiken
- [ ] Finanz-Queries
- [ ] Unit-Tests für Workflows

### Phase 3: UI-Widgets (Woche 5-6)
- [ ] Chat-Dialog Widget
- [ ] Form Widget (Manual Mode)
- [ ] Confirmation-Dialog
- [ ] Notification Widget
- [ ] Table-With-Detail Widget
- [ ] Searchable-Combobox
- [ ] Export-Dialog

### Phase 4: Screen-Implementierung (Woche 7-10)
- [ ] Dashboard (Screen 1)
  - [ ] Statistik-Boxen
  - [ ] Location-Übersicht
  - [ ] Finanz-Zusammenfassung
  - [ ] ASCII-Graphen (BarChart, Sparkline)
- [ ] Rental Management (Screen 2)
  - [ ] Tab: Verleihen (Chat + Manual)
  - [ ] Tab: Liste (Table + Detail)
  - [ ] Tab: Verlängern (Chat)
  - [ ] Tab: Zurückgeben (Chat)
  - [ ] Tab: Defekt melden
- [ ] Finanzen (Screen 3)
  - [ ] Übersicht
  - [ ] Schuldentabelle
- [ ] Management (Screen 4)
  - [ ] Schließfächer-Verwaltung
  - [ ] Standorte-Verwaltung
  - [ ] Backup/Export/Import

### Phase 5: Export/Import (Woche 11)
- [ ] JSON-Export (Full Backup)
- [ ] CSV-Export (Tabellen)
- [ ] Markdown-Export (Reports)
- [ ] JSON-Import (Restore)
- [ ] CSV-Import (Bulk-Import)

### Phase 6: Polish & Testing (Woche 12-13)
- [ ] Integration-Tests für alle Workflows
- [ ] UI-Tests (TestBackend)
- [ ] Fehlerbehandlung verbessern
- [ ] Deutsche UI-Texte finalisieren
- [ ] Performance-Optimierung
- [ ] Dokumentation (README, API-Docs)

### Phase 7: Deployment (Woche 14)
- [ ] Windows-Build testen
- [ ] Installer/Packaging
- [ ] User-Guide (Markdown)
- [ ] Schulung vorbereiten
- [ ] Release v2.0

---

## 11. Konfiguration & Deployment

### 11.1 Cargo.toml

```toml
[package]
name = "schliessfach-manager"
version = "2.0.0"
edition = "2021"
authors = ["DJL Foundation"]

[dependencies]
ratatui = "0.26"
crossterm = "0.27"
rusqlite = { version = "0.31", features = ["bundled"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
color-eyre = "0.6"
unicode-width = "0.1"
directories = "5.0"
csv = "1.3"
anyhow = "1.0"

[dev-dependencies]
tempfile = "3.10"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### 11.2 Plattform-spezifische Pfade

```rust
// src/config.rs

use directories::ProjectDirs;
use std::path::PathBuf;

pub struct Config {
    pub db_path: PathBuf,
    pub export_path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self> {
        let db_path = if let Ok(custom_path) = std::env::var("SCHLIESSFACH_MANAGER_DB_PATH") {
            PathBuf::from(custom_path)
        } else {
            let proj_dirs = ProjectDirs::from("de", "djl-foundation", "schliessfach-manager")
                .ok_or_else(|| anyhow!("Konnte Projektverzeichnis nicht ermitteln"))?;
            
            let data_dir = proj_dirs.data_dir();
            std::fs::create_dir_all(data_dir)?;
            data_dir.join("schliessfach.db")
        };
        
        let export_path = dirs::download_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join("Downloads")))
            .ok_or_else(|| anyhow!("Konnte Download-Verzeichnis nicht ermitteln"))?;
        
        Ok(Config {
            db_path,
            export_path,
        })
    }
}
```

### 11.3 Cross-Platform-Build

**Windows**:
```bash
# Auf Linux/Mac für Windows kompilieren
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

**Linux**:
```bash
cargo build --release
```

**Installer (Windows - NSIS)**:
```nsis
# installer.nsi
!include "MUI2.nsh"

Name "Schließfach-Manager"
OutFile "schliessfach-manager-setup.exe"
InstallDir "$PROGRAMFILES64\SchliessfachManager"

!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES

Section "Install"
    SetOutPath "$INSTDIR"
    File "target\x86_64-pc-windows-gnu\release\schliessfach-manager.exe"
    CreateShortcut "$DESKTOP\Schließfach-Manager.lnk" "$INSTDIR\schliessfach-manager.exe"
SectionEnd
```

---

## 12. User-Guide (Kurzfassung)

### Erste Schritte

1. **Installation**: Programm ausführen, Datenbank wird automatisch angelegt
2. **Standorte anlegen**: Management → Standorte → [N] Neuer Standort
3. **Schließfächer anlegen**: Management → Schließfächer → [B] Bulk-Erstellung
4. **Erstes Schließfach verleihen**: Rental Management → Verleihen

### Tägliche Nutzung

**Schließfach verleihen**:
- Rental Management → Verleihen
- Chat-Modus folgen oder [M] für manuelles Formular
- Geld-Bestätigung nicht vergessen!

**Verlängerung**:
- Rental Management → Verlängern
- Nummer oder Username eingeben
- Verlängerungsdauer wählen

**Zurückgabe**:
- Rental Management → Zurückgeben
- Bei Schulden: Zahlung bestätigen
- Pfand-Rückgabe bestätigen

**Defekt melden**:
- Rental Management → Defekt melden
- Nummer eingeben
- Optional: Notiz hinzufügen

### Verwaltung

**Finanzen einsehen**:
- Finanzen → Übersicht
- Zeitraum wählen
- [E] für Export

**Schuldner kontaktieren**:
- Finanzen → Schuldentabelle
- [E] Export als CSV für E-Mail-Import

**Backup erstellen**:
- Management → Backup → [1] Vollständiger Export
- Datei wird in Downloads gespeichert

### Tastatur-Shortcuts

**Global**:
- `Tab` / `Shift+Tab`: Zwischen Hauptmenüs wechseln
- `Q`: Programm beenden
- `?` oder `F1`: Hilfe anzeigen

**Navigation**:
- `↑↓` oder `jk`: Navigation in Listen
- `PageUp` / `PageDown`: Schnelle Navigation
- `Home` / `End`: Anfang/Ende

**Aktionen**:
- `Enter`: Bestätigen/Auswählen
- `ESC`: Abbrechen/Zurück
- `/`: Suche starten
- `E`: Bearbeiten
- `D`: Löschen (mit Bestätigung)
- `N`: Neu anlegen

**Chat-Modus**:
- `C`: Zu Chat wechseln
- `M`: Zu Manual wechseln
- `Enter`: Auswahl bestätigen

---

## 13. Zusätzliche Features (Nice-to-Have)

### 13.1 Audit-Log-Viewer

Screen für Audit-Log-Anzeige:
- Filtert nach Entity-Type, Action, Zeitraum
- Zeigt detaillierte Änderungshistorie
- Export als CSV/JSON

### 13.2 Automatische Reminder

Optional: E-Mail-Versand an Schuldner
- Integration mit SMTP
- Templates für Reminder-Mails
- Konfigurierbar in Settings

### 13.3 Multi-User-Support

Optional: Benutzerverwaltung
- Login-Screen
- Berechtigungen (Admin, User, Read-Only)
- Audit-Log mit Username

### 13.4 Statistik-Erweiterungen

- Trend-Analysen (Belegung über Zeit)
- Umsatz-Prognosen
- Standort-Vergleiche
- Export als Diagramme (ASCII-Art oder extern)

### 13.5 Externe Datenquellen

- IServ-API-Integration für Username-Validierung
- Auto-Complete für Benutzernamen
- Automatischer Import von Schülerlisten

---

## 14. Bekannte Einschränkungen & Trade-Offs

### 14.1 Technische Einschränkungen

- **TUI-Framework**: Keine Maus-Unterstützung in allen Terminals
- **SQLite**: Single-User-Database, keine gleichzeitigen Zugriffe
- **Graphen**: Limitiert auf ASCII-Art (keine echten Charts)

### 14.2 Design-Entscheidungen

- **Chat vs. Form**: Chat-Modus für häufige Aktionen, Form für Experten
- **Keine Undo-Funktion**: Audit-Log ermöglicht Nachvollziehbarkeit
- **Manuelle Geld-Bestätigung**: Vertrauen in Benutzer, keine Kassen-Integration

### 14.3 Zukünftige Verbesserungen

- Web-UI als Alternative
- Mobile App für Vor-Ort-Verwaltung
- Cloud-Sync für Multi-Device-Support
- Foto-Upload für Schäden

---

## 15. Anhang: Beispiel-Daten

### Seed-Daten für Tests

```rust
pub fn seed_test_data(db: &Database) -> Result<()> {
    // Standorte
    db.create_location("Hauptgebäude", "Erdgeschoss, Flur West")?;
    db.create_location("Turnhalle", "Eingangsbereich")?;
    db.create_location("Neubau", "1. OG, Südflügel")?;
    
    // Schließfächer
    for i in 1..=60 {
        db.create_locker(
            &format!("A-{:03}", i),
            "Hauptgebäude",
            (i - 1) * 5, // 0-295cm verteilt
        )?;
    }
    
    for i in 1..=50 {
        db.create_locker(
            &format!("B-{:03}", i),
            "Turnhalle",
            (i - 1) * 6,
        )?;
    }
    
    // Beispiel-Verleih
    let locker1 = db.get_locker_by_label("A-001")?;
    let rental1 = db.create_rental(
        locker1.id,
        "max.mustermann",
        TenantType::Schüler,
        Utc::now().date_naive() - Duration::days(100),
        Utc::now().date_naive() + Duration::days(265),
    )?;
    
    db.create_payment(rental1.id, 1000, PaymentType::Deposit, Utc::now().date_naive() - Duration::days(100))?;
    db.create_payment(rental1.id, 1000, PaymentType::Extension, Utc::now().date_naive() - Duration::days(100))?;
    
    // Überfälliger Verleih
    let locker2 = db.get_locker_by_label("A-042")?;
    let rental2 = db.create_rental(
        locker2.id,
        "anna.schmidt",
        TenantType::Schüler,
        Utc::now().date_naive() - Duration::days(450),
        Utc::now().date_naive() - Duration::days(85), // Überfällig!
    )?;
    
    db.create_payment(rental2.id, 1000, PaymentType::Deposit, Utc::now().date_naive() - Duration::days(450))?;
    db.create_payment(rental2.id, 1000, PaymentType::Extension, Utc::now().date_naive() - Duration::days(450))?;
    
    // Defektes Schließfach
    let locker3 = db.get_locker_by_label("B-025")?;
    db.mark_locker_damaged(locker3.id)?;
    
    Ok(())
}
```

---

**Ende der Spezifikation Teil 2**

Fortsetzung siehe `claude45sonnet_spec.md` für vollständige Details zu Datenmodell, UI-Komponenten und Kernarchitektur.