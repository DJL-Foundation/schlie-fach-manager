# Changelog

Alle wichtigen Änderungen am Schließfach-Manager werden in dieser Datei dokumentiert.

Das Format basiert auf [Keep a Changelog](https://keepachangelog.com/de/1.0.0/),
und dieses Projekt folgt [Semantic Versioning](https://semver.org/lang/de/).

## [2.1.0] - 2025-01-20

### Hinzugefügt

#### Neue UI-Features

- **3× Escape zum Dashboard**: Dreimaliges Drücken von Escape innerhalb von 1 Sekunde 
  führt direkt zum Dashboard zurück - von jedem Bildschirm aus
- **Escape-Indikator `[|||]`**: Visuelles Feedback in der Status-Bar zeigt den 
  Escape-Zähler an (rechts unten)
- **Screensaver**: Automatischer Bildschirmschoner nach Inaktivität
  - 60 Sekunden Timeout (konfigurierbar)
  - 15 Sekunden Countdown-Vorwarnung
  - 5 verschiedene ASCII-Animationen (zufällig)
  - Deaktivierung durch beliebige Taste → Rückkehr zum Dashboard
- **Window Switcher (`^`)**: Schnelles Wechseln zwischen Hauptbildschirmen
- **Dashboard-Shortcut `5`**: Direkte Navigation zu "Defekt melden"
- **Fixed-Height Dashboard Layout**: Stabiles Layout bei Terminal-Resize

#### Import-Modul

- **`import_full_backup_json()`**: Vollständige Backup-Wiederherstellung aus JSON
- **`import_full_backup_toml()`**: Vollständige Backup-Wiederherstellung aus TOML
- **`import_lockers_csv()`**: Schließfächer aus CSV importieren
- **`import_rentals_csv()`**: Vermietungen aus CSV importieren
- **`ImportStats`**: Detaillierte Statistiken über Import-Vorgänge

#### Dokumentation

- **docs/KEYBINDINGS.md**: Vollständige Tastenkombinations-Dokumentation
- **docs/USER_GUIDE.md**: Umfassendes Benutzerhandbuch mit Workflows
- **docs/CHANGELOG.md**: Änderungsprotokoll im Keep a Changelog Format
- Verbesserte Inline-Dokumentation für alle Module

### Geändert

- **Export-Modul**: TOML als Standard-Export-Format (vorher JSON)
- **Keybind-Bar**: Zweizeilig mit globalen und kontextspezifischen Keybinds
- **Status-Bar**: Escape-Indikator hinzugefügt
- **Version**: Aktualisiert auf 2.1.0

### Verbessert

- **CSV-Import**: Flexible Spaltennamen (DE/EN)
- **Fehlerbehandlung**: Bessere Fehlermeldungen beim Import
- **Transaktionen**: Atomare Import-Operationen
- **Dokumentation**: Comprehensive rustdoc für alle öffentlichen APIs

### Technisch

- Neues `import`-Modul mit Untermodulen für JSON, TOML, CSV
- `ImportStats`-Struct für Import-Reporting
- ID-Mapping bei Importen für korrekte Referenzverknüpfung
- Tests für alle Import-Funktionen

---

## [2.0.0] - 2024-12-01

### Hinzugefügt

- **Terminal UI**: Vollständige TUI mit ratatui
- **Dashboard**: Echtzeit-Übersicht mit Statistiken
- **Verleih-Management**: Such-, Listen-, Verlängerungs- und Rückgabe-Funktionen
- **Finanz-Tracking**: Zahlungsverfolgung und Schuldner-Übersicht
- **Verwaltung**: Schließfach- und Standortverwaltung
- **Export**: JSON, TOML, CSV, Markdown Export
- **Import**: JSON und TOML Import
- **SQLite-Datenbank**: Lokale Datenspeicherung
- **Audit-Logging**: Änderungsverfolgung

### Technisch

- Rust-basierte Implementierung
- Cross-platform Support (Linux, macOS, Windows)
- Automatische Datenbank-Migrationen
- Umfangreiche Test-Suite

---

## [1.0.0] - 2024-06-01

### Hinzugefügt

- Initiale Version
- Grundlegende Schließfach-Verwaltung
- Kommandozeilen-Interface
- SQLite-Speicherung

---

## Versionshinweise

### Upgrade von 2.0 auf 2.1

Das Upgrade ist vollständig abwärtskompatibel:

1. Bestehende Datenbanken werden automatisch migriert
2. Bestehende Export-Dateien bleiben lesbar
3. Keine manuellen Schritte erforderlich

**Neue Features nutzen:**
- 3× Escape für schnelle Dashboard-Navigation ausprobieren
- Screensaver-Timeout in Einstellungen anpassen
- CSV-Import für Massendaten-Import nutzen

### Migration von 1.0 auf 2.0

Bei Upgrade von Version 1.0:

1. Backup der bestehenden Datenbank erstellen
2. Neue Version installieren
3. Automatische Migration beim ersten Start
4. Neue TUI-Oberfläche nutzen

---

## Geplante Features

Für zukünftige Versionen geplant:

- [ ] Multi-User-Support
- [ ] Netzwerk-Synchronisation
- [ ] E-Mail-Benachrichtigungen
- [ ] Statistik-Dashboard mit Grafiken
- [ ] Mobile-App-Companion
- [ ] API-Server-Modus
