# Schließfach-Manager v2.1

Terminal-basierte Verwaltungssoftware für Schließfächer, Verleihvorgänge und Finanzen.

## Features

- ✅ Dashboard mit Fixed-Height Layout und Trend-Graph
- ✅ Verleih-Management mit Dialog-Wizards
- ✅ Finanzübersicht und Zahlungsverlauf
- ✅ Standort- und Einstellungen-Verwaltung
- ✅ Export/Import (TOML, JSON, CSV, Markdown)
- ✅ Konfigurierbare Preise, Billing Periods, Screensaver-Timeout
- ✅ Audit-Logging und Belegungs-Historie
- ✅ Window Switcher und globale Keybind-Bar
- ✅ 3x Escape → Dashboard
- ✅ Screensaver mit ASCII-Animationen

## Installation

```bash
git clone https://github.com/DJL-Foundation/schlie-fach-manager.git
cd schlie-fach-manager
cargo build --release
./target/release/schliessfach-manager
```

## Tastenkombinationen

### Globale Keybinds
- `Tab` / `Shift+Tab` – Navigation
- `^` – Window Switcher
- `Shift+Q` – Beenden
- `Esc` – Zurück/Abbrechen (**3x = Dashboard**)
- `Enter` – Bestätigen

### Dashboard
- `1` – Neuen Verleih suchen
- `2` – Verleihliste
- `3` – Vertrag verlängern
- `4` – Rückgabe
- `5` – Defekt melden

## Screensaver

Nach 60 Sekunden Inaktivität (konfigurierbar):
- 15 Sekunden Countdown
- Automatische Aktivierung
- 5 verschiedene ASCII-Animationen
- Jede Taste deaktiviert → **Direkt zum Dashboard**

Weitere Details findest du in den Dokumenten unter `docs/`.
