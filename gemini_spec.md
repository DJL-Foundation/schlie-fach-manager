# Schließfach Manager Next-Gen - Spezifikation & Implementierungsplan

## 1. Übersicht
Diese Spezifikation beschreibt die Neuimplementierung des "Schließfach Managers". Das Ziel ist eine robuste Terminal User Interface (TUI) Anwendung in Rust, die den täglichen Betrieb der Schließfachverwaltung an einer Schule abbildet. Der Fokus liegt auf UX (Wizard-Dialoge, Dashboard) und Datensicherheit (SQLite, Backups).

## 2. Technischer Stack
*   **Sprache:** Rust (Edition 2021)
*   **UI Framework:** `ratatui` (für das TUI Rendering)
*   **Event Handling:** `crossterm`
*   **Datenbank:** `rusqlite` (SQLite)
*   **Validierung/Logik:** Benutzerdefinierte State-Machines für die "Wizards".
*   **Export:** `serde`, `csv`, `serde_json` (für XML/JSON/Markdown Exports).
*   **Testing:** `rstest
` und Standard `test` Module für Unit/Integration Tests.

## 3. Datenmodell (Datenbank Schema)

Wir verwenden ein relationales Modell, um Schließfächer von den Verleihvorgängen zu trennen. Dies ermöglicht Historie und sauberere Finanzberechnungen.

### Tabelle: `lockers` (Die physischen Schließfächer)
| Feld | Typ | Beschreibung |
|---|---|---|
| `id` | INTEGER PK | Interne ID |
| `display_number` | TEXT | Die sichtbare Nummer am Schließfach (z.B. "A-101") |
| `location_id` | INTEGER FK | Verweis auf `locations` Tabelle |
| `height` | TEXT | Enum: 'Top', 'Middle', 'Bottom' (Für "Kein Dachboden für 5. Klässler") |
| `status` | TEXT | Enum: 'Free', 'Occupied', 'Maintenance' |
| `is_damaged` | BOOLEAN | Unabhängig vom Status (z.B. Kratzer, aber nutzbar) |
| `note` | TEXT | Interne Notizen (Defekt-Beschreibung) |

### Tabelle: `locations` (Standorte)
| Feld | Typ | Beschreibung |
|---|---|---|
| `id` | INTEGER PK | |
| `name` | TEXT | z.B. "Hauptgebäude", "Sporthalle", "Dachboden" |

### Tabelle: `leases` (Die Verleihvorgänge/Historie)
| Feld | Typ | Beschreibung |
|---|---|---|
| `id` | INTEGER PK | |
| `locker_id` | INTEGER FK | Verweis auf `lockers` |
| `tenant_username` | TEXT | IServ Benutzername (z.B. max.mustermann) |
| `tenant_type` | TEXT | Enum: 'Student', 'Teacher' |
| `start_date` | DATETIME | Datum des Verleihbeginns |
| `end_date` | DATETIME | Aktuelles Ablaufdatum (wird bei Verlängerung hochgesetzt) |
| `deposit_paid` | BOOLEAN | Wurde Pfand (10€) bezahlt? |
| `yearly_fee_paid_until`| DATETIME | Bis wann ist die Jahresgebühr gedeckt? |
| `is_active` | BOOLEAN | True = Aktueller Mieter, False = Historie |

## 4. UI Architektur & Navigation

Die App basiert auf einem Tab-System mit globalen Hotkeys.

**Hauptmenü-
Struktur:**
1.  **Dashboard** (Startseite)
2.  **Schließfach Management** (Day-to-Day Operations)
3.  **Finanzen** (Reporting)
4.  **Admin / Management** (Stammdaten & Config)

### UX-Konzepte
*   **Wizard-Mode ('Chat'):** Ein interaktiver Dialog für komplexe Aufgaben (Verleih, Verlängerung). Sieht aus wie ein Chat-Verlauf. Der User antwortet auf Fragen der App.
*   **Manual-Mode ('Formular'):** Klassische TUI-Formulareingabe für Power-User oder Korrekturen. Umschaltbar mit `m` (Manuell) und `c` (Chat).
*   **Popups:** Für Bestätigungen und kurze Infos (Auto-Close nach 10s oder Enter).

---

## 5. Detaillierte Funktionsbeschreibung

### 5.1 Startseite (Dashboard)
Ein Read-Only Dashboard mit Widgets:
*   **Belegungs-Graph:** Pie-Chart oder Gauge (Frei vs. Belegt vs. Defekt).
*   **Standort-Übersicht:** Balkendiagramm (Belegung pro Standort).
*   **Action Items:** Liste mit "Dringende Reparaturen" und "Überfällige Zahlungen/Abläufe".
*   **Finanz-Vorschau:** "Mögliche Einnahmen durch Verlängerungen diesen Monat".

### 5.2 Schließfach Management
Hat 3 Sub-Tabs oder Modi:

#### A. Suchen & Verleihen (Wizard)
*   **Ablauf (Chat Mode):**
    1.  Bot: "Welcher Standort wird bevorzugt?" (Dropdown/Liste)
    2.  Bot: "Welche Höhe?" (Top/Middle/Bottom)
    3.  *System sucht freie Fächer.*
    4.  Bot: "Vorschlag: Fach #123. Ist das okay?" (Ja/Nein/Anderes suchen)
    5.  Bot: "Wie lautet der IServ Benutzername?" (Input mit Suffix `@athenetz.de` und Validierung). Hinweis im Chat: "Denk an den IServ Namen für Mahnungen!"
    6.  Bot: "Ist der Mieter Lehrer oder Schüler?" (Selection)
    7.  Bot: "Speichern?"
    8.  **Pre-Save Check:** Popup "Hast du 20€ erhalten?" (10€ Pfand + 10€ Miete).
    9.  **Post-Save Check:** Popup "Hast du den Schüler erinnert, dass jährlich verlängert werden muss?"
