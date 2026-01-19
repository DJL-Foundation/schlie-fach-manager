# Changelog

Alle nennenswerten Änderungen an diesem Projekt werden in dieser Datei dokumentiert.

Das Format basiert auf [Keep a Changelog](https://keepachangelog.com/de/1.0.0/),
und dieses Projekt folgt [Semantic Versioning](https://semver.org/lang/de/).

## [2.1.0] - 2026-01-19

### Hinzugefügt

- **TypeScript-Implementierung** - Vollständige Neuimplementierung in TypeScript
- **Bun Runtime** - Native SQLite-Unterstützung mit `bun:sqlite`
- **Zod Validation** - Runtime Schema-Validierung für alle Datenbankobjekte

#### UI-Komponenten
- Header mit Window Switcher (3-Teil-Vorschau)
- Keybind-Bar mit globalen und kontextspezifischen Tastenkürzeln
- Status-Bar mit Escape-Indikator `[│││]`
- Wizard-System für Dialog-basierte Interaktionen

#### Screens
- Dashboard mit Übersicht, Statistiken, Alarmen und Trend-Graph
- Verleihverwaltung mit Tabs (Aktiv, Überfällig, Bald fällig)
- Finanz-Screen mit Einnahmenübersicht
- Verwaltungs-Screen mit Systemstatistiken

#### Features
- **Triple-Escape** - Dreimal Escape drücken für Dashboard
- **Window Switcher** - Mit `^` zwischen Screens wechseln
- **Screensaver** - 5 Animationen nach 60 Sekunden Inaktivität
  - Spinning Clock
  - Bouncing Box
  - Matrix Rain
  - Loading Spinner
  - Waving Text

#### Workflows
- Verleih-Wizard (Neuer Verleih)
- Verlängerungs-Wizard
- Rückgabe-Wizard
- Schadensmeldungs-Wizard
- Bulk-Erstellungs-Wizard

#### Export/Import
- JSON Export/Import mit Validierung
- CSV Export (Schließfächer, Verleih, Audit-Log)
- TOML Export/Import
- Markdown-Berichte (Vollbericht, Kurzübersicht)

#### Datenbank
- SQLite mit WAL-Modus
- 3-stufige Schema-Migration
- Audit-Logging für alle Aktionen
- Belegungsverlauf für Trendanalyse

#### Tests
- 58 automatisierte Tests
- Abdeckung für DB, UI, Workflows und Export/Import

### Technologie-Stack

- TypeScript 5.3+
- Bun 1.0+
- bun:sqlite (native SQLite)
- Zod (Schema-Validierung)
- date-fns (Datums-Handling)

## [2.0.0] - Vorversion

Vorherige Implementierung (nicht in TypeScript).

---

Vollständige Commit-Historie auf GitHub.
