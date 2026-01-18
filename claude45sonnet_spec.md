# Schließfach-Manager v2.0 - Vollständige Implementierungsspezifikation

## 1. Projektübersicht

### 1.1 Zielsetzung
Ein vollständiges TUI-basiertes Verwaltungssystem für Schulschließfächer mit umfassendem Funktionsumfang für Verleih, Finanzverwaltung, Wartung und Reporting.

### 1.2 Technologie-Stack
- **Sprache**: Rust (Edition 2021)
- **TUI Framework**: ratatui 0.26+
- **Terminal**: crossterm
- **Datenbank**: SQLite (rusqlite mit bundled feature)
- **Zusätzliche Crates**:
  - `serde` + `serde_json` - Serialisierung/Export
  - `chrono` - Datums-/Zeitverwaltung
  - `color-eyre` - Fehlerbehandlung
  - `unicode-width` - Textbreiten-Berechnungen
  - `directories` - Plattformübergreifende Datenpfade
  - `csv` - CSV-Export
  - `anyhow` - Vereinfachte Fehlerbehandlung

### 1.3 Architektur-Prinzipien
- **Modularer Aufbau**: Klare Trennung von UI, Business-Logik und Datenschicht
- **State Machine Pattern**: Explizite App-States für verschiedene Screens/Modi
- **Event-Driven**: Asynchrone Event-Behandlung für responsive UI
- **Testbarkeit**: Unit-Tests für alle Kernkomponenten, Integration-Tests für Workflows

---

## 2. Datenmodell

### 2.1 Datenbankschema

#### Tabelle: `lockers`
```sql
CREATE TABLE IF NOT EXISTS lockers (
    id INTEGER PRIMARY KEY,
    label TEXT NOT NULL UNIQUE,          -- z.B. "A-101", "B-042"
    location TEXT NOT NULL,               -- Standort: "Hauptgebäude", "Turnhalle", etc.
    height INTEGER NOT NULL,              -- Höhe vom Boden in cm (0-300)
    is_damaged BOOLEAN DEFAULT 0,         -- Wartungsstatus
    created_at TEXT NOT NULL              -- ISO 8601 timestamp
);
```

#### Tabelle: `rentals`
```sql
CREATE TABLE IF NOT EXISTS rentals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    locker_id INTEGER NOT NULL,
    tenant_username TEXT NOT NULL,        -- IServ Username (ohne @athenetz.de)
    tenant_type TEXT NOT NULL,            -- "Schüler" oder "Lehrer"
    rental_start_date TEXT NOT NULL,      -- ISO 8601 date
    rental_end_date TEXT NOT NULL,        -- ISO 8601 date (initial: start + 1 Jahr)
    deposit_paid BOOLEAN DEFAULT 0,       -- 10€ Pfand erhalten
    deposit_returned BOOLEAN DEFAULT 0,   -- 10€ Pfand zurückgegeben
    created_at TEXT NOT NULL,
    returned_at TEXT,                     -- NULL = aktiv verliehen
    FOREIGN KEY (locker_id) REFERENCES lockers(id) ON DELETE CASCADE
);
```

#### Tabelle: `payments`
```sql
CREATE TABLE IF NOT EXISTS payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rental_id INTEGER NOT NULL,
    amount_cents INTEGER NOT NULL,        -- Betrag in Cent (1000 = 10€)
    payment_type TEXT NOT NULL,           -- "Deposit", "Extension", "DepositReturn"
    payment_date TEXT NOT NULL,           -- ISO 8601 date
    notes TEXT,
    FOREIGN KEY (rental_id) REFERENCES rentals(id) ON DELETE CASCADE
);
```

#### Tabelle: `locations`
```sql
CREATE TABLE IF NOT EXISTS locations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TEXT NOT NULL
);
```

#### Tabelle: `audit_log`
```sql
CREATE TABLE IF NOT EXISTS audit_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    action TEXT NOT NULL,                 -- "locker_created", "rental_started", etc.
    entity_type TEXT NOT NULL,            -- "locker", "rental", "payment"
    entity_id INTEGER,
    details TEXT,                         -- JSON mit zusätzlichen Infos
    username TEXT                         -- Optional: wer die Aktion durchgeführt hat
);
```

### 2.2 Rust-Domain-Modelle

```rust
// src/models/locker.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Locker {
    pub id: i64,
    pub label: String,
    pub location: String,
    pub height: i32,
    pub is_damaged: bool,
    pub created_at: DateTime<Utc>,
}

// src/models/rental.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rental {
    pub id: i64,
    pub locker_id: i64,
    pub tenant_username: String,
    pub tenant_type: TenantType,
    pub rental_start_date: NaiveDate,
    pub rental_end_date: NaiveDate,
    pub deposit_paid: bool,
    pub deposit_returned: bool,
    pub created_at: DateTime<Utc>,
    pub returned_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TenantType {
    Schüler,
    Lehrer,
}

// src/models/payment.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: i64,
    pub rental_id: i64,
    pub amount_cents: i32,
    pub payment_type: PaymentType,
    pub payment_date: NaiveDate,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentType {
    Deposit,        // 10€ Pfand
    Extension,      // 10€ pro Jahr
    DepositReturn,  // -10€ (Pfand zurück)
}
```

### 2.3 Aggregierte Daten-Strukturen

```rust
// Für Dashboard und Reports
#[derive(Debug, Clone)]
pub struct DashboardStats {
    pub total_lockers: i32,
    pub occupied_lockers: i32,
    pub damaged_lockers: i32,
    pub damaged_and_occupied: i32,
    pub locations: Vec<LocationStats>,
    pub expiring_soon: Vec<RentalWithLocker>, // < 30 Tage
    pub overdue_rentals: Vec<RentalWithLocker>,
    pub total_revenue_cents: i32,
    pub outstanding_payments_cents: i32,
}

#[derive(Debug, Clone)]
pub struct LocationStats {
    pub location: String,
    pub total: i32,
    pub occupied: i32,
    pub damaged: i32,
}

#[derive(Debug, Clone)]
pub struct RentalWithLocker {
    pub rental: Rental,
    pub locker: Locker,
}

#[derive(Debug, Clone)]
pub struct DebtorInfo {
    pub username: String,
    pub email: String,
    pub total_debt_cents: i32,
    pub tenant_type: TenantType,
}
```

