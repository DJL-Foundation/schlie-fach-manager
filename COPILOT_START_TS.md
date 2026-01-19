# 🚀 Nachricht an Copilot – TypeScript Implementation Start

## 📋 Aufgabe

Implementiere den **Schließfach-Manager v2.1** komplett in **TypeScript** basierend auf der **ts-spec.md**.

## 📖 Pflichtlektüre

Lies dir die **ts-spec.md** vollständig und gründlich durch, bevor du beginnst!

## 🎯 Technologie-Stack

- **TypeScript 5.3+** – Typsicherheit und moderne Features
- **OpenTUI** – Terminal UI Framework für Node.js/Bun
- **Bun** – Runtime und Build-Tool (`bun build --compile`)
- **better-sqlite3** – SQLite-Datenbank
- **Zod** – Runtime Schema Validation
- **date-fns** – Datumsverarbeitung

## 🏗️ Projekt-Setup

```bash
# Initialisierung
bun init

# Dependencies installieren
bun add @opentui/core better-sqlite3 date-fns zod
bun add -d @types/better-sqlite3 @types/bun typescript

# Projektstruktur erstellen (siehe ts-spec.md Abschnitt 2.2)
mkdir -p src/{db,ui/{screens,widgets},workflows,export,import,screensaver,types}
mkdir -p tests/{integration,ui,helpers}
mkdir -p docs
```

## ✅ Implementierungs-Checkliste

### **Phase 1: Basis-Setup** ⭐ KRITISCH

- [ ] `package.json` mit allen Dependencies
- [ ] `tsconfig.json` (siehe ts-spec.md 2.3)
- [ ] `bunfig.toml` (siehe ts-spec.md 2.4)
- [ ] Projektstruktur (alle Verzeichnisse)
- [ ] `src/types/` – Alle TypeScript Interfaces
- [ ] `src/db/schema.ts` – Zod Schemas
- [ ] `src/db/connection.ts` – Database Connection
- [ ] `src/db/migrations.ts` – Schema Migrations
- [ ] **Test:** Datenbank erstellen und migrieren

### **Phase 2: UI Foundation** ⭐ KRITISCH

- [ ] `src/ui/theme.ts` – Theme System mit allen Farben
- [ ] `src/ui/index.ts` – Layout-Berechnung
- [ ] OpenTUI Terminal Setup
- [ ] **Test:** Theme laden und Layout berechnen

### **Phase 3: Globale UI-Komponenten** ⭐ KRITISCH

- [ ] `src/ui/widgets/header.ts` – Header mit Window Switcher
  - [ ] Basis-Header (Titel + Screen-Name)
  - [ ] Window Switcher (3-Part Preview)
  - [ ] Tab/Shift+Tab Navigation
  - [ ] Enter/Escape Handling
- [ ] `src/ui/widgets/keybind-bar.ts` – Globale + Context Keybinds
  - [ ] Zweizeilig (Global | Context)
  - [ ] Farbcodierung
  - [ ] Auto-Truncate bei zu kleinem Terminal
- [ ] `src/ui/widgets/status-bar.ts` – Status + Escape Indicator
  - [ ] Status-Nachrichten (Info/Success/Warning/Error)
  - [ ] Escape Counter (0-3)
  - [ ] Visual Indicator `[│││]` rechts
  - [ ] Auto-Reset nach 1 Sekunde
- [ ] **Test:** Alle Widgets einzeln rendern

### **Phase 4: 3x Escape Feature** ⭐ KRITISCH

- [ ] Escape Counter in StatusBar
- [ ] Visual Indicator Animation (pipes füllen sich)
- [ ] Auto-Reset nach 1s Inaktivität
- [ ] Navigation zu Dashboard bei 3. Escape
- [ ] Reset bei anderer Taste
- [ ] **Test:** Triple-Escape Flow vollständig

### **Phase 5: Dashboard** ⭐ KRITISCH

- [ ] `src/ui/screens/dashboard.ts`
- [ ] `loadDashboardData()` – Alle DB-Queries
- [ ] Fixed-Height Layout (siehe ts-spec.md 6.1)
  - [ ] Übersicht: 5 Zeilen (fixed)
  - [ ] Statistiken: 8 Zeilen (fixed)
  - [ ] Alarme: 4 Zeilen (fixed)
  - [ ] Graph: Rest (flexible, min 10)
- [ ] Statistiken nach Größe und Standort
- [ ] Alarme (überfällig, bald fällig)
- [ ] Trend-Graph (ASCII Bars)
- [ ] Keybinds: 1-4 für Navigation
- [ ] **Test:** Dashboard mit Testdaten rendern

