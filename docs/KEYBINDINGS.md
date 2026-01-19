# Tastenkombinationen / Keybindings

Vollständige Übersicht aller Tastenkombinationen im Schließfach-Manager v2.1.

## Globale Keybinds (immer verfügbar)

Diese Tastenkombinationen funktionieren in allen Bildschirmen und Modi.

| Taste | Aktion | Beschreibung |
|-------|--------|--------------|
| `Tab` | Nächster Eintrag | Wechselt zum nächsten Feld, Tab oder Listeneintrag |
| `Shift+Tab` | Vorheriger Eintrag | Wechselt zum vorherigen Feld, Tab oder Listeneintrag |
| `^` (Caret) | Window Switcher | Öffnet den Fenster-Umschalter |
| `Shift+Q` | Programm beenden | Beendet die Anwendung sicher |
| `Esc` | Abbrechen/Zurück | Schließt Dialoge, bricht Aktionen ab |
| `Esc` ×3 | **Direkt zum Dashboard** | 3× Escape innerhalb 1 Sekunde → Dashboard |
| `Enter` | Bestätigen | Bestätigt Auswahl oder führt Aktion aus |

## Escape-Indikator `[|||]`

Der Escape-Indikator befindet sich rechts unten in der Status-Bar.

### Funktionsweise

```
[|||]  ← Alle Pipes inaktiv (grau)
[|··]  ← 1× Escape gedrückt
[||·]  ← 2× Escape gedrückt  
[|||]  ← 3× Escape → Sprung zum Dashboard!
```

- **Visuelle Rückmeldung**: Die Pipes `|` werden nacheinander hervorgehoben (gelb)
- **Zeitfenster**: Die Escapes müssen innerhalb von 1 Sekunde gedrückt werden
- **Auto-Reset**: Nach 1 Sekunde Inaktivität oder bei anderer Taste wird der Zähler zurückgesetzt

### Anwendungsfall

Der 3×-Escape-Mechanismus ermöglicht schnelle Navigation zurück zum Dashboard aus jedem Bildschirm, 
ohne mehrfach durch Menüs navigieren zu müssen.

## Dashboard-Shortcuts

Vom Dashboard aus können Sie schnell in verschiedene Bereiche springen.

| Taste | Aktion | Ziel |
|-------|--------|------|
| `1` | Neuen Verleih suchen | Verleih-Management → Suche |
| `2` | Verleihliste anzeigen | Verleih-Management → Liste |
| `3` | Vertrag verlängern | Verleih-Management → Verlängern |
| `4` | Schließfach zurückgeben | Verleih-Management → Rückgabe |
| `5` | Defekt melden | Verleih-Management → Defekt |

## Window Switcher (`^`)

Der Window Switcher ermöglicht schnelles Wechseln zwischen Hauptbildschirmen.

### Aktivierung

Drücken Sie `^` (Caret-Taste, meist `Shift+6`) um den Window Switcher zu öffnen.

### Navigation im Window Switcher

| Taste | Aktion |
|-------|--------|
| `Tab` | Nächstes Fenster |
| `Shift+Tab` | Vorheriges Fenster |
| `Enter` | Fenster auswählen und wechseln |
| `Esc` | Abbrechen, zurück zum vorherigen Bildschirm |

### Verfügbare Fenster (zyklisch)

1. **Dashboard** - Hauptübersicht
2. **Verleih-Management** - Verleihvorgänge
3. **Finanzen** - Zahlungen und Schulden
4. **Verwaltung** - Schließfächer, Standorte, Einstellungen

## Screensaver

Der Screensaver aktiviert sich automatisch nach Inaktivität.

### Timing

- **Inaktivitäts-Timeout**: 60 Sekunden (konfigurierbar in Einstellungen)
- **Countdown**: 15 Sekunden Vorwarnung in der Status-Bar
- **Aktivierung**: Nach Countdown ohne Eingabe

### Deaktivierung

- **Beliebige Taste** beendet den Screensaver sofort
- **Rückkehr**: Immer direkt zum **Dashboard** (nicht zum vorherigen Bildschirm)
- **Status-Nachricht**: Die vorherige Status-Bar-Nachricht wird wiederhergestellt

### Animationen

5 verschiedene ASCII-Animationen werden zufällig ausgewählt:
- Spinning Clock (Uhr)
- Bouncing Box
- Matrix Rain
- Loading Spinner
- Waving Text

## Navigations-Tasten

Diese Tasten funktionieren in Listen und Tabellen.

| Taste | Aktion |
|-------|--------|
| `↑` / `k` | Nach oben |
| `↓` / `j` | Nach unten |
| `←` / `h` | Nach links / Vorheriger Tab |
| `→` / `l` | Nach rechts / Nächster Tab |
| `Home` | Zum Anfang |
| `End` | Zum Ende |
| `Page Up` | Seite hoch |
| `Page Down` | Seite runter |

## Wizard-Keybinds

In Dialog-Wizards (z.B. Neuer Verleih, Verlängern, Rückgabe).

| Taste | Aktion |
|-------|--------|
| `↑` / `↓` | Option auswählen |
| `Enter` | Auswahl bestätigen |
| `Space` | Multi-Select (wo verfügbar) |
| `Esc` | Wizard abbrechen |
| `Backspace` | Zum vorherigen Schritt |

## Bearbeitungs-Keybinds

In Texteingabefeldern.

| Taste | Aktion |
|-------|--------|
| Buchstaben/Zahlen | Text eingeben |
| `Backspace` | Zeichen löschen |
| `Delete` | Zeichen rechts löschen |
| `Home` | Zum Anfang des Feldes |
| `End` | Zum Ende des Feldes |
| `Enter` | Eingabe bestätigen |
| `Esc` | Eingabe abbrechen |

## Finanz-Bildschirm

Zusätzliche Keybinds im Finanz-Bereich.

| Taste | Aktion |
|-------|--------|
| `E` | Export starten |
| `I` | Import starten |
| `F` | Filter anwenden |
| `R` | Daten aktualisieren |

## Verwaltungs-Bildschirm

Zusätzliche Keybinds im Verwaltungs-Bereich.

| Taste | Aktion |
|-------|--------|
| `N` | Neues Element erstellen |
| `D` | Element löschen (mit Bestätigung) |
| `E` | Element bearbeiten |
| `B` | Bulk-Erstellung |

## Tipps & Tricks

### Schnelle Navigation

1. **3× Escape**: Von überall zum Dashboard
2. **Window Switcher (`^`)**: Schnell zwischen Hauptbereichen wechseln
3. **Zifferntasten 1-5**: Vom Dashboard direkt in Funktionen springen

### Produktivität

- Nutzen Sie `Tab` und `Shift+Tab` für konsistente Navigation
- Der Escape-Indikator hilft beim Timing für 3×-Escape
- Screensaver-Exit führt immer zum Dashboard für einen frischen Start

### Vim-Style Navigation

Die Tasten `h`, `j`, `k`, `l` funktionieren als Alternative zu den Pfeiltasten 
für Benutzer, die Vim-Shortcuts gewohnt sind.