---

## 3. Anwendungsarchitektur

### 3.1 Modul-Struktur

```
src/
├── main.rs                      # Entry point, Event-Loop
├── app.rs                       # Haupt-App-State-Maschine
├── config.rs                    # Konfiguration, DB-Pfad
├── models/
│   ├── mod.rs
│   ├── locker.rs
│   ├── rental.rs
│   ├── payment.rs
│   └── stats.rs                 # DashboardStats, etc.
├── db/
│   ├── mod.rs
│   ├── connection.rs            # Database wrapper
│   ├── migrations.rs            # Schema-Migrationen
│   ├── lockers.rs               # Locker CRUD
│   ├── rentals.rs               # Rental CRUD
│   ├── payments.rs              # Payment CRUD
│   ├── queries.rs               # Komplexe Queries (Stats, Joins)
│   └── seed.rs                  # Test-Daten
├── ui/
│   ├── mod.rs
│   ├── theme.rs                 # Farben, Styles
│   ├── screens/
│   │   ├── mod.rs
│   │   ├── dashboard.rs         # Screen 1: Startseite
│   │   ├── rental_search.rs     # Screen 2.1: Verleihen
│   │   ├── rental_list.rs       # Screen 2.2: Schließfächer-Liste
│   │   ├── rental_extend.rs     # Screen 2.3: Verlängern
│   │   ├── rental_return.rs     # Screen 2.4: Zurückgeben
│   │   ├── damage_report.rs     # Screen 2.5: Defekt melden
│   │   ├── finance_overview.rs  # Screen 3: Finanzen
│   │   ├── finance_debtors.rs   # Screen 3 Sub: Schuldentabelle
│   │   └── management.rs        # Screen 4: Management
│   ├── widgets/
│   │   ├── mod.rs
│   │   ├── chat_dialog.rs       # Interaktiver Dialog (Chat-Style)
│   │   ├── form_dialog.rs       # Manuelles Formular
│   │   ├── confirmation.rs      # Bestätigungs-Popup
│   │   ├── notification.rs      # Temporäre Benachrichtigung
│   │   ├── table_with_detail.rs # Tabelle + Detail-Panel
│   │   ├── searchable_list.rs   # Dropdown mit Suche
│   │   └── export_dialog.rs     # Export-Format-Auswahl
│   └── state.rs                 # UI-State (aktueller Screen, etc.)
├── workflows/
│   ├── mod.rs
│   ├── rent_locker.rs           # Workflow: Schließfach verleihen
│   ├── extend_rental.rs         # Workflow: Verlängern
│   ├── return_locker.rs         # Workflow: Zurückgeben
│   └── create_lockers.rs        # Workflow: Bulk-Erstellung
├── export/
│   ├── mod.rs
│   ├── markdown.rs              # Markdown-Export
│   ├── csv.rs                   # CSV-Export
│   └── json.rs                  # JSON-Export
└── tests/
    ├── integration.rs
    └── fixtures/
        └── test_data.rs
```