### **Phase 6: Screensaver** 🎨

- [ ] `src/screensaver/animations.ts` – Animation Interface
- [ ] 5 Animationen implementieren:
  - [ ] SpinningClock (🕐🕑🕒...)
  - [ ] BouncingBox (Box springt in Border)
  - [ ] MatrixRain (Fallende 0/1)
  - [ ] LoadingSpinner (⠋⠙⠹...)
  - [ ] WavingText (Text wellenförmig)
- [ ] `AnimationPlayer` – Frame-Management
- [ ] `getRandomAnimation()` – Zufällige Auswahl
- [ ] `src/ui/screens/screensaver.ts` – Fullscreen Rendering
- [ ] Inaktivitäts-Tracking in App
  - [ ] `lastActivity` Timestamp
  - [ ] 60s Timeout (aus Settings)
  - [ ] 15s Countdown in Status Bar
  - [ ] Aktivierung bei Timeout
  - [ ] Deaktivierung bei beliebiger Taste
  - [ ] **Exit direkt zum Dashboard** (nicht vorheriger Screen!)
- [ ] **Test:** Screensaver nach Timeout, Exit auf Taste

### **Phase 7: Window Switcher** 🪟

- [ ] Aktivierung mit `^` (Circumflex)
- [ ] 3-Part Preview (prev | **current** | next)
- [ ] Tab/Shift+Tab Navigation
- [ ] Enter = Wechseln
- [ ] Escape = Abbrechen
- [ ] Header zeigt Anleitung unten
- [ ] **Test:** Vollständiger Navigation-Flow

### **Phase 8: Wizard System** 🧙

- [ ] `src/ui/widgets/wizard.ts`
- [ ] `WizardMessage` Interface (System/User)
- [ ] `MessageContent` (Question/Answer/Info)
- [ ] `WizardOption` mit Label/Value/Metadata
- [ ] `WizardRenderer` – Chat-Style Rendering
  - [ ] Nachrichten in Boxen
  - [ ] System (🖥) vs User (👤)
  - [ ] Scrollbare Historie
  - [ ] Optionen als selectable Liste
- [ ] Keyboard Navigation (↑↓, Enter, Esc)
- [ ] **Test:** Einfacher Wizard-Flow

### **Phase 9: Workflows** 🔄

- [ ] `src/workflows/rent.ts` – Verleih-Wizard
- [ ] `src/workflows/extend.ts` – Verlängerungs-Wizard
- [ ] `src/workflows/return-locker.ts` – Rückgabe-Wizard
- [ ] `src/workflows/damage.ts` – Schadensmeldung
- [ ] Alle Wizards verwenden WizardRenderer
- [ ] DB-Integration (Insert/Update)
- [ ] Audit-Logging
- [ ] **Test:** Jeder Workflow End-to-End

### **Phase 10: Weitere Screens** 📺

- [ ] `src/ui/screens/rental-management.ts` – Tabs mit Wizards
- [ ] `src/ui/screens/finances.ts` – Zahlungsübersicht
- [ ] `src/ui/screens/management.ts` – Verwaltung + Settings
- [ ] Navigation zwischen Screens
- [ ] Context-Keybinds pro Screen
- [ ] **Test:** Alle Screens navigierbar

### **Phase 11: Export/Import** 💾

- [ ] `src/export/toml.ts` – TOML Export
- [ ] `src/import/toml.ts` – TOML Import
- [ ] `src/export/json.ts` – JSON Export
- [ ] `src/import/json.ts` – JSON Import
- [ ] `src/export/csv.ts` – CSV Export
- [ ] `src/export/markdown.ts` – Markdown Reports
- [ ] UI für Format-Auswahl
- [ ] **Test:** Round-Trip (Export → Import)

### **Phase 12: Audit & History** 📊

- [ ] `src/db/audit.ts` – Audit Logging
- [ ] `logAction()` in alle Workflows integrieren
- [ ] Audit Log Viewer (Screen)
- [ ] `src/db/history.ts` – Occupancy History
- [ ] Snapshot-Job (täglich)
- [ ] **Test:** Audit Logs korrekt geschrieben

### **Phase 13: App & Main Loop** 🎮

- [ ] `src/app.ts` – Application State
  - [ ] Screen Management
  - [ ] Window Switcher State
  - [ ] Screensaver State
  - [ ] Event Handling
- [ ] `src/index.ts` – Main Entry Point
  - [ ] Terminal Setup
  - [ ] Event Loop (60 FPS)
  - [ ] Keyboard Input
  - [ ] Render Loop
  - [ ] Cleanup
- [ ] **Test:** Komplette App startet und läuft

