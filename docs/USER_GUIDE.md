# Benutzerhandbuch - Schließfach-Manager v2.1

## Einführung

Der Schließfach-Manager ist eine Terminal-basierte Anwendung zur Verwaltung von Schließfächern. Diese Anleitung führt Sie durch alle Funktionen der Anwendung.

## Erste Schritte

### Anwendung starten

```bash
bun run dev
```

Beim ersten Start wird automatisch eine SQLite-Datenbank erstellt.

### Interface-Aufbau

Das Interface besteht aus vier Bereichen:

1. **Header** - Zeigt den Titel und aktuellen Screen
2. **Inhalt** - Hauptbereich für den aktuellen Screen
3. **Keybind-Bar** - Zeigt verfügbare Tastenkürzel
4. **Status-Bar** - Statusmeldungen und Escape-Indikator

## Screens

### Dashboard

Das Dashboard zeigt eine Übersicht aller wichtigen Informationen:

- **Übersicht** - Gesamtzahl und Belegung der Schließfächer
- **Statistiken** - Aufschlüsselung nach Größe und Standort
- **Alarme** - Überfällige und bald fällige Verleih
- **Trend-Graph** - Belegungsentwicklung der letzten 30 Tage

**Tastenkürzel:**
- `1` - Zur Verleihverwaltung
- `2` - Zu den Finanzen
- `3` - Zur Verwaltung
- `r` - Dashboard aktualisieren

### Verleihverwaltung

Verwalten Sie hier alle aktiven Verleih:

**Tabs:**
- **Aktive Verleih** - Alle laufenden Vermietungen
- **Überfällig** - Verleih nach Ablaufdatum
- **Bald fällig** - In den nächsten 30 Tagen ablaufend

**Tastenkürzel:**
- `n` - Neuer Verleih (Wizard)
- `e` - Verlängern (Wizard)
- `r` - Rückgabe (Wizard)
- `d` - Schaden melden (Wizard)
- `↑`/`↓` oder `k`/`j` - Navigation in der Liste

### Finanzen

Übersicht über alle Zahlungen:

- Einnahmen der letzten 7, 30 und 365 Tage
- Aufschlüsselung nach Zahlungsart
- Offene Pfandbeträge

**Tastenkürzel:**
- `p` - Neue Zahlung erfassen
- `e` - Export

### Verwaltung

Systemeinstellungen und Datenmanagement:

- **Standorte** - Standorte verwalten
- **Einstellungen** - Systemkonfiguration
- **Audit-Log** - Protokoll aller Aktionen
- **Import** - Daten importieren
- **Export** - Daten exportieren

**Tastenkürzel:**
- `l` - Standorte
- `s` - Einstellungen
- `a` - Audit-Log anzeigen
- `i` - Import
- `x` - Export

## Wizards

### Verleih-Wizard

1. **Standort auswählen** - Wählen Sie den Standort
2. **Größe auswählen** - Wählen Sie die Schließfachgröße
3. **Schließfach auswählen** - Wählen Sie ein freies Fach
4. **Mietdauer** - 1, 3, 6 oder 12 Monate
5. **Pfand** - Wurde das Pfand bezahlt?
6. **Zahlungsart** - Falls Pfand bezahlt
7. **Bestätigung** - Zusammenfassung und Abschluss

### Verlängerungs-Wizard

1. **Verleih auswählen** - Aktiven Verleih wählen
2. **Zeitraum** - Verlängerungsdauer wählen
3. **Bestätigung** - Neue Laufzeit bestätigen

### Rückgabe-Wizard

1. **Verleih auswählen** - Zu beendenden Verleih wählen
2. **Pfand** - Pfand zurückgeben?
3. **Schäden** - Wurden Schäden festgestellt?
4. **Bestätigung** - Rückgabe abschließen

### Schadensmeldung

1. **Schließfach auswählen** - Betroffenes Fach wählen
2. **Aktion** - Schaden melden oder als repariert markieren
3. **Bestätigung** - Aktion bestätigen

## Globale Navigation

### Window Switcher

Drücken Sie `^` (Circumflex/Zirkumflex) um den Window Switcher zu öffnen:

- **Tab** - Nächstes Fenster
- **Shift+Tab** - Vorheriges Fenster
- **Enter** - Fenster auswählen
- **Esc** - Abbrechen

### Triple-Escape

Drücken Sie dreimal `Esc` um sofort zum Dashboard zurückzukehren.
Der Escape-Indikator `[│││]` in der Statusleiste zeigt den Fortschritt.

### Screensaver

Nach 60 Sekunden Inaktivität wird ein Screensaver aktiviert:

- 15 Sekunden vor Aktivierung erscheint ein Countdown
- Beliebige Taste drücken um zu beenden
- Nach dem Screensaver: Direkt zum Dashboard

## Export-Formate

### JSON

Vollständiger Export aller Daten im JSON-Format.
Geeignet für Backups und Datenaustausch.

### CSV

Tabellarische Exporte (Excel-kompatibel):
- Schließfächer
- Aktive Verleih
- Überfällige Verleih
- Audit-Log

### TOML

Konfigurationsformat für Einstellungen.

### Markdown

Lesbare Berichte für Dokumentation:
- Vollständiger Bericht
- Kurzübersicht

## Tastenkürzel-Übersicht

| Taste | Funktion |
|-------|----------|
| `^` | Window Switcher |
| `Esc×3` | Dashboard |
| `q` | Beenden |
| `↑`/`↓` oder `k`/`j` | Liste navigieren |
| `Enter` | Auswählen |
| `Tab` | Nächstes Element |
| `Shift+Tab` | Vorheriges Element |

## Tipps

1. **Regelmäßige Exports** - Erstellen Sie regelmäßig JSON-Backups
2. **Audit-Log prüfen** - Überprüfen Sie das Protokoll auf ungewöhnliche Aktivitäten
3. **Schäden dokumentieren** - Melden Sie Schäden sofort für korrekte Pfandabwicklung
4. **Dashboard nutzen** - Behalten Sie überfällige Verleih im Blick

## Fehlerbehebung

### Datenbank-Fehler

Die Datenbank befindet sich unter:
- Linux: `~/.local/share/schliessfach-manager/data.db`
- macOS: `~/Library/Application Support/schliessfach-manager/data.db`
- Windows: `%APPDATA%/schliessfach-manager/data.db`

### Terminal-Probleme

Stellen Sie sicher, dass Ihr Terminal:
- ANSI-Farben unterstützt
- UTF-8 Zeichensatz verwendet
- Mindestens 80x24 Zeichen groß ist

---

Bei weiteren Fragen kontaktieren Sie uns auf GitHub.