### 3.2 App-State-Machine

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppScreen {
    Dashboard,
    RentalManagement(RentalManagementTab),
    Finance(FinanceTab),
    Management(ManagementTab),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RentalManagementTab {
    Search,      // 2.1: Verleihen
    List,        // 2.2: Liste
    Extend,      // 2.3: Verlängern
    Return,      // 2.4: Zurückgeben
    Damage,      // 2.5: Defekt melden
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinanceTab {
    Overview,    // Übersicht
    Debtors,     // Schuldentabelle
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManagementTab {
    Lockers,     // Schließfächer verwalten
    Locations,   // Standorte verwalten
    Backup,      // Export/Import
}

pub struct App {
    pub screen: AppScreen,
    pub db: Database,
    // Screen-spezifische States
    pub dashboard_state: DashboardState,
    pub rental_search_state: RentalSearchState,
    pub rental_list_state: RentalListState,
    // ... weitere States
    pub notification: Option<Notification>,
    pub should_quit: bool,
}
```

---

## 4. Hauptmenüs & Screens

### 4.1 Screen 1: Dashboard (Startseite)

#### Funktionalität
- **Übersicht** über aktuelle Kennzahlen
- **Graphische Darstellung** (ASCII-Grafiken mit ratatui BarChart/Sparkline)
- **Quick Actions**: Shortcuts zu häufigen Aktionen

#### Layout
```
┌────────────────────────────────────────────────────────────────────────────┐
│ Schließfach-Manager v2.0                                    [Dashboard]    │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  ┌─ Belegung ─────────────────┐  ┌─ Standorte ────────────────────────┐  │
│  │ Gesamt: 150                │  │ Hauptgebäude:    45/60  [███▒▒]    │  │
│  │ Belegt: 112 (75%)          │  │ Turnhalle:       35/50  [████▒]    │  │
│  │ Frei:    38 (25%)          │  │ Neubau:          32/40  [████▒]    │  │
│  │ Defekt:   8 (5%)           │  └────────────────────────────────────┘  │
│  │   davon belegt: 3          │                                           │
│  │                            │  ┌─ Finanzen ─────────────────────────┐  │
│  │ [████████████████▒▒▒▒▒▒▒]  │  │ Einnahmen (Monat):     1.240,00 €  │  │
│  └────────────────────────────┘  │ Einnahmen (Jahr):     14.880,00 €  │  │
│                                   │ Ausstehend:              320,00 €  │  │
│  ┌─ Aktionen erforderlich ────┐  │ Mögliche Einnahmen:      380,00 €  │  │
│  │ • 5 Verlängerungen überfällig│ └────────────────────────────────────┘  │
│  │ • 8 Schließfächer defekt   │                                           │
│  │ • 12 Verträge laufen in    │  ┌─ Graphen ──────────────────────────┐  │
│  │   30 Tagen aus             │  │ Belegung (12 Monate):              │  │
│  └────────────────────────────┘  │ ▆▇██████████▇▆                      │  │
│                                   └────────────────────────────────────┘  │
├────────────────────────────────────────────────────────────────────────────┤
│ [1] Verleihen | [2] Liste | [3] Verlängern | [4] Zurückgeben | [Q] Beenden│
└────────────────────────────────────────────────────────────────────────────┘
```

#### Datenquellen
- `DashboardStats` aus `db::queries::get_dashboard_stats()`
- Zeitreihen-Daten für Graphen: `get_occupancy_history(months: i32)`

#### Navigation
- **Zahlen 1-4**: Direkte Shortcuts zu Untermenüs
- **Pfeiltasten**: Navigation zwischen Bereichen
- **Enter**: Aktionen aufrufen (z.B. bei "5 Verlängerungen überfällig")
- **Q**: Beenden
- **Tab**: Wechsel zu nächstem Hauptmenü (Rental Management)

---

### 4.2 Screen 2: Rental Management

Gemeinsamer Header mit Tabs:
```
┌────────────────────────────────────────────────────────────────────────────┐
│ Schließfach-Manager                           [Verleih-Verwaltung]        │
│ ┌──────────┬──────────┬────────────┬──────────────┬───────────────┐       │
│ │ Verleihen│  Liste  │ Verlängern │ Zurückgeben │ Defekt melden │       │
│ └──────────┴──────────┴────────────┴──────────────┴───────────────┘       │
├────────────────────────────────────────────────────────────────────────────┤
```

#### 4.2.1 Tab: Verleihen (Search)

**Modi**: Chat (C) / Manual (M)

##### Chat-Modus (Standard)
Interaktiver Dialog im Stil von CLI-Prompts:

```
┌─ Neuer Verleih ─────────────────────────────────────────────────────────┐
│                                                                          │
│  > Bevorzugter Standort?                                                │
│    [Hauptgebäude] [Turnhalle] [Neubau]                                  │
│                                                                          │
│  ✓ Hauptgebäude ausgewählt                                              │
│                                                                          │
│  > Bevorzugte Höhe?                                                     │
│    [Unten (0-50cm)] [Mitte (50-150cm)] [Oben (150-300cm)] [Egal]       │
│                                                                          │
│  ✓ Mitte ausgewählt                                                     │
│                                                                          │
│  > Passende Schließfächer gefunden:                                     │
│    • A-042 (Hauptgebäude, 100cm)                                        │
│    • A-043 (Hauptgebäude, 105cm)                                        │
│    • A-051 (Hauptgebäude, 120cm)                                        │
│                                                                          │
│    [Enter] für A-042 | [↓/↑] andere auswählen | [ESC] abbrechen        │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

**Workflow**:
1. Standort-Auswahl (Dropdown/Buttons)
2. Höhe-Auswahl (Buttons: Unten/Mitte/Oben/Egal)
3. System schlägt passende freie Schließfächer vor
4. Benutzer wählt Schließfach aus
5. **Bestätigungs-Dialog** mit allen Schließfach-Details
6. Eingabe: IServ-Username (mit Hinweis "@athenetz.de" rechts im Input)
7. Auswahl: Tenant-Type (Schüler/Lehrer)
8. **Manual-View** öffnet sich zur finalen Bearbeitung
9. **Zahlungs-Dialog**: "Hast du 20€ erhalten?" (Subtitle: 10€ Pfand + 10€ Jahr)
10. **Erinnerungs-Dialog**: "Hast du an jährliche Erneuerung erinnert?"
11. Speichern → Navigation zu Rental List mit diesem Eintrag fokussiert
12. **Notification**: "Verleih erfolgreich angelegt! (ID: 123)" (10 Sek., Enter zum Schließen)

##### Manual-Modus (M-Taste)
Formular mit allen Feldern direkt editierbar:

```
┌─ Manueller Verleih ─────────────────────────────────────────────────────┐
│                                                                          │
│  Schließfach-ID: [___________▼]  <-- Combobox mit Suche                │
│                                                                          │
│  Benutzername:   [____________]@athenetz.de                             │
│                                                                          │
│  Typ:            ( ) Schüler  (•) Lehrer                                │
│                                                                          │
│  Beginn:         [2024-01-15] [📅]                                      │
│                                                                          │
│  Ende:           [2025-01-15] [📅] (Standard: +1 Jahr)                  │
│                                                                          │
│  Pfand erhalten: [✓] Ja  [ ] Nein                                       │
│                                                                          │
│  Notizen:        [_________________________________]                    │
│                  [_________________________________]                    │
│                                                                          │
│  [Enter] Speichern | [ESC] Abbrechen | [C] Zurück zu Chat              │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

#### 4.2.2 Tab: Liste

Tabelle + Detail-View (Split-Screen):

```
┌─ Schließfächer ─────────────────────────────────────────────────────────┐
│ Suche: [____________]                           Status: [Alle ▼] [↻]    │
├──────────────────────────────────────┬──────────────────────────────────┤
│ ID    │ Standort      │ Verleih │... │ ┌─ Details: A-042 ─────────────┐│
│ A-042 │ Hauptgebäude  │ max.mu  │    │ │                              ││
│►A-043 │ Hauptgebäude  │ -       │    │ │ Label: A-043                 ││
│ A-044 │ Hauptgebäude  │ anna.s  │    │ │ Standort: Hauptgebäude       ││
│ A-045 │ Hauptgebäude  │ -       │    │ │ Höhe: 105 cm                 ││
│ A-051 │ Hauptgebäude  │ peter.k │    │ │ Status: Frei                 ││
│ A-052 │ Hauptgebäude  │ -       │    │ │ Defekt: Nein                 ││
│ ...                                  │ │                              ││
│                                      │ │ ┌─ Aktueller Verleih ───────┐││
│ 35/150 Einträge (Filter: Alle)      │ │ │ Kein aktiver Verleih      │││
│                                      │ │ └───────────────────────────┘││
│                                      │ │                              ││
│                                      │ │ [E] Bearbeiten               ││
│                                      │ │ [C] Neuer Verleih            ││
│                                      │ │ [D] Verleih löschen          ││
│                                      │ └──────────────────────────────┘│
├──────────────────────────────────────┴──────────────────────────────────┤
│ [/] Suchen | [↑↓] Navigation | [E] Edit | [C] Neu | [D] Löschen | [ESC]│
└──────────────────────────────────────────────────────────────────────────┘
```

**Funktionen**:
- **Suche** (`/`): Filter nach ID, Username, Standort
- **Status-Filter**: Alle / Belegt / Frei / Defekt
- **Navigation**: ↑↓ oder J/K, PageUp/PageDown
- **Detail-Panel**: Zeigt vollständige Infos zum gewählten Schließfach
- **Edit-Modus** (`E`):
  - Wenn belegt: Bearbeite Verleih-Details (Username, Daten, Notizen)
  - Wenn frei: Bearbeite Schließfach-Details (Label, Standort, Höhe)
- **Neu** (`C`): Neuer Verleih für gewähltes freies Schließfach
- **Löschen** (`D`): Verleih löschen (mit Bestätigung)

#### 4.2.3 Tab: Verlängern (Extend)

Chat-Modus only (kein Manual):

```
┌─ Verleih verlängern ────────────────────────────────────────────────────┐
│                                                                          │
│  > Erinnert sich der Mieter an die Schließfach-Nummer?                 │
│    [Ja] [Nein]                                                          │
│                                                                          │
│  ✓ Ja ausgewählt                                                        │
│                                                                          │
│  > Schließfach-Nummer eingeben:                                         │
│    [A-042___]                                                           │
│                                                                          │
│  ✓ Verleih gefunden:                                                    │
│    • Schließfach: A-042 (Hauptgebäude)                                  │
│    • Mieter: max.mustermann (Schüler)                                   │
│    • Aktuelles Ende: 15.01.2025                                         │
│    • Status: Läuft aus in 28 Tagen                                      │
│                                                                          │
│    Ist das korrekt? [Ja] [Nein]                                         │
│                                                                          │
│  > Verlängerung um wie viele Jahre? (10€ pro Jahr)                      │
│    [1] [2] [3] [Anderer Betrag: ___€]                                   │
│                                                                          │
│  ✓ 1 Jahr ausgewählt (10,00 €)                                          │
│                                                                          │
│  Neues Enddatum: 15.01.2026                                             │
│                                                                          │
│  [Enter] Weiter | [ESC] Abbrechen                                       │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

**Workflow**:
1. Frage: Nummer bekannt? Ja/Nein
2. Falls Ja: Nummer eingeben → Combobox mit allen belegten Schließfächern
3. Falls Nein: Username eingeben → System sucht Verleih
4. **Bestätigungs-Dialog** mit Verleih-Details
5. Verlängerungsdauer auswählen (1/2/3 Jahre oder custom-Betrag)
   - Input validiert: Muss Vielfaches von 10€ sein
   - Berechnet neues Enddatum
6. **Zahlungs-Dialog**: "Hast du {betrag}€ erhalten?"
7. **Erinnerungs-Dialog**: "Hast du erinnert: Erneuerung in {dauer} erforderlich?"
8. Speichern → Payment-Eintrag (Extension) + Update rental_end_date
9. **Notification**: "Verleih erfolgreich verlängert bis {datum}"

#### 4.2.4 Tab: Zurückgeben (Return)

Ähnlich wie Extend:

```
┌─ Schließfach zurückgeben ───────────────────────────────────────────────┐
│                                                                          │
│  > Erinnert sich der Mieter an die Schließfach-Nummer?                 │
│    [Ja] [Nein]                                                          │
│                                                                          │
│  ✓ Nein ausgewählt                                                      │
│                                                                          │
│  > IServ-Benutzername eingeben:                                         │
│    [max.mustermann___]@athenetz.de                                      │
│                                                                          │
│  ✓ Verleih gefunden:                                                    │
│    • Schließfach: A-042 (Hauptgebäude)                                  │
│    • Mieter: max.mustermann (Schüler)                                   │
│    • Mietbeginn: 15.01.2024                                             │
│    • Vertragsende: 15.01.2025                                           │
│    • Status: Überfällig seit 47 Tagen! ⚠                                │
│                                                                          │
│  ⚠ WARNUNG: Ausstehende Verlängerungen                                  │
│    • Überfällig seit: 47 Tage                                           │
│    • Geschuldeter Betrag: 10,00 € (1 Jahr)                              │
│                                                                          │
│    Hat der Mieter die Schulden beglichen?                               │
│    [Ja, 10€ erhalten] [Nein, abbrechen]                                 │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

**Workflow**:
1. Nummer oder Username eingeben
2. Verleih finden
3. **Schulden-Prüfung**:
   - Falls überfällig: Dialog "Schulden: X€, beglichen?" Ja/Nein
   - Falls Nein: Abbruch, zurück zum Menü
   - Falls genau 10€ schuldig: Automatisch vom Pfand abziehen (kein Pfand zurück)
4. Falls erlaubt (keine oder beglichene Schulden):
   - **Pfand-Dialog**: "Hast du 10€ Pfand zurückgegeben?" Ja/Nein
5. Speichern:
   - `returned_at` setzen
   - `deposit_returned` aktualisieren
   - Falls Schulden bezahlt: Payment-Eintrag (Extension)
   - Falls Pfand zurück: Payment-Eintrag (DepositReturn, negativ)
6. **Notification**: "Schließfach A-042 erfolgreich zurückgegeben"

#### 4.2.5 Tab: Defekt melden (Damage)

Einfaches Formular:

```
┌─ Defekt melden ─────────────────────────────────────────────────────────┐
│                                                                          │
│  Schließfach-Nummer: [A-042___▼]                                        │
│                                                                          │
│  Aktueller Status:                                                      │
│    • Label: A-042                                                       │
│    • Standort: Hauptgebäude                                             │
│    • Belegt: Ja (max.mustermann)                                        │
│    • Defekt: Nein                                                       │
│                                                                          │
│  Notizen (optional):                                                    │
│  [_________________________________________________]                    │
│  [_________________________________________________]                    │
│  [_________________________________________________]                    │
│                                                                          │
│  ⚠ Hinweis: Ein defektes Schließfach kann weiterhin verliehen sein!    │
│             Die Reparatur sollte mit dem Mieter koordiniert werden.     │
│                                                                          │
│  [Enter] Als defekt markieren | [ESC] Abbrechen                         │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

**Funktionen**:
- Suche/Auswahl eines Schließfachs
- Zeigt aktuellen Status
- Setzt `is_damaged = TRUE`
- Optional: Notiz hinzufügen (wird in Audit-Log gespeichert)
- **Notification**: "A-042 als defekt markiert"

**Hinweis**: Reparatur-Status kann nur in Management → Lockers zurückgesetzt werden

---

### 4.3 Screen 3: Finanzen

Header mit Tabs:
```
┌────────────────────────────────────────────────────────────────────────────┐
│ Schließfach-Manager                                     [Finanzen]         │
│ ┌──────────────┬─────────────────┐                                         │
│ │  Übersicht  │ Schuldentabelle │                                         │
│ └──────────────┴─────────────────┘                                         │
├────────────────────────────────────────────────────────────────────────────┤
```

#### 4.3.1 Tab: Übersicht

```
┌─ Finanzübersicht ───────────────────────────────────────────────────────┐
│                                                                          │
│  Zeitraum: [Lebenszeit ▼]                                               │
│            [1 Jahr] [6 Monate] [3 Monate] [2 Monate] [1 Monat]          │
│                                                                          │
│  ┌─ Einnahmen (Zeitraum: Lebenszeit) ──────────────────────────────────┐│
│  │ Pfand-Einnahmen:          1.230,00 € (123 × 10€)                    ││
│  │ Verlängerungen:           9.450,00 € (945 × 10€)                    ││
│  │ Pfand-Rückgaben:           -840,00 € (84 × 10€)                     ││
│  │ ────────────────────────────────────────────────────────────────    ││
│  │ Gesamt-Einnahmen:         9.840,00 €                                ││
│  │ Netto (nach Pfand):       9.000,00 €                                ││
│  └─────────────────────────────────────────────────────────────────────┘│
│                                                                          │
│  ┌─ Ausstehende Zahlungen ──────────────────────────────────────────────┐│
│  │ Überfällige Verlängerungen:   320,00 € (32 Verleih)                 ││
│  │   davon Schüler:              280,00 € (28 Verleih)                 ││
│  │   davon Lehrer:                40,00 € (4 Verleih)                  ││
│  │                                                                      ││
│  │ Laufende Verleih (mögliche zukünftige Einnahmen):                   ││
│  │   Nächste 30 Tage:            120,00 € (12 Verleih)                 ││
│  │   Nächste 90 Tage:            380,00 € (38 Verleih)                 ││
│  └─────────────────────────────────────────────────────────────────────┘│
│                                                                          │
│  ┌─ Trend (12 Monate) ──────────────────────────────────────────────────┐│
│  │ 1000€ │                            ▆█                                ││
│  │  800€ │                      ▄▅▆▇███                                ││
│  │  600€ │              ▂▃▄▅▆▇███████                                  ││
│  │  400€ │        ▁▂▃▄▅███████████████                                 ││
│  │  200€ │  ▁▂▃▄▅█████████████████████                                 ││
│  │    0€ └───────────────────────────────────────────────────────────  ││
│  │        J F M A M J J A S O N D                                      ││
│  └─────────────────────────────────────────────────────────────────────┘│
│                                                                          │
│  [Tab] Schuldentabelle | [E] Exportieren | [ESC] Zurück                │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

**Datenquellen**:
- `db::queries::get_payment_summary(start: Date, end: Option<Date>)`
- `db::queries::get_overdue_rentals()`
- `db::queries::get_expiring_rentals(days: i32)`

**Export-Funktionen** (`E`):
- Dialog: Format auswählen (Markdown / JSON / CSV)
- Speichert Datei in `~/Downloads/` mit Zeitstempel
  - `finanzuebersicht_2024-06-15.md`
  - `finanzuebersicht_2024-06-15.json`
  - `finanzuebersicht_2024-06-15.csv`

#### 4.3.2 Tab: Schuldentabelle

```
┌─ Schuldentabelle ───────────────────────────────────────────────────────┐
│                                                                          │
│  Filter: [Alle ▼]  Sortierung: [Schulden (hoch→tief) ▼]                │
│                                                                          │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │ E-Mail                        │ Typ      │ Schulden │ Seit Tagen │ │
│  ├────────────────────────────────────────────────────────────────────┤ │
│  │ max.mustermann@athenetz.de    │ Schüler  │   30,00 €│     89     │ │
│  │ anna.schmidt@athenetz.de      │ Schüler  │   20,00 €│     62     │ │
│  │ peter.klein@athenetz.de       │ Schüler  │   20,00 €│     55     │ │
│  │ lisa.mueller@athenetz.de      │ Lehrer   │   10,00 €│     47     │ │
│  │ ...                                                                 │ │
│  │                                                                     │ │
│  │ Gesamt: 32 Schuldner          │          │  320,00 €│            │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                                                                          │
│  [E] Exportieren | [/] Suchen | [ESC] Zurück                            │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

**Datenquellen**:
- `db::queries::get_debtors()` → `Vec<DebtorInfo>`

**Filter**:
- Alle / Nur Schüler / Nur Lehrer
- Sortierung: Schulden (hoch/tief), Dauer (lang/kurz), Alphabetisch

**Export** (`E`):
- Formate: Markdown / JSON / CSV
- Dateinamen: `schuldentabelle_2024-06-15.{md|json|csv}`

**Export-Beispiele**:

*Markdown*:
```markdown
# Schuldentabelle
Stand: 15.06.2024

| E-Mail | Typ | Schulden | Überfällig seit |
|--------|-----|----------|-----------------|
| max.mustermann@athenetz.de | Schüler | 30,00 € | 89 Tagen |
| anna.schmidt@athenetz.de | Schüler | 20,00 € | 62 Tagen |

**Gesamt: 32 Schuldner, 320,00 € ausstehend**
```

*CSV*:
```csv
Email,Typ,Schulden_EUR,Ueberfaellig_Tage
max.mustermann@athenetz.de,Schüler,30.00,89
anna.schmidt@athenetz.de,Schüler,20.00,62
```

---

### 4.4 Screen 4: Management

Header mit Tabs:
```
┌────────────────────────────────────────────────────────────────────────────┐
│ Schließfach-Manager                                  [Verwaltung]          │
│ ┌──────────────┬──────────────┬──────────────┐                             │
│ │ Schließfächer│  Standorte   │    Backup    │                             │
│ └──────────────┴──────────────┴──────────────┘                             │
├────────────────────────────────────────────────────────────────────────────┤
```

#### 4.4.1 Tab: Schließfächer

```
┌─ Schließfächer-Verwaltung ──────────────────────────────────────────────┐
│                                                                          │
│  [N] Neues Schließfach | [B] Bulk-Erstellung | [/] Suchen              │
│                                                                          │
│  ┌────────────────────────────────────────┬────────────────────────────┐ │
│  │ ID    │ Standort     │ Höhe │ Defekt │ │ ┌─ Details: A-042 ───────┐│ │
│  │►A-042 │ Hauptgebäude │ 100  │ Ja  ⚠  │ │ │ Label: A-042           ││ │
│  │ A-043 │ Hauptgebäude │ 105  │ Nein   │ │ │ Standort: Hauptgebäude ││ │
│  │ A-044 │ Hauptgebäude │ 110  │ Nein   │ │ │ Höhe: 100 cm           ││ │
│  │ ...                                   │ │ │ Defekt: Ja ⚠           ││ │
│  │                                       │ │ │                        ││ │
│  │ 150 Schließfächer (8 defekt)         │ │ │ [E] Bearbeiten         ││ │
│  │                                       │ │ │ [R] Repariert markieren││ │
│  │                                       │ │ │ [D] Löschen            ││ │
│  │                                       │ │ └────────────────────────┘│ │
│  └────────────────────────────────────────┴────────────────────────────┘ │
│                                                                          │
│  [N] Neu | [B] Bulk | [E] Edit | [R] Reparatur-Reset | [D] Löschen     │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

**Funktionen**:

##### Neues Schließfach (`N`)
Formular:
```
┌─ Neues Schließfach ─────────────────────────────────────────────────────┐
│  Label:         [A-042_____]                                            │
│  Standort:      [Hauptgebäude ▼]                                        │
│  Höhe (cm):     [100____]                                               │
│  [Enter] Speichern | [ESC] Abbrechen                                    │
└──────────────────────────────────────────────────────────────────────────┘
```

##### Bulk-Erstellung (`B`)
Dialog für automatisches Anlegen mehrerer Schließfächer:
```
┌─ Bulk-Erstellung ───────────────────────────────────────────────────────┐
│                                                                          │
│  Standort:      [Hauptgebäude ▼]                                        │
│                                                                          │
│  ID-Präfix:     [A-___]                                                 │
│                                                                          │
│  Start-Nummer:  [1___]                                                  │
│                                                                          │
│  End-Nummer:    [50__]                                                  │
│                                                                          │
│  Höhe (cm):     [Auto (verteilt) ▼]                                     │
│                 [Auto] [Fest: ___cm]                                    │
│                                                                          │
│  Vorschau:                                                              │
│    A-001 (Hauptgebäude, ~20cm)                                          │
│    A-002 (Hauptgebäude, ~26cm)                                          │
│    ...                                                                  │
│    A-050 (Hauptgebäude, ~280cm)                                         │
│                                                                          │
│  Gesamt: 50 Schließfächer werden angelegt                               │
│                                                                          │
│  [Enter] Erstellen | [ESC] Abbrechen                                    │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

**Auto-Höhe**: Verteilt Höhen gleichmäßig zwischen 0-300cm (z.B. bei 50 Schließfächern: Schritte von ~6cm)

##### Reparatur-Reset (`R`)
Setzt `is_damaged = FALSE` für gewähltes Schließfach.
Bestätigungs-Dialog: "A-042 als repariert markieren?"

##### Löschen (`D`)
**Nur möglich wenn**:
- Kein aktiver Verleih existiert
- Alle historischen Verleih sind geschlossen (`returned_at` gesetzt)

Bestätigungs-Dialog:
```
⚠ Schließfach A-042 wirklich löschen?

Dieses Schließfach hat 3 historische Verleih.
Diese bleiben in der Datenbank erhalten.

[Enter] Löschen | [ESC] Abbrechen
```

#### 4.4.2 Tab: Standorte

```
┌─ Standorte-Verwaltung ──────────────────────────────────────────────────┐
│                                                                          │
│  [N] Neuer Standort                                                     │
│                                                                          │
│  ┌─────────────────────────────────────┬───────────────────────────────┐│
│  │ Name           │ Schließfächer │... │ ┌─ Hauptgebäude ───────────┐ ││
│  │►Hauptgebäude   │ 60            │    │ │ Name: Hauptgebäude        │ ││
│  │ Turnhalle      │ 50            │    │ │ Beschreibung:             │ ││
│  │ Neubau         │ 40            │    │ │   Erdgeschoss, Flur West  │ ││
│  │                                     │ │                           │ ││
│  │ 3 Standorte                         │ │ Schließfächer: 60         │ ││
│  │                                     │ │   Belegt: 45              │ ││
│  │                                     │ │   Frei: 15                │ ││
│  │                                     │ │                           │ ││
│  │                                     │ │ [E] Bearbeiten            │ ││
│  │                                     │ │ [D] Löschen               │ ││
│  │                                     │ └───────────────────────────┘ ││
│  └─────────────────────────────────────┴───────────────────────────────┘│
│                                                                          │
│  [N] Neu | [E] Edit | [D] Löschen | [ESC] Zurück                       │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

**Löschen**: Nur möglich wenn keine Schließfächer diesem Standort zugeordnet sind.

#### 4.4.3 Tab: Backup

```
┌─ Backup & Datenmanagement ──────────────────────────────────────────────┐
│                                                                          │
│  Datenbank-Pfad:                                                        │
│  /home/user/.local/share/schliessfach-manager/schliessfach.db           │
│  Größe: 2,4 MB | Letzte Änderung: 15.06.2024 14:32                      │
│                                                                          │
│  ┌─ Export ─────────────────────────────────────────────────────────┐  │
│  │                                                                   │  │
│  │  [1] Vollständiger Export (JSON)                                 │  │
│  │      Alle Daten (Schließfächer, Verleih, Zahlungen, Standorte)  │  │
│  │      → schliessfach_backup_2024-06-15_14-32.json                 │  │
│  │                                                                   │  │
│  │  [2] Schließfächer (CSV)                                         │  │
│  │      → lockers_2024-06-15.csv                                    │  │
│  │                                                                   │  │
│  │  [3] Aktive Verleih (CSV)                                        │  │
│  │      → active_rentals_2024-06-15.csv                             │  │
│  │                                                                   │  │
│  │  [4] Zahlungshistorie (CSV)                                      │  │
│  │      → payments_2024-06-15.csv                                   │  │
│  │                                                                   │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                                                          │
│  ┌─ Import ─────────────────────────────────────────────────────────┐  │
│  │                                                                   │  │
│  │  [5] Vollständiger Import (JSON)                                 │  │
│  │      ⚠ WARNUNG: Überschreibt alle vorhandenen Daten!             │  │
│  │      Erstellt automatisch Backup vor Import.                     │  │
│  │                                                                   │  │
│  │  [6] Schließfächer importieren (CSV)                             │  │
│  │      Fügt neue Schließfächer hinzu (dupliziert nicht)            │  │
│  │                                                                   │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                                                          │
│  Speicherort: ~/Downloads/                                              │
│                                                                          │
│  [1-6] Aktion wählen | [ESC] Zurück                                     │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

**Export-Formate**:

*JSON (Vollständig)*:
```json
{
  "version": "2.0",
  "exported_at": "2024-06-15T14:32:00Z",
  "lockers": [...],
  "rentals": [...],
  "payments": [...],
  "locations": [...]
}
```

*CSV (Beispiele)*:
- `lockers.csv`: id, label, location, height, is_damaged, created_at
- `active_rentals.csv`: id, locker_label, tenant_username, tenant_type, start_date, end_date, deposit_paid
- `payments.csv`: id, rental_id, amount_cents, payment_type, payment_date

**Import**:
- Datei-Browser öffnet sich (ratatui File-Picker oder externe Auswahl)
- Vor vollständigem Import: Automatisches Backup der aktuellen DB erstellen
- Validierung der Import-Daten (Schema-Check, Constraints)
- Fehlerbehandlung mit detailliertem Report

---

## 5. UI/UX-Komponenten & Widgets

### 5.1 Chat-Dialog Widget

Interaktiver Dialog im Chat-Stil (wie `npx create-xyz`):

```rust
pub struct ChatDialog {
    steps: Vec<ChatStep>,
    current_step: usize,
    history: Vec<ChatMessage>,
    input: String,
    cursor_position: usize,
}

pub enum ChatStep {
    Question {
        prompt: String,
        subtitle: Option<String>,
        input_type: InputType,
    },
    Confirmation {
        prompt: String,
        details: Vec<(String, String)>,
    },
    Info {
        message: String,
    },
}

pub enum InputType {
    Text,
    Choice(Vec<String>),
    Dropdown(Vec<String>),
    YesNo,
    Money, // Validiert Geldbeträge
}
```

**Features**:
- **History**: Zeigt vorherige Schritte grau an
- **Current Input**: Highlighted
- **Validation**: Inline-Fehler bei ungültiger Eingabe
- **Navigation**: ↑/↓ für Choices, Tab für Completion
- **Shortcuts**: Bestätigungs-Tasten (Y/N, Enter)

### 5.2 Manual-Form Widget

Editierbares Formular mit Feldern:

```rust
pub struct Form {
    fields: Vec<FormField>,
    focused_field: usize,
}

pub enum FormField {
    TextInput { label: String, value: String, suffix: Option<String> },
    Combobox { label: String, options: Vec<String>, selected: usize },
    Radio { label: String, options: Vec<String>, selected: usize },
    Checkbox { label: String, checked: bool },
    DatePicker { label: String, date: NaiveDate },
    TextArea { label: String, value: String, lines: usize },
}
```

**Features**:
- **Tab-Navigation**: Zwischen Feldern wechseln
- **Validation**: Per-Feld-Regeln
- **Visual Feedback**: Fokussiertes Feld highlighted
- **Keyboard Shortcuts**: Direkte Feld-Auswahl (Alt+Zahl)

### 5.3 Confirmation-Dialog

Einfacher Bestätigungs-Popup:

```rust
pub struct ConfirmationDialog {
    title: String,
    message: String,
    subtitle: Option<String>,
    buttons: Vec<Button>,
    selected_button: usize,
}

pub struct Button {
    label: String,
    action: ButtonAction,
    style: ButtonStyle,
}

pub enum ButtonAction {
    Confirm,
    Cancel,
    Custom(String),
}
```

**Rendering**:
```
┌─ Bestätigung ───────────────────────────────┐
│                                             │
│  Hast du 20€ erhalten und in die Kasse     │
│  getan?                                     │
│                                             │
│  10€ als Pfand; 10€ jährliche Verleihkosten│
│                                             │
│  ┌─────────┐  ┌─────────┐                  │
│  │   Ja    │  │  Nein   │                  │
│  └─────────┘  └─────────┘                  │
│                                             │
└─────────────────────────────────────────────┘
```

### 5.4 Notification Widget

Temporärer Popup (z.B. 10 Sekunden):

```rust
pub struct Notification {
    message: String,
    level: NotificationLevel,
    created_at: Instant,
    duration: Duration,
}

pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}
```

**Rendering**: Kleiner Popup oben rechts
```
┌─────────────────────────────────┐
│ ✓ Gespeichert! (ID: 123)        │
│ [Enter] OK                      │
└─────────────────────────────────┘
```

### 5.5 Table-With-Detail Widget

Geteilter Screen: Tabelle links, Details rechts:

```rust
pub struct TableWithDetail<T> {
    items: Vec<T>,
    selected_index: usize,
    columns: Vec<ColumnDef>,
    detail_renderer: Box<dyn Fn(&T) -> Vec<Line>>,
    filter: Option<String>,
}
```

**Features**:
- **Sortierung**: Click auf Column-Header
- **Filter**: Live-Suche mit `/`
- **Detail-Panel**: Automatisch aktualisiert bei Selektion
- **Paging**: PageUp/PageDown bei vielen Einträgen

### 5.6 Searchable-Combobox

Dropdown mit integrierter Suche:

```rust
pub struct Combobox {
    options: Vec<String>,
    filtered_options: Vec<usize>, // Indices
    filter: String,
    selected: usize,
    is_open: bool,
}
```

**Verhalten**:
- **Dropdown öffnen**: Enter oder ↓
- **Tippen**: Filtert Liste live
- **Navigation**: ↑/↓ durch gefilterte Optionen
- **Auswahl**: Enter bestätigt

### 5.7 Export-Dialog

Format-Auswahl für Exporte:

```rust
pub struct ExportDialog {
    format_options: Vec<ExportFormat>,
    selected_format: usize,
    filename_preview: String,
}

pub enum ExportFormat {
    Json,
    Csv,
    Markdown,
}
```

---

## 6. Workflows & Business-Logik

### 6.1 Workflow: Schließfach verleihen

```rust
pub struct RentLockerWorkflow {
    state: RentState,
    db: Database,
}

pub enum RentState {
    ChooseLocation,
    ChooseHeight,
    SelectLocker { available: Vec<Locker> },
    ConfirmLocker { locker: Locker },
    EnterUsername,
    ChooseTenantType,
    ManualEdit { draft: Rental },
    ConfirmPayment,
    ConfirmReminder,
    Completed { rental_id: i64 },
}

impl RentLockerWorkflow {
    pub fn next(&mut self, input: WorkflowInput) -> Result<WorkflowAction> {
        match &self.state {
            RentState::ChooseLocation => {
                // Verarbeite Standort-Auswahl
                // → Transition zu ChooseHeight
            },
            // ... weitere State-Transitions
        }
    }
}
```

**States**:
1. `ChooseLocation`: Standort-Buttons
2. `ChooseHeight`: Höhe-Buttons (Unten/Mitte/Oben/Egal)
3. `SelectLocker`: Liste passender Schließfächer
4. `ConfirmLocker`: Bestätigung mit Details
5. `EnterUsername`: Text-Input mit "@athenetz.de" Suffix
6. `ChooseTenantType`: Radio-Buttons (Schüler/Lehrer)
7. `ManualEdit`: Form mit allen Feldern
8. `ConfirmPayment`: Bestätigungs-Dialog "20€ erhalten?"
9. `ConfirmReminder`: Bestätigungs-Dialog "Erinnert?"
10. `Completed`: Speichern, Navigation zur Liste

### 6.2 Workflow: Verlängern

Ähnlich strukturiert, aber vereinfacht:

```rust
pub enum ExtendState {
    AskKnowsNumber,
    EnterNumber { knows_number: bool },
    EnterUsername,
    ConfirmRental { rental: RentalWithLocker },
    ChooseDuration,
    ConfirmPayment { amount_cents: i32 },
    ConfirmReminder,
    Completed,
}
```

### 6.3 Workflow: Zurückgeben

Includes Schulden-Prüfung:

```rust
pub enum ReturnState {
    AskKnowsNumber,
    EnterNumber { knows_number: bool },
    EnterUsername,
    CheckDebts { rental: RentalWithLocker },
    ConfirmDebtPayment { debt_cents: i32 },
    ConfirmDepositReturn,
    Completed,
}
```
Weiterführung in spec_part2.md
