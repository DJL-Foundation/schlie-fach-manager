# Benutzerhandbuch / User Guide

Schließfach-Manager v2.1 - Terminal-basierte Verwaltungssoftware für Schließfächer

## Inhaltsverzeichnis

1. [Installation](#installation)
2. [Schnellstart](#schnellstart)
3. [Bildschirme](#bildschirme)
4. [Workflows](#workflows)
5. [Export & Import](#export--import)
6. [Einstellungen](#einstellungen)
7. [Fehlerbehebung](#fehlerbehebung)

---

## Installation

### Voraussetzungen

- Moderner Terminal-Emulator mit UTF-8 Support
- Mindestgröße: 80×24 Zeichen (empfohlen: 120×40+)
- Betriebssystem: Linux, macOS oder Windows

### Installation von crates.io

```bash
cargo install schliessfach-manager
```

### Installation aus dem Quellcode

```bash
git clone https://github.com/DJL-Foundation/schlie-fach-manager.git
cd schlie-fach-manager
cargo build --release
./target/release/schliessfach-manager
```

### Vorkompilierte Binaries

Laden Sie vorkompilierte Binaries von der [Releases-Seite](https://github.com/DJL-Foundation/schlie-fach-manager/releases) herunter.

| Plattform | Architektur | Datei |
|-----------|-------------|-------|
| Linux | x86_64 | `schliessfach-manager-linux-x86_64.tar.gz` |
| Linux | ARM64 | `schliessfach-manager-linux-aarch64.tar.gz` |
| macOS | x86_64 | `schliessfach-manager-macos-x86_64.tar.gz` |
| macOS | Apple Silicon | `schliessfach-manager-macos-aarch64.tar.gz` |
| Windows | x86_64 | `schliessfach-manager-windows-x86_64.zip` |

---

## Schnellstart

### Anwendung starten

```bash
schliessfach-manager
```

### Erste Schritte

1. **Dashboard anzeigen**: Die Anwendung startet automatisch im Dashboard
2. **Schließfächer anlegen**: 
   - Wechseln Sie zur Verwaltung (`^` → Verwaltung)
   - Nutzen Sie "Bulk-Erstellung" für mehrere Schließfächer
3. **Ersten Verleih erstellen**: 
   - Drücken Sie `1` im Dashboard
   - Folgen Sie dem Wizard

### Grundlegende Tastenkombinationen

| Taste | Aktion |
|-------|--------|
| `Tab` | Nächstes Element |
| `Esc` | Zurück/Abbrechen |
| `Esc` ×3 | Zum Dashboard |
| `^` | Fenster wechseln |
| `Shift+Q` | Beenden |

---

## Bildschirme

### Dashboard

Das Dashboard bietet eine Übersicht über:

```
┌─────────────────────────────────────────────┐
│ BELEGUNG                                     │
│ Gesamt: 120/150 (80% belegt)                │
│ ████████████████████░░░░░                   │
├─────────────────────────────────────────────┤
│ AKTIONEN ERFORDERLICH                        │
│ • 5 überfällige Rückgaben                   │
│ • 12 auslaufende Verträge (30 Tage)         │
│ • 3 defekte Schließfächer                   │
├─────────────────────────────────────────────┤
│ STANDORTE         │ FINANZEN                │
│ Hauptgebäude: 80  │ Ausstehend: €142,50     │
│ Turnhalle: 40     │ Einnahmen (30T): €892   │
└─────────────────────────────────────────────┘
```

**Dashboard-Shortcuts:**
- `1` - Neuen Verleih suchen
- `2` - Verleihliste anzeigen
- `3` - Vertrag verlängern
- `4` - Schließfach zurückgeben
- `5` - Defekt melden

### Verleih-Management

Fünf Tabs für alle Verleihvorgänge:

1. **Suche**: Freie Schließfächer finden und vermieten
2. **Liste**: Alle aktiven Vermietungen
3. **Verlängern**: Verträge verlängern
4. **Rückgabe**: Schließfächer zurücknehmen
5. **Defekt**: Defekte melden und reparieren

### Finanzen

Übersicht über alle finanziellen Aspekte:

- **Ausstehende Zahlungen**: Offene Schulden
- **Einnahmen**: Nach Zeitraum gefiltert
- **Pfandbeträge**: Gesammelt vs. ausstehend
- **Zahlungsverlauf**: Chronologische Liste

### Verwaltung

Vier Tabs für die Administration:

1. **Schließfächer**: Erstellen, bearbeiten, Status ändern
2. **Standorte**: Standortverwaltung
3. **Einstellungen**: Preise, Timeouts, Export-Einstellungen
4. **Audit Log**: Änderungsverlauf

---

## Workflows

### Workflow: Neuer Verleih

1. **Vom Dashboard**: Drücken Sie `1` oder navigieren Sie zu Verleih-Management → Suche
2. **Standort wählen**: Wählen Sie den gewünschten Standort
3. **Größe wählen**: Wählen Sie die gewünschte Schließfach-Größe
4. **Schließfach auswählen**: Ein passendes Schließfach wird vorgeschlagen
5. **Mieter-Daten eingeben**:
   - Benutzername (wird zu Email: benutzername@athenetz.de)
   - Mietertyp: Schüler oder Lehrer
6. **Zeitraum festlegen**: Start- und Enddatum
7. **Pfand**: Pfandzahlung bestätigen
8. **Bestätigung**: Verleih abschließen

**Beispiel-Dialog:**

```
┌─────────────────────────────────────────────┐
│ System                          [Schritt 1/5]│
├─────────────────────────────────────────────┤
│ Welchen Standort bevorzugt der Verleiher?   │
│                                              │
│   > Hauptgebäude (42 verfügbar)             │
│     Turnhalle (12 verfügbar)                │
│     Keller (8 verfügbar)                    │
└─────────────────────────────────────────────┘
```

### Workflow: Vertrag verlängern

1. **Vom Dashboard**: Drücken Sie `3`
2. **Mieter suchen**: Benutzername oder Schließfach-Nummer eingeben
3. **Neues Enddatum**: Verlängerungszeitraum wählen (1 Jahr Standard)
4. **Gebühr**: Jahresgebühr (10€) bestätigen
5. **Bestätigung**: Verlängerung abschließen

### Workflow: Rückgabe

1. **Vom Dashboard**: Drücken Sie `4`
2. **Schließfach identifizieren**: Nummer oder Mieter eingeben
3. **Zustand prüfen**: Schließfach-Zustand bestätigen
   - OK: Pfand wird zurückgegeben
   - Beschädigt: Schadensprotokoll erstellen
4. **Schulden prüfen**: Offene Beträge anzeigen
5. **Pfandrückgabe**: Bei keinen Schulden wird Pfand erstattet
6. **Bestätigung**: Rückgabe abschließen

### Workflow: Defekt melden

1. **Vom Dashboard**: Drücken Sie `5`
2. **Schließfach auswählen**: Nummer eingeben oder aus Liste wählen
3. **Schadensart**: Art des Defekts beschreiben
4. **Foto-Notiz**: Optional: Notiz für Dokumentation
5. **Status setzen**: "Defekt" markieren
6. **Später**: "Repariert" markieren wenn behoben

---

## Export & Import

### Unterstützte Formate

| Format | Export | Import | Verwendung |
|--------|--------|--------|------------|
| **TOML** | ✓ | ✓ | Standard-Backup, manuell editierbar |
| JSON | ✓ | ✓ | API-Integration, vollständige Backups |
| CSV | ✓ | ✓ | Tabellenkalkulation, Listen |
| Markdown | ✓ | ✗ | Reports, Dokumentation |

### Export durchführen

1. Navigieren Sie zu **Finanzen** oder **Verwaltung**
2. Drücken Sie `E` für Export
3. Wählen Sie das Format
4. Wählen Sie den Speicherort
5. Export wird erstellt

**Export-Beispiel (TOML):**

```toml
[metadata]
version = "2.1.0"
export_date = "2025-01-20T15:30:00Z"

[[lockers]]
id = 1
label = "A-001"
location = "Hauptgebäude"
height = 150
is_damaged = false

[[rentals]]
id = 1
locker_id = 1
tenant_username = "max.mustermann"
tenant_type = "Schüler"
start_date = "2024-06-01"
end_date = "2025-05-31"
```

### Import durchführen

1. Navigieren Sie zu **Finanzen** oder **Verwaltung**
2. Drücken Sie `I` für Import
3. Wählen Sie die Datei
4. Vorschau der zu importierenden Daten
5. Import bestätigen

**Wichtig:**
- Duplikate werden automatisch übersprungen
- IDs werden bei Import neu vergeben
- Referenzen (Locker → Rental) werden automatisch verknüpft

### CSV-Import

**Schließfächer-CSV:**
```csv
Label,Standort,Hoehe_cm,Defekt
A-001,Hauptgebäude,150,Nein
A-002,Hauptgebäude,120,Ja
B-001,Turnhalle,100,Nein
```

**Vermietungen-CSV:**
```csv
Schliessfach,Mieter,Typ,Beginn,Ende
A-001,max.mustermann,Schüler,2024-01-01,2024-12-31
A-002,anna.schmidt,Lehrer,2024-02-01,2025-01-31
```

---

## Einstellungen

### Preise & Abrechnung

| Einstellung | Standard | Beschreibung |
|-------------|----------|--------------|
| Pfandbetrag | 10,00 € | Kaution bei Verleih |
| Jahresgebühr | 10,00 € | Verlängerungsgebühr pro Jahr |
| Überfälligkeitsgebühr | 10,00 € | Pro Jahr (anteilig) |
| Währung | EUR | ISO 4217 Code |

### Screensaver

| Einstellung | Standard | Beschreibung |
|-------------|----------|--------------|
| Timeout | 60 s | Inaktivität bis Countdown |
| Countdown | 15 s | Vorwarnung vor Aktivierung |

### Export-Einstellungen

| Einstellung | Standard | Beschreibung |
|-------------|----------|--------------|
| Standard-Format | TOML | Bevorzugtes Export-Format |
| Export-Verzeichnis | `~/exports/` | Zielverzeichnis |

---

## Fehlerbehebung

### Anwendung startet nicht

**Problem:** Fehlermeldung "Terminal zu klein"

**Lösung:** Vergrößern Sie das Terminal-Fenster auf mindestens 80×24 Zeichen.

---

**Problem:** Datenbank-Fehler beim Start

**Lösung:** 
1. Prüfen Sie die Berechtigungen im Datenverzeichnis
2. Linux: `~/.local/share/schliessfach-manager/`
3. macOS: `~/Library/Application Support/schliessfach-manager/`
4. Windows: `%APPDATA%\schliessfach-manager\`

### Navigation funktioniert nicht

**Problem:** Tasten reagieren nicht

**Lösung:**
1. Prüfen Sie, ob Sie in einem Eingabefeld sind (Escape drücken)
2. Prüfen Sie den aktuellen Modus (Window Switcher aktiv?)
3. Nutzen Sie den Escape-Indikator: 3× Escape → Dashboard

### Import schlägt fehl

**Problem:** "Datei konnte nicht gelesen werden"

**Lösung:**
1. Prüfen Sie den Dateipfad
2. Prüfen Sie die Dateicodierung (UTF-8 erforderlich)
3. Prüfen Sie das Dateiformat (JSON/TOML/CSV)

---

**Problem:** "Duplikate wurden übersprungen"

**Erklärung:** Datensätze mit gleichen Kennungen existieren bereits.

**Lösung:** Das ist normales Verhalten. Nur neue Datensätze werden importiert.

### Screensaver

**Problem:** Screensaver aktiviert sich zu schnell

**Lösung:** 
1. Verwaltung → Einstellungen
2. Screensaver-Timeout erhöhen (z.B. 300 Sekunden)

---

**Problem:** Nach Screensaver-Exit: Falscher Bildschirm

**Erklärung:** Der Screensaver kehrt immer zum Dashboard zurück (by design).

**Lösung:** Navigieren Sie mit `^` (Window Switcher) oder Dashboard-Shortcuts.

### Daten-Backup

**Best Practice für regelmäßige Backups:**

```bash
# Automatisches Backup-Skript
#!/bin/bash
BACKUP_DIR="$HOME/backups/schliessfach"
DATE=$(date +%Y-%m-%d)
mkdir -p "$BACKUP_DIR"
cp ~/.local/share/schliessfach-manager/data.db "$BACKUP_DIR/data-$DATE.db"
```

### Kontakt & Support

Bei weiteren Fragen:
- GitHub Issues: [github.com/DJL-Foundation/schlie-fach-manager/issues](https://github.com/DJL-Foundation/schlie-fach-manager/issues)
- Dokumentation: [docs.rs/schliessfach-manager](https://docs.rs/schliessfach-manager)