*   **Manual Mode:** Ein Formular, in dem ID, User, Typ direkt eingetragen werden können. Dropdowns mit Suchfunktion.

#### B. Schließfächer Liste
*   Tabellarische Ansicht (Ratatui Table).
*   Spalten: ID, Nummer, Verleiher, Startdatum, Ablaufdatum, Status.
*   Navigation: Hoch/Runter, `/` zum Filtern.
*   Detail-Ansicht: Rechts (oder Popup) zeigt alle Details zum selektierten Fach.
*   Aktion `e`: Editiert den *laufenden* Mietvertrag (Wechselt in Manual-Edit View). Löschen (`d`) beendet Vertrag.

#### C. Verlängern & Rückgabe
*   **Verlängern:**
    *   Frage: "Nummer bekannt?" -> Eingabe Nummer ODER Eingabe Username.
    *   Bestätigung: "Fach 123 von Max Mustermann?"
    *   Input: "Wie viel Geld wurde übergeben?" (Validierung: Muss durch 10 teilbar sein). Default: 10€.
    *   Logik: Addiert 1 Jahr pro 10€ auf das `end_date`.
*   **Rückgabe:**
    *   Prüfung auf offene Schulden (Ist `end_date` < `today`?).
    *   Dialog: "Benutzer schuldet noch X€. Bezahlt?" (Ja -> weiter, Nein -> Abbruch).
    *   Dialog: "Pfand (10€) zurückgegeben?"
    *   Aktion: Setzt `leases.is_active = false`, `lockers.status = Free`.

#### D. Defekt Meldung
*   Simple Eingabe der ID.
*   Setzt `lockers.is_damaged = true` und `lockers.note`.
*   Status bleibt "Occupied" wenn verliehen, sonst "Maintenance".

### 5.3 Finanzen
Nur für Administratoren/Lehrer.
*   **Ansicht:** Tabelle mit "Einnahmen Historie" (Filterbar: Letzte 30 Tage, 6 Monate, Lifetime).
*   **Schuldner-Liste:** Aggregierte Tabelle: `E-Mail (@athenetz.de)` | `Offener Betrag`.
*   **Export:** Hotkey `x` öffnet Export-Dialog. Speichert `[ReportTyp]_[Datum].json/xml/md` in den Download-Ordner.

### 5.4 Management (Admin)
*   **CRUD:** Standorte und Schließfächer manuell anlegen/editieren.
*   **Reparatur:** "Defekt"-Status zurücksetzen.
*   **Bulk-Creation (QOL):**
    *   Dialog: Standort wählen.
    *   Präfix eingeben (z.B. "B-").
    *   Start-Nummer (1) bis End-Nummer (50).
    *   Erstellt automatisch 50 Einträge in der DB.
*   **Backup:** DB Export/Import Funktion.

## 6. Implementierungsplan

### Phase 1: Core & Domain (Tage 1-2)
*   [ ] Rust Projekt Setup & Dependencies (`ratatui`, `rusqlite`, `serde`).
*   [ ] DB Modul: Migrationen erstellen (Tabellen `lockers`, `locations`, `leases`).
*   [ ] Model Modul: Rust Structs, die das DB Schema abbilden.
*   [ ] Logik: Funktionen für `rent_locker`, `extend_lease`, `return_locker`, `calculate_debt`.
*   [ ] Tests: Unit Tests für die Finanzlogik und DB Constraints.

### Phase 2: Basis UI Framework (Tage 3-4)
*   [ ] State Management: `App` Struct, das den aktuellen Tab und globalen State hält.
*   [ ] Layouting: Grundgerüst für Dashboard, Header (Tabs) und Content-Area.
*   [ ] Input Handling: Event-Loop für Key-Presses und Tab-Switching.

### Phase 3: Operations Module (Der "Wizard") (Tage 5-7)
*   [ ] Implementierung des "Chat Widgets" (Render History, Input Field).
*   [ ] State Machine für den Verleih-Prozess (Enum States: `AskLocation`, `AskHeight`, `ConfirmLocker`, ...).
*   [ ] Integration der "Manual" Formulare (Wechsel zwischen Chat/Formular).
*   [ ] Validierung der Inputs (Usernames, Geldbeträge).

### Phase 4: Dashboard & Listen (Tage 8-9)
*   [ ] Implementierung der "Schließfächer Liste" mit Suche/Filter.
*   [ ] Dashboard Widgets mit Live-Daten aus der DB (Queries für Statistiken).

### Phase 5: Finanzen & Admin (Tage 10-11)
*   [ ] Finanz-Reports und Schuldenberechnung.
*   [ ] Export Funktionen (XML/Markdown Generator).
*   [ ] Admin Page: Bulk-Creation Tool.

### Phase 6: Polish & Tests (Tag 12)
*   [ ] UI Feinschliff (Farben, Ränder, deutsche Texte).
*   [ ] E2E Walkthrough (Manueller Test aller Flows).
*   [ ] Error Handling (Was passiert bei DB Lock? Ungültigen Inputs?).

## 7. Besondere Anforderungen (Checkliste)
*   [ ] **IServ Integration:** Automatisches Suffix `@athenetz.de` in der UI visualisieren.
*   [ ] **Geld:** Input nur in 10er Schritten bei Verlängerung.
*   [ ] **Erinnerungen:** Popups ("Geld kassiert?", "Belehrt?") sind zwingend.
*   [ ] **Maintenance:** Kann `true` sein, auch wenn Status `Occupied`.
*   [ ] **Dateinamen:** Export Format `[Name]_[Datum].[ext]`.