# Schließfach-Manager v2.1-Tauri - Benutzerhandbuch

## Inhaltsverzeichnis

1. [Einführung](#einführung)
2. [Installation](#installation)
3. [Erste Schritte](#erste-schritte)
4. [Dashboard](#dashboard)
5. [Verleih-Management](#verleih-management)
6. [Schließfach-Verwaltung](#schließfach-verwaltung)
7. [Einstellungen](#einstellungen)
8. [Export & Import](#export--import)
9. [Tastaturkürzel](#tastaturkürzel)
10. [Fehlerbehebung](#fehlerbehebung)

---

## Einführung

Der **Schließfach-Manager v2.1-Tauri** ist eine moderne Desktop-Anwendung zur Verwaltung von Schließfächern. Mit dieser Software können Sie:

- Schließfächer an verschiedenen Standorten verwalten
- Verleihe erstellen, verlängern und zurückgeben
- Zahlungen und Pfand verfolgen
- Überfällige Verleihe überwachen
- Daten exportieren und importieren

### Systemanforderungen

- **Windows:** Windows 10 oder neuer
- **macOS:** macOS 10.15 (Catalina) oder neuer
- **Linux:** Ubuntu 18.04 oder vergleichbar

---

## Installation

### Windows

1. Laden Sie die `.msi` Datei herunter
2. Führen Sie die Datei aus
3. Folgen Sie den Installationsanweisungen

### macOS

1. Laden Sie die `.dmg` Datei herunter
2. Öffnen Sie die Datei
3. Ziehen Sie die App in den Applications-Ordner

### Linux

1. Laden Sie die `.AppImage` oder `.deb` Datei herunter
2. Für AppImage: Rechtsklick → Eigenschaften → Ausführbar machen
3. Für .deb: `sudo dpkg -i schließfach-manager.deb`

---

## Erste Schritte

### Hauptfenster

Nach dem Start sehen Sie das Hauptfenster mit:

- **TopBar**: Logo, Breadcrumb-Navigation, Shortcuts-Hinweis
- **Sidebar**: Navigation zu allen Bereichen
- **Hauptbereich**: Inhalte je nach ausgewählter Seite
- **StatusBar**: DB-Status, Belegung, Uhrzeit

### Navigation

Die Sidebar ist in Kategorien unterteilt:

1. **Dashboard** - Übersicht und Statistiken
2. **Verleih** - Alle Verleih-Funktionen
3. **Finanzen** - Zahlungen und Berichte
4. **Verwaltung** - Schließfächer, Standorte, Einstellungen

---

## Dashboard

Das Dashboard zeigt Ihnen eine Übersicht:

### Kennzahlen

- **Belegung**: Prozentuale Auslastung aller Schließfächer
- **Umsatz (30 Tage)**: Einnahmen der letzten 30 Tage
- **Überfällig**: Anzahl überfälliger Verleihe
- **Auslaufend**: Verleihe, die in 7 Tagen enden

### Diagramme

- **Belegungstrend**: Entwicklung der Belegung über 30 Tage
- **Nach Größe**: Belegung aufgeschlüsselt nach Schließfachgröße
- **Nach Standort**: Belegung pro Standort

---

## Verleih-Management

### Neuen Verleih erstellen

1. Navigieren Sie zu **Verleih → Neu verleihen**
2. Geben Sie die Mieter-Informationen ein:
   - Name (Pflichtfeld)
   - E-Mail (optional)
   - Telefon (optional)
   - Notizen (optional)
3. Wählen Sie ein verfügbares Schließfach
4. Legen Sie Startdatum und Dauer fest
5. Bestätigen Sie die Pfandzahlung
6. Klicken Sie auf **Abschließen**

### Aktive Verleihe

Unter **Verleih → Aktive Verleihe** sehen Sie:

- Alle aktuell laufenden Verleihe
- Warnungen für bald ablaufende Verleihe
- Schnellaktionen: Verlängern, Rückgabe

### Überfällige Verleihe

Unter **Verleih → Überfällig** finden Sie:

- Alle überfälligen Verleihe mit Dringlichkeitsanzeige
- Kontaktinformationen der Mieter
- Schnellaktionen: Kontaktieren, Rückgabe

### Verleih verlängern

1. Gehen Sie zu **Verleih → Aktive Verleihe**
2. Klicken Sie bei einem Verleih auf **Verlängern**
3. Wählen Sie die Verlängerungsdauer
4. Bestätigen Sie die Verlängerung

### Rückgabe durchführen

1. Finden Sie den Verleih in der Liste
2. Klicken Sie auf **Rückgabe**
3. Bestätigen Sie die Pfandrückgabe
4. Der Verleih wird abgeschlossen

---

## Schließfach-Verwaltung

### Schließfächer verwalten

Unter **Verwaltung → Schließfächer** können Sie:

- Alle Schließfächer einsehen
- Nach Nummer oder Standort suchen
- Schließfächer erstellen, bearbeiten und löschen

### Neues Schließfach erstellen

1. Klicken Sie auf **Neues Schließfach**
2. Geben Sie ein:
   - Nummer (z.B. "A-001")
   - Standort (z.B. "Gebäude A")
   - Größe (S, M, L, XL)
   - Notizen (optional)
3. Klicken Sie auf **Erstellen**

### Schließfach bearbeiten

1. Klicken Sie auf das Bearbeiten-Symbol (Stift)
2. Ändern Sie die gewünschten Felder
3. Markieren Sie ggf. als beschädigt
4. Klicken Sie auf **Speichern**

### Schließfach löschen

1. Klicken Sie auf das Löschen-Symbol (Papierkorb)
2. Bestätigen Sie die Löschung

**Hinweis:** Schließfächer mit aktiven Verleihen können nicht gelöscht werden.

---

## Einstellungen

Unter **Verwaltung → Einstellungen** können Sie anpassen:

### Finanzielle Einstellungen

- **Pfandbetrag**: Standardpfand bei Neuverleihen
- **Jahresgebühr**: Jährliche Mietkosten
- **Berechnungszeitraum**: Monatlich oder jährlich
- **Währung**: EUR, CHF, USD

### Anzeige-Einstellungen

- **Screensaver-Timeout**: Zeit bis zur Bildschirmsperre

### Speichern

Klicken Sie auf **Speichern**, um Änderungen zu übernehmen.

---

## Export & Import

### Daten exportieren

1. Gehen Sie zu **Verwaltung → Export/Import**
2. Wählen Sie das Format:
   - **TOML**: Empfohlen für Backups
   - **JSON**: Für API-Kompatibilität
   - **CSV**: Für Excel-Import
3. Klicken Sie auf **Exportieren**
4. Wählen Sie den Speicherort

### Daten importieren

1. Klicken Sie auf **Datei auswählen**
2. Wählen Sie eine zuvor exportierte Datei
3. Bestätigen Sie den Import

**Warnung:** Der Import kann vorhandene Daten überschreiben!

---

## Tastaturkürzel

Drücken Sie `Strg + ?` um alle Kürzel anzuzeigen.

### Navigation

| Kürzel | Aktion |
|--------|--------|
| `Strg + H` | Dashboard |
| `Strg + N` | Neuer Verleih |
| `Strg + R` | Rückgabe |
| `Strg + ,` | Einstellungen |

### Global

| Kürzel | Aktion |
|--------|--------|
| `Strg + ?` | Shortcuts anzeigen |
| `Esc` | Dialog schließen |

### Tabellen

| Kürzel | Aktion |
|--------|--------|
| `↑` / `↓` | Zeile wechseln |
| `Enter` | Details öffnen |
| `Space` | Auswählen |

---

## Fehlerbehebung

### Die App startet nicht

1. Stellen Sie sicher, dass Ihr System die Mindestanforderungen erfüllt
2. Versuchen Sie, die App als Administrator zu starten
3. Prüfen Sie, ob alle Abhängigkeiten installiert sind

### Datenbankfehler

Die Datenbank befindet sich unter:
- **Windows:** `%APPDATA%\de.djl.schließfach-manager\`
- **macOS:** `~/Library/Application Support/de.djl.schließfach-manager/`
- **Linux:** `~/.local/share/schließfach-manager/`

Bei Problemen:
1. Erstellen Sie ein Backup der `lockers.db` Datei
2. Löschen Sie die Datei
3. Starten Sie die App neu (erstellt neue Datenbank)

### Fehlende Daten nach Update

1. Exportieren Sie Ihre Daten vor dem Update
2. Importieren Sie nach dem Update

### Sonstige Probleme

Bei weiteren Problemen:
1. Prüfen Sie die Console (F12 in der App)
2. Erstellen Sie ein Issue im GitHub-Repository

---

## Support

Bei Fragen oder Problemen:
- GitHub: [Issues erstellen](../../issues)
- E-Mail: support@djl-foundation.de

---

*Schließfach-Manager v2.1-Tauri - © DJL Foundation*