### **Phase 14: Tests** 🧪

- [ ] `tests/helpers/test-utils.ts` – Test Database Setup
- [ ] Unit Tests:
  - [ ] DB Queries
  - [ ] Workflows
  - [ ] Export/Import
  - [ ] Animationen
- [ ] Integration Tests:
  - [ ] Rental Lifecycle
  - [ ] Audit Logging
  - [ ] Settings Persistence
- [ ] UI Tests:
  - [ ] Triple Escape
  - [ ] Window Switcher
  - [ ] Screensaver
  - [ ] Wizard Navigation
- [ ] **Ziel: >85% Coverage**

### **Phase 15: Build & Docs** 📦

- [ ] `bun build --compile` konfigurieren
- [ ] Cross-platform Build testen
- [ ] `docs/README.md` – Installation & Features
- [ ] `docs/USER_GUIDE.md` – Bedienungsanleitung
- [ ] `docs/KEYBINDINGS.md` – Alle Tastenkombinationen
- [ ] `docs/CHANGELOG.md` – Version History
- [ ] Inline-Dokumentation (TSDoc)
- [ ] **Deliverable:** Standalone Binary (~50-70MB)

## 🚨 WICHTIGE REGELN

### ✅ DO:

1. **Lies die gesamte ts-spec.md VOLLSTÄNDIG**
2. **Arbeite Phase für Phase** – keine Sprünge!
3. **Teste nach jedem Schritt** – `bun test`
4. **Typsicherheit:** Alle Functions mit expliziten Types
5. **Zod Schemas:** Runtime-Validierung für DB-Daten
6. **Error Handling:** Try-Catch für alle DB-Operationen
7. **Dokumentation:** TSDoc für alle Public Functions
8. **Git Commits:** Nach jeder abgeschlossenen Phase

### ❌ DON'T:

1. ❌ **KEINE Placeholders** (`// TODO`, `// FIXME`)
2. ❌ **KEINE halbfertigen Implementierungen**
3. ❌ **KEINE Tests überspringen**
4. ❌ **NICHT von Spec abweichen** ohne Rücksprache
5. ❌ **KEINE `any` Types** (außer absolut notwendig)
6. ❌ **KEIN console.log** im Production Code

## 🎨 Code-Style

```typescript
// ✅ Gut: Explizite Types, Zod Validation
export interface DashboardData {
  totalLockers: number;
  occupiedLockers: number;
  occupancyPercent: number;
}

export function loadDashboardData(): DashboardData {
  const db = DatabaseConnection.getConnection();
  const result = db.prepare('SELECT COUNT(*) as count FROM lockers').get();
  const validated = z.object({ count: z.number() }).parse(result);
  
  return {
    totalLockers: validated.count,
    occupiedLockers: 0, // TODO query
    occupancyPercent: 0
  };
}

// ❌ Schlecht: any, keine Validation
export function loadData(): any {
  const db = getDb();
  return db.prepare('SELECT * FROM lockers').all();
}
```

## 🔍 Verifikation nach jeder Phase

```bash
# TypeScript Check
bun run typecheck

# Tests
bun test

# Build
bun build src/index.ts --compile --outfile dist/app

# Manual Test
./dist/app
```

## 📝 Commit-Messages

```
feat(ui): implement header with window switcher

- Add Header component with basic rendering
- Implement 3-part window preview
- Add Tab/Shift+Tab navigation
- Add Enter/Escape handling

Covers: ts-spec.md Phase 3 (Header)
Tests: All passing
```

## 🎯 Definition of Done

Eine Phase ist **FERTIG** wenn:

- [ ] Code kompiliert ohne Errors
- [ ] `bun test` – alle Tests grün
- [ ] `bun run typecheck` – keine Type Errors
- [ ] Keine `TODO`/`FIXME` Kommentare
- [ ] TSDoc Dokumentation vorhanden
- [ ] Funktionalität = Spec
- [ ] Git Commit mit Mapping zur Spec

## 🚀 Los geht's!

**Starte mit Phase 1: Basis-Setup**

1. Erstelle `package.json`
2. Erstelle `tsconfig.json`
3. Erstelle `bunfig.toml`
4. Installiere Dependencies: `bun install`
5. Erstelle Projektstruktur
6. Implementiere `src/types/index.ts`
7. Implementiere `src/db/schema.ts`
8. Implementiere `src/db/connection.ts`
9. Implementiere `src/db/migrations.ts`
10. Teste: Datenbank erstellen und migrieren

**Viel Erfolg! 🎉**

Bei Fragen oder Unklarheiten: **STOPP** und frage nach!