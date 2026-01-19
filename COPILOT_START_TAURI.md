# Copilot Arbeitsanweisung: Schließfach-Manager v2.1-Tauri

**Projekt:** Schließfach-Manager v2.1-Tauri (Desktop GUI)  
**Basis:** v2.1 ratatui Spec → Tauri GUI Adaption  
**Tech-Stack:** React + TypeScript + Tailwind + Tauri (Rust Backend)  
**Design:** Catppuccin Mocha inspired  
**Status:** Implementierung  

---

## 📋 Übersicht

Du implementierst eine **moderne Desktop-Anwendung** zur Verwaltung von Schließfächern. Die gesamte Business-Logik und Datenstruktur stammt aus der bewährten v2.1-ratatui-Version. Deine Aufgabe ist die **GUI-Adaption mit Tauri**.

**Hauptdokumente:**
1. `v2.1-tauri.md` – Vollständige technische Spezifikation
2. `tauri-design-system.md` – Design-System & UI-Guidelines
3. Diese Datei – Deine tägliche Arbeitsanweisung

---

## 🎯 Arbeitsphilosophie

### ✅ DO
- **Spec-First:** Lies IMMER zuerst die relevanten Abschnitte in `v2.1-tauri.md`
- **Inkrementell:** Kleine, testbare Schritte statt große Refactorings
- **Type-Safety:** Nutze TypeScript und Zod für maximale Typsicherheit
- **Komponenten-Wiederverwendung:** DRY-Prinzip, baue auf bestehenden UI-Komponenten auf
- **Error Handling:** Alle Tauri-Commands und API-Calls mit try-catch + Toast-Feedback
- **Accessibility:** Semantic HTML, ARIA-Labels, Keyboard-Navigation
- **Performance:** React Query Caching, Lazy Loading für große Listen
- **Git:** Atomare Commits mit konventionellen Commit-Messages

### ❌ DON'T
- **Keine Spec-Abweichungen** ohne explizite Anfrage
- **Keine hartkodierten Werte** (nutze Settings/Config)
- **Keine inline-styles** (nutze Tailwind-Classes)
- **Keine `any` Types** (explizite Typisierung)
- **Keine Silent Failures** (immer Error-Feedback via Toast)
- **Keine Magic Numbers** (benenne Konstanten sinnvoll)
- **Keine God Components** (max. 200 Zeilen, sonst aufteilen)

---

## 📦 Implementierungs-Phasen

### Phase 1: Projekt-Setup ✨ (KRITISCH)

**Ziel:** Lauffähiges Tauri-Projekt mit Theme und Basis-Layout

#### 1.1 Initialisierung
```bash
# Tauri-Projekt erstellen
npm create tauri-app@latest schließfach-manager-tauri
# Template: React + TypeScript

cd schließfach-manager-tauri
npm install

# Dependencies
npm install @tanstack/react-query @tanstack/react-table \
  react-hook-form zod @hookform/resolvers date-fns \
  recharts lucide-react sonner clsx tailwind-merge \
  react-router-dom

npm install -D tailwindcss postcss autoprefixer \
  @types/react @types/react-dom vitest \
  @testing-library/react @testing-library/user-event
```

#### 1.2 Tailwind-Konfiguration
- `tailwind.config.js` gemäß `v2.1-tauri.md` Abschnitt 5.2
- `postcss.config.js` erstellen
- `src-ui/styles/theme.css` mit CSS-Variablen (Abschnitt 5.1)
- `src-ui/styles/globals.css` mit Tailwind-Imports

#### 1.3 Projekt-Struktur
```
src-ui/
├── components/
│   ├── layout/      # TopBar, Sidebar, StatusBar, MainLayout
│   ├── ui/          # Button, Input, Card, Table, Modal, Badge
│   ├── wizards/     # WizardLayout, RentWizard, etc.
│   └── charts/      # OccupancyChart, RevenueChart
├── pages/           # Dashboard, rentals/, finances/, management/
├── hooks/           # use-lockers, use-rentals, use-keyboard, etc.
├── lib/             # tauri.ts, utils.ts, cn.ts
├── types/           # Type definitions
├── schemas/         # Zod schemas
└── styles/          # theme.css, globals.css
```

#### 1.4 Rust Backend Setup
- `src-tauri/Cargo.toml` Dependencies gemäß `v2.1-tauri.md` Abschnitt 2.2
- `src/db/` Ordner: `connection.rs`, `migrations.rs`, `models.rs`
- `src/commands/` Ordner: `dashboard.rs`, `lockers.rs`, `rentals.rs`, etc.
- `src/services/` Ordner: Business-Logik-Layer

#### 1.5 Datenbank-Migration
- DB-Schema aus v2.1 übernehmen (Abschnitt 3)
- Migration-System implementieren (`run_migrations` Command)
- Test: Datenbank erstellen, Tabellen verifizieren

**Verifikation:**
- [ ] `npm run tauri dev` startet erfolgreich
- [ ] Theme-Farben sind sichtbar (Catppuccin Mocha)
- [ ] Datenbank wird erstellt unter `~/.local/share/schließfach-manager/`
- [ ] Console: Keine Errors

**Commit:**
```
feat(setup): Initialize Tauri project with theme and database

- Add Tailwind config with Catppuccin Mocha theme
- Setup project structure (components, pages, hooks)
- Implement database schema and migrations
- Add Rust dependencies (rusqlite, chrono, serde)
```

---

### Phase 2: Core Components 🧱 (HOCH)

**Ziel:** Wiederverwendbare UI-Komponenten nach Design-System

#### 2.1 Utility Functions
**Dateien:**
- `src-ui/lib/cn.ts` – Tailwind class merger (clsx + tailwind-merge)
- `src-ui/lib/utils.ts` – Hilfsfunktionen (formatDate, formatCurrency, etc.)

```typescript
// src-ui/lib/cn.ts
import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
```

#### 2.2 UI Components
**Implementiere in dieser Reihenfolge:**

1. **Button** (`src-ui/components/ui/Button.tsx`)
   - Variants: primary, secondary, outline, ghost, danger
   - Sizes: sm, md, lg, icon
   - Loading State mit Spinner
   - Vorlage: `v2.1-tauri.md` Abschnitt 7.1

2. **Input** (`src-ui/components/ui/Input.tsx`)
   - Label, Error, HelperText Support
   - Focus States, Error Styling
   - Vorlage: Abschnitt 7.3

3. **Select** (`src-ui/components/ui/Select.tsx`)
   - Dropdown mit Options-Array
   - Error Styling
   - Vorlage: Abschnitt 7.4

4. **Card** (`src-ui/components/ui/Card.tsx`)
   - Card, CardHeader, CardTitle, CardContent
   - Vorlage: Abschnitt 7.2

5. **Table** (`src-ui/components/ui/Table.tsx`)
   - Table, TableHeader, TableBody, TableRow, TableHead, TableCell
   - Hover States
   - Vorlage: Abschnitt 7.5

6. **Modal** (`src-ui/components/ui/Modal.tsx`)
   - Overlay, Close Button
   - Sizes: sm, md, lg, xl
   - Keyboard: Esc zum Schließen
   - Vorlage: Abschnitt 7.6

7. **Badge** (`src-ui/components/ui/Badge.tsx`)
   - Variants: default, success, warning, error, info
   - Vorlage: Abschnitt 7.7

**Verifikation:**
- [ ] Jede Komponente hat TypeScript-Interface für Props
- [ ] Styling entspricht Design-System (Catppuccin-Farben)
- [ ] Hover/Focus States funktionieren
- [ ] Alle Komponenten sind exportiert und importierbar

**Commit (pro Komponente):**
```
feat(ui): Add Button component with variants and states

- Implement primary, secondary, outline, ghost, danger variants
- Add sm, md, lg, icon sizes
- Include loading state with spinner animation
- Follow Catppuccin Mocha color scheme
```

---

### Phase 3: Layout & Navigation 🗺️ (HOCH)

**Ziel:** Funktionale Navigation mit Sidebar und TopBar

#### 3.1 TopBar
**Datei:** `src-ui/components/layout/TopBar.tsx`

- Logo (32×32px, abgerundet)
- App-Name: "Schließfach-Manager"
- Version-Badge: "v2.1-Tauri"
- Breadcrumb-Navigation (dynamisch basierend auf Route)
- Keyboard-Shortcuts-Hint (Strg+?)
- Vorlage: `v2.1-tauri.md` Abschnitt 6.1

#### 3.2 Sidebar
**Datei:** `src-ui/components/layout/Sidebar.tsx`

- Navigation-Items aus NAV_ITEMS Array
- Kategorien (Dashboard, Verleih, Finanzen, Verwaltung)
- Verschachtelte Seiten (Children)
- Active State Highlighting
- Collapse/Expand Funktion (240px ↔ 64px)
- Icons: Lucide React
- Vorlage: Abschnitt 6.2

**NAV_ITEMS:**
```typescript
[
  { label: 'Dashboard', icon: Home, path: '/' },
  {
    label: 'Verleih',
    icon: Archive,
    children: [
      { label: 'Übersicht', path: '/rentals/overview' },
      { label: 'Neu verleihen', path: '/rentals/new' },
      { label: 'Aktive Verleihe', path: '/rentals/active' },
      { label: 'Überfällig', path: '/rentals/overdue' },
      { label: 'Verlängern', path: '/rentals/extend' },
      { label: 'Rückgabe', path: '/rentals/return' },
      { label: 'Verlauf', path: '/rentals/history' },
    ],
  },
  // ... (siehe Spec Abschnitt 6.2)
]
```

#### 3.3 StatusBar
**Datei:** `src-ui/components/layout/StatusBar.tsx`

- DB-Status (Verbunden/Fehler)
- Belegungsstatistik (live)
- Uhr (aktualisiert jede Sekunde)
- Version
- Vorlage: Abschnitt 6.3

#### 3.4 MainLayout
**Datei:** `src-ui/components/layout/MainLayout.tsx`

- Kombiniert TopBar, Sidebar, Main Content (Outlet), StatusBar
- Toast-Container (Sonner)
- Vorlage: Abschnitt 6.4

#### 3.5 React Router Setup
**Datei:** `src-ui/App.tsx`

```typescript
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { MainLayout } from '@/components/layout/MainLayout';
import { Dashboard } from '@/pages/Dashboard';
// ... weitere Imports

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<MainLayout />}>
          <Route index element={<Dashboard />} />
          <Route path="rentals">
            <Route path="overview" element={<RentalsOverview />} />
            <Route path="new" element={<RentWizard />} />
            {/* ... */}
          </Route>
          {/* ... */}
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
```

**Verifikation:**
- [ ] Navigation funktioniert (Sidebar-Clicks ändern Route)
- [ ] Active State wird korrekt angezeigt
- [ ] TopBar zeigt Breadcrumb passend zur Route
- [ ] StatusBar zeigt Platzhalter-Daten
- [ ] Sidebar kollabiert/expandiert

**Commit:**
```
feat(layout): Implement main layout with navigation

- Add TopBar with logo, breadcrumb, and version badge
- Implement Sidebar with collapsible categories
- Add StatusBar with DB status, occupancy, and clock
- Setup MainLayout combining all layout components
- Configure React Router with route structure
```

---

### Phase 4: Tauri Backend & API 🦀 (KRITISCH)

**Ziel:** Funktionierende Rust Commands für Frontend-Integration

#### 4.1 Datenbank-Connection
**Datei:** `src/db/connection.rs`

```rust
use rusqlite::{Connection, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct DbConnection(pub Arc<Mutex<Connection>>);

pub fn establish_connection() -> Result<Connection> {
    let data_dir = directories::ProjectDirs::from("de", "djl", "schließfach-manager")
        .expect("Could not determine data directory");
    
    std::fs::create_dir_all(data_dir.data_dir())?;
    
    let db_path = data_dir.data_dir().join("lockers.db");
    Connection::open(db_path)
}
```

#### 4.2 Dashboard Command
**Datei:** `src/commands/dashboard.rs`

- Implementiere `get_dashboard_data` Command
- Alle Aggregationen aus DB holen
- Vorlage: `v2.1-tauri.md` Abschnitt 10.1
- **WICHTIG:** Error Handling mit `Result<T, String>`

#### 4.3 Locker Commands
**Datei:** `src/commands/lockers.rs`

Commands:
- `get_all_lockers() -> Result<Vec<Locker>, String>`
- `get_available_lockers() -> Result<Vec<Locker>, String>`
- `create_locker(input: CreateLockerInput) -> Result<i64, String>`
- `update_locker(input: UpdateLockerInput) -> Result<(), String>`
- `delete_locker(id: i64) -> Result<(), String>`

Vorlage: Abschnitt 10.2

#### 4.4 Rental Commands
**Datei:** `src/commands/rentals.rs`

Commands:
- `get_active_rentals() -> Result<Vec<Rental>, String>`
- `get_overdue_rentals() -> Result<Vec<Rental>, String>`
- `create_rental(input: CreateRentalInput) -> Result<i64, String>`
- `extend_rental(id: i64, months: i32) -> Result<(), String>`
- `return_rental(id: i64) -> Result<(), String>`

Vorlage: Abschnitt 10.3

#### 4.5 Tauri Main Setup
**Datei:** `src/main.rs`

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod services;
mod error;

use db::connection::{establish_connection, DbConnection};
use std::sync::Arc;
use tokio::sync::Mutex;

fn main() {
    let conn = establish_connection().expect("Failed to establish DB connection");
    
    // Run migrations
    db::migrations::run_migrations(&conn).expect("Failed to run migrations");
    
    let db = DbConnection(Arc::new(Mutex::new(conn)));
    
    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            commands::dashboard::get_dashboard_data,
            commands::dashboard::get_status_bar_data,
            commands::lockers::get_all_lockers,
            commands::lockers::get_available_lockers,
            commands::lockers::create_locker,
            commands::lockers::update_locker,
            commands::lockers::delete_locker,
            commands::rentals::get_active_rentals,
            commands::rentals::create_rental,
            // ... weitere Commands
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### 4.6 Frontend API Wrapper
**Datei:** `src-ui/lib/tauri.ts`

```typescript
import { invoke } from '@tauri-apps/api/core';

export async function getDashboardData(): Promise<DashboardData> {
  return invoke('get_dashboard_data');
}

export async function getAllLockers(): Promise<Locker[]> {
  return invoke('get_all_lockers');
}

export async function createLocker(input: CreateLockerInput): Promise<number> {
  return invoke('create_locker', { input });
}

// ... weitere Wrapper-Funktionen
```

**Verifikation:**
- [ ] `npm run tauri dev` kompiliert Rust ohne Errors
- [ ] Browser Console: Commands können aufgerufen werden
- [ ] Test: `window.__TAURI__.invoke('get_all_lockers')` gibt Daten zurück
- [ ] Datenbank wird mit Migrations erstellt

**Commit:**
```
feat(backend): Implement Tauri commands for lockers and rentals

- Add database connection and migration system
- Implement dashboard aggregation command
- Add CRUD commands for lockers
- Add rental management commands
- Create frontend API wrapper (tauri.ts)
```

---

### Phase 5: React Query Integration 🔄 (HOCH)

**Ziel:** Typsichere API-Calls mit Caching und Error Handling

#### 5.1 React Query Setup
**Datei:** `src-ui/main.tsx`

```typescript
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 30000, // 30s
      retry: 1,
    },
  },
});

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <App />
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  </React.StrictMode>
);
```

#### 5.2 Custom Hooks
**Datei:** `src-ui/hooks/use-lockers.ts`

```typescript
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import * as api from '@/lib/tauri';
import { toast } from 'sonner';

export function useLockers() {
  return useQuery({
    queryKey: ['lockers'],
    queryFn: api.getAllLockers,
  });
}

export function useAvailableLockers() {
  return useQuery({
    queryKey: ['lockers', 'available'],
    queryFn: api.getAvailableLockers,
  });
}

export function useCreateLocker() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: api.createLocker,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['lockers'] });
      toast.success('Schließfach erfolgreich erstellt');
    },
    onError: (error: any) => {
      toast.error(`Fehler: ${error}`);
    },
  });
}

// ... weitere Mutations (update, delete)
```

Erstelle analog:
- `src-ui/hooks/use-rentals.ts`
- `src-ui/hooks/use-dashboard.ts`
- `src-ui/hooks/use-payments.ts`
- `src-ui/hooks/use-settings.ts`

**Verifikation:**
- [ ] React Query DevTools zeigen Queries an
- [ ] Cache funktioniert (zweiter Aufruf nutzt Cache)
- [ ] Mutations invalidieren Queries korrekt
- [ ] Toast-Benachrichtigungen erscheinen bei Success/Error

**Commit:**
```
feat(hooks): Add React Query hooks for data fetching

- Setup QueryClient with default options
- Implement useLockers, useAvailableLockers, useCreateLocker
- Add useRentals, useDashboard hooks
- Include Toast notifications for mutations
- Add React Query DevTools for debugging
```

---

### Phase 6: Dashboard Implementation 📊 (HOCH)

**Ziel:** Funktionales Dashboard mit Live-Daten und Charts

#### 6.1 Dashboard Page
**Datei:** `src-ui/pages/Dashboard.tsx`

Komponenten:
1. **Page Header** (Titel + Beschreibung)
2. **Stats Grid** (4 Cards: Belegung, Umsatz, Überfällig, Auslaufend)
3. **Charts Row** (Occupancy Trend + By Size)
4. **By Location Grid**

Vorlage: `v2.1-tauri.md` Abschnitt 8.1

**Wichtig:**
- Nutze `useDashboard()` Hook
- Loading State während Datenabruf
- Error State bei Fehler

#### 6.2 Occupancy Chart
**Datei:** `src-ui/components/charts/OccupancyChart.tsx`

- Recharts LineChart
- 30 Tage History
- Vorlage: Abschnitt 8.2

#### 6.3 StatusBar mit Live-Daten
**Update:** `src-ui/components/layout/StatusBar.tsx`

- Nutze `useQuery` für `get_status_bar_data`
- RefetchInterval: 30s
- Live-Uhr mit `useState` + `setInterval`

**Verifikation:**
- [ ] Dashboard zeigt alle Stats korrekt
- [ ] Chart rendert mit echten Daten
- [ ] Loading States funktionieren
- [ ] StatusBar aktualisiert sich automatisch
- [ ] Bei leerer DB: sinnvolle Platzhalter (0 Lockers)

**Commit:**
```
feat(dashboard): Implement dashboard with stats and charts

- Add dashboard page with 4 stat cards
- Implement occupancy trend chart (Recharts)
- Add by-size and by-location visualizations
- Update StatusBar with live data (30s refetch)
- Include loading and error states
```

---

### Phase 7: Wizard System 🧙 (HOCH)

**Ziel:** Multi-Step-Formulare für komplexe Workflows

#### 7.1 Wizard Layout
**Datei:** `src-ui/components/wizards/WizardLayout.tsx`

Features:
- Step Indicator (visual progress)
- Forward/Back Navigation
- Data Persistence zwischen Steps
- Cancel Button
- Complete Handler (async)
- Vorlage: `v2.1-tauri.md` Abschnitt 9.1

#### 7.2 Rent Wizard
**Datei:** `src-ui/components/wizards/RentWizard.tsx`

Steps:
1. **Mieter-Informationen** (Name, Email, Phone)
2. **Schließfach auswählen** (Grid mit verfügbaren Lockers)
3. **Zeitraum & Zahlung** (Startdatum, Dauer, Pfand)

Vorlage: Abschnitt 9.2

**Wichtig:**
- Validierung mit Zod
- `useAvailableLockers()` für Step 2
- `useCreateRental()` Mutation beim Complete
- Navigation nach `/rentals/active` bei Erfolg

#### 7.3 Weitere Wizards (später)
- `ExtendWizard.tsx` – Verleih verlängern
- `ReturnWizard.tsx` – Rückgabe durchführen
- `BulkCreateWizard.tsx` – Mehrere Lockers anlegen

**Verifikation:**
- [ ] Step Indicator zeigt Fortschritt
- [ ] Zurück-Button funktioniert (Daten bleiben erhalten)
- [ ] Validierung verhindert ungültige Eingaben
- [ ] Complete-Button disabled bis alle Daten valide
- [ ] Toast bei Erfolg, Navigation zur Übersicht

**Commit:**
```
feat(wizards): Implement wizard system and rent wizard

- Add WizardLayout component with step indicator
- Implement RentWizard (3 steps: renter, locker, payment)
- Add form validation with Zod schemas
- Include navigation and data persistence
- Show success toast and redirect after completion
```

---

### Phase 8: Tabellen & Listen 📋 (MITTEL)

**Ziel:** Übersichts-Seiten mit Tabellen und Filtern

#### 8.1 Rentals Overview
**Datei:** `src-ui/pages/rentals/Overview.tsx`

Features:
- Tabelle mit allen Rentals
- Sortierung (by end_date, renter_name, etc.)
- Filter (Status: Aktiv, Überfällig, Abgeschlossen)
- Aktionen: Verlängern, Rückgabe, Details

Nutze `@tanstack/react-table` für Sortierung/Pagination

#### 8.2 Active Rentals
**Datei:** `src-ui/pages/rentals/Active.tsx`

- Nur aktive Verleihe (end_date >= heute, deposit_returned = 0)
- Highlight: Auslaufende Rentals (< 7 Tage)
- Quick Actions: Extend, Return

#### 8.3 Overdue Rentals
**Datei:** `src-ui/pages/rentals/Overdue.tsx`

- Überfällige Rentals (end_date < heute)
- Warnung-Badge bei jedem Eintrag
- CTA: Kontaktieren oder Rückgabe

#### 8.4 Lockers Management
**Datei:** `src-ui/pages/management/Lockers.tsx`

- CRUD für Lockers
- Modal für Create/Edit
- Delete mit Confirmation
- Status: Verfügbar, Belegt, Beschädigt

**Verifikation:**
- [ ] Tabellen zeigen Daten korrekt
- [ ] Sortierung funktioniert
- [ ] Filter ändern angezeigte Daten
- [ ] CRUD-Aktionen aktualisieren Tabelle (React Query Invalidation)
- [ ] Confirmation-Dialog vor Delete

**Commit:**
```
feat(tables): Add rental and locker management tables

- Implement rental overview with sorting and filters
- Add active rentals page with quick actions
- Create overdue rentals page with warnings
- Add locker CRUD page with modal forms
- Use @tanstack/react-table for advanced features
```

---

### Phase 9: Settings & Export 🔧 (MITTEL)

#### 9.1 Settings Page
**Datei:** `src-ui/pages/management/Settings.tsx`

Einstellungen:
- Pfand (Cents)
- Jahresgebühr (Cents)
- Berechnungszeitraum (Monthly/Yearly)
- Währung
- Screensaver Timeout

Form mit `react-hook-form` + Zod

#### 9.2 Export/Import UI
**Datei:** `src-ui/pages/management/Export.tsx`

- Format-Auswahl (TOML, JSON, CSV)
- Export-Button → Tauri File Dialog (`@tauri-apps/plugin-dialog`)
- Import-Button → File Upload + Command
- Vorlage: `v2.1-tauri.md` Abschnitt 12.2

#### 9.3 Export Commands
**Datei:** `src/commands/export.rs`

- `export_data(format, path)` Command
- Unterstützte Formate: TOML, JSON, CSV
- Audit-Log Eintrag bei Export

**Verifikation:**
- [ ] Settings speichern funktioniert
- [ ] Settings werden beim Reload korrekt geladen
- [ ] Export erstellt Datei am gewählten Ort
- [ ] Export-Datei ist valide und lesbar
- [ ] Import stellt Daten wieder her

**Commit:**
```
feat(settings): Add settings page and export/import

- Implement settings form with validation
- Add export functionality (TOML, JSON, CSV)
- Create import UI with file dialog
- Include audit log entries for export/import
- Use Tauri file system APIs
```

---

### Phase 10: Keyboard Shortcuts ⌨️ (MITTEL)

#### 10.1 Global Shortcuts Hook
**Datei:** `src-ui/hooks/use-keyboard.ts`

- `useKeyboardShortcuts(shortcuts[])`
- `useGlobalShortcuts()` Hook für App-weite Shortcuts
- Vorlage: `v2.1-tauri.md` Abschnitt 11.1

Shortcuts:
- `Strg+H` → Dashboard
- `Strg+N` → Neuer Verleih
- `Strg+R` → Rückgabe
- `Strg+,` → Einstellungen
- `Strg+?` → Shortcuts-Overlay
- `Esc` → Modal schließen

#### 10.2 Shortcuts Overlay
**Datei:** `src-ui/components/ShortcutsOverlay.tsx`

- Modal mit allen Shortcuts
- Gruppiert nach Kategorie (Global, Navigation, Tabellen)
- Öffnen mit `Strg+?`

#### 10.3 Integration
- `useGlobalShortcuts()` in `App.tsx` aufrufen
- Modal State verwalten

**Verifikation:**
- [ ] Alle Shortcuts funktionieren
- [ ] `Strg+?` öffnet Overlay
- [ ] Esc schließt Modals
- [ ] Shortcuts funktionieren nicht in Input-Feldern (prevent default)

**Commit:**
```
feat(shortcuts): Implement keyboard shortcuts system

- Add useKeyboardShortcuts and useGlobalShortcuts hooks
- Implement shortcuts overlay (Ctrl+?)
- Add global shortcuts (navigation, actions)
- Prevent shortcuts in input fields
- Document all shortcuts in overlay
```

---

### Phase 11: Polish & Animations ✨ (NIEDRIG)

#### 11.1 Transitions
- Page transitions (fade-in)
- Modal animations (zoom-in)
- Sidebar collapse animation
- Button hover effects

#### 11.2 Loading States
- Skeleton Screens für Tabellen
- Spinner für Buttons
- Progress Bars für lange Aktionen

#### 11.3 Empty States
- Leere Tabellen: Illustration + CTA
- Keine Suchergebnisse: Hilfetext
- Erste Benutzung: Onboarding-Hints

#### 11.4 Screensaver (Optional)
**Datei:** `src-ui/pages/Screensaver.tsx`

- CSS-Animation statt ASCII
- Aktivierung nach Inaktivität
- Exit bei beliebiger Eingabe
- Vorlage: `v2.1-tauri.md` Abschnitt 13

**Verifikation:**
- [ ] Animationen laufen smooth (60fps)
- [ ] Loading States zeigen sich bei langsamen Operationen
- [ ] Empty States sind hilfreich und visuell ansprechend
- [ ] Screensaver (falls implementiert) funktioniert

**Commit:**
```
feat(polish): Add animations and improved UX

- Implement page and modal transitions
- Add loading skeletons for tables
- Create empty state components
- Add hover effects and micro-animations
- Optional: Implement screensaver
```

---

### Phase 12: Testing 🧪 (HOCH)

**Ziel:** Robuste Tests für Backend und Frontend

#### 12.1 Backend Tests
**Dateien:** `tests/backend/*.rs`

Test-Typen:
- Unit Tests für Commands
- Integration Tests für DB-Operationen
- Migration Tests

Beispiel:
```rust
#[tokio::test]
async fn test_create_locker() {
    let temp_dir = TempDir::new().unwrap();
    let db = setup_test_db(&temp_dir.path().join("test.db")).await;
    
    let input = CreateLockerInput {
        number: "A01".to_string(),
        location: "Hauptgebäude".to_string(),
        size: "S".to_string(),
        notes: None,
    };
    
    let id = create_locker(State::from(&db), input).await.unwrap();
    assert!(id > 0);
}
```

#### 12.2 Frontend Tests
**Dateien:** `tests/frontend/*.test.tsx`

Test-Typen:
- Component Tests (Button, Input, Card, etc.)
- Hook Tests (use-lockers, use-rentals)
- Integration Tests (Wizard-Flows)

Beispiel:
```typescript
import { render, screen, fireEvent } from '@testing-library/react';
import { Button } from '@/components/ui/Button';

describe('Button', () => {
  it('renders correctly', () => {
    render(<Button>Click me</Button>);
    expect(screen.getByText('Click me')).toBeInTheDocument();
  });
  
  it('calls onClick handler', () => {
    const handleClick = vi.fn();
    render(<Button onClick={handleClick}>Click</Button>);
    fireEvent.click(screen.getByText('Click'));
    expect(handleClick).toHaveBeenCalledTimes(1);
  });
});
```

#### 12.3 Coverage Goal
- Backend: > 80%
- Frontend: > 70%
- Kritische Pfade: 100% (CRUD, Wizards)

**Verifikation:**
- [ ] Alle Tests laufen durch (`cargo test`, `npm test`)
- [ ] Coverage-Report zeigt ausreichende Abdeckung
- [ ] CI läuft erfolgreich

**Commit:**
```
test: Add comprehensive test suite

- Implement backend unit and integration tests
- Add frontend component and hook tests
- Create test utilities and helpers
- Achieve >80% backend and >70% frontend coverage
- Add test workflow to CI/CD
```

---

## 🔍 Täglicher Workflow

### Vor jeder Aufgabe:
1. **Spec lesen:** Relevanten Abschnitt in `v2.1-tauri.md` durchgehen
2. **Design prüfen:** Falls UI-Komponente → `tauri-design-system.md` konsultieren
3. **Types definieren:** TypeScript-Interfaces/Types anlegen
4. **Skelett bauen:** Grundstruktur ohne Logik
5. **Implementieren:** Schritt für Schritt
6. **Testen:** Manuell + automatisiert
7. **Commit:** Atomarer Commit mit konventioneller Message

### Nach jeder Aufgabe:
- [ ] Code kompiliert ohne Errors
- [ ] TypeScript Errors: 0
- [ ] Console Errors: 0
- [ ] Tests laufen durch
- [ ] UI sieht aus wie Design-System
- [ ] Spec-Anforderungen erfüllt
- [ ] Commit Message ist beschreibend

### Bei Problemen:
1. **Error analysieren:** Console, Terminal, Rust Compiler Output
2. **Spec prüfen:** Ist die Implementierung spec-konform?
3. **Types prüfen:** TypeScript-Fehler? Zod-Schema korrekt?
4. **Logs hinzufügen:** `console.log`, `println!` für Debugging
5. **Minimal reproduzieren:** Kleinsten Code, der Problem zeigt
6. **Dokumentieren:** Problem + Lösung in Commit Message

---

## 🚨 Error Handling Pattern

### Frontend (TypeScript)
```typescript
try {
  const result = await invoke('some_command', { arg });
  toast.success('Erfolgreich!');
} catch (error) {
  console.error('Error in some_command:', error);
  toast.error(`Fehler: ${error}`);
}
```

### Backend (Rust)
```rust
#[tauri::command]
pub async fn some_command(arg: String) -> Result<String, String> {
    // Business logic
    do_something(arg).map_err(|e| e.to_string())
}
```

**Wichtig:**
- Alle Commands geben `Result<T, String>` zurück
- Frontend fängt alle Errors mit try-catch
- Toast-Benachrichtigung bei Success UND Error
- Console.error für Debugging

---

## 📐 Code-Style-Regeln

### TypeScript/React
```typescript
// ✅ DO
export function MyComponent({ prop1, prop2 }: MyComponentProps) {
  const [state, setState] = useState(initialValue);
  
  useEffect(() => {
    // side effects
  }, [dependencies]);
  
  const handleClick = () => {
    // event handler
  };
  
  return (
    <div className="flex items-center gap-4">
      {/* JSX */}
    </div>
  );
}

// ❌ DON'T
export default function MyComponent(props) { // no default export, no any types
  return <div style={{color: 'red'}}>{/* inline styles */}</div>;
}
```

### Rust
```rust
// ✅ DO
#[tauri::command]
pub async fn get_lockers(db: State<'_, DbConnection>) -> Result<Vec<Locker>, String> {
    let conn = db.0.lock().await;
    // query logic
    Ok(lockers)
}

// ❌ DON'T
pub async fn get_lockers(db: State<'_, DbConnection>) -> Vec<Locker> { // no error handling
    // ...
}
```

### Tailwind
```tsx
// ✅ DO
<button className="px-4 py-2 bg-primary text-base rounded-md hover:bg-primary/90 transition-colors">
  Click me
</button>

// ❌ DON'T
<button style={{ backgroundColor: '#b4befe', padding: '8px 16px' }}>
  Click me
</button>
```

---

## 🎯 Qualitäts-Checkliste

Vor jedem Commit:
- [ ] **Kompiliert:** `npm run tauri dev` startet ohne Errors
- [ ] **Types:** Keine `any`, alle Props typisiert
- [ ] **Styling:** Tailwind-Classes, keine inline-styles
- [ ] **Theme:** Catppuccin-Farben verwendet (`var(--color-*)`)
- [ ] **Error Handling:** Try-catch + Toast-Feedback
- [ ] **Accessibility:** Semantic HTML, Labels, ARIA wenn nötig
- [ ] **Performance:** Keine unnötigen Rerenders (React.memo wenn sinnvoll)
- [ ] **Tests:** Neue Funktionalität getestet
- [ ] **Docs:** Komplexe Logik kommentiert
- [ ] **Commit:** Conventional Commit Message

---

## 📚 Wichtige Dateien (Quick Reference)

### Spec & Design
- `v2.1-tauri.md` → Technische Spezifikation (HAUPTDOKUMENT)
- `tauri-design-system.md` → UI/UX Guidelines
- Diese Datei → Tägliche Arbeitsanweisung

### Config
- `tailwind.config.js` → Tailwind-Konfiguration
- `src-ui/styles/theme.css` → CSS-Variablen (Catppuccin)
- `src-tauri/tauri.conf.json` → Tauri App Config
- `tsconfig.json` → TypeScript Config

### Key Components
- `src-ui/App.tsx` → Root Component + Routing
- `src-ui/components/layout/MainLayout.tsx` → Layout Wrapper
- `src-ui/components/ui/` → Reusable UI Components
- `src-ui/hooks/` → Custom React Hooks
- `src-ui/lib/tauri.ts` → Tauri API Wrapper

### Backend
- `src/main.rs` → Tauri Entry Point
- `src/commands/` → Tauri Commands (RPC API)
- `src/db/` → Database Layer
- `src/services/` → Business Logic

---

## 🎨 Design-System Quick Reference

### Colors (CSS Variables)
```css
--color-primary     /* Lavender #b4befe */
--color-success     /* Green #a6e3a1 */
--color-warning     /* Yellow #f9e2af */
--color-error       /* Red #f38ba8 */
--color-info        /* Sky #89dceb */

--color-base        /* Background #1e1e2e */
--color-surface-0   /* Cards #313244 */
--color-text        /* Text #cdd6f4 */
```

### Spacing
```css
--spacing-xs: 4px
--spacing-sm: 8px
--spacing-md: 16px
--spacing-lg: 24px
--spacing-xl: 32px
```

### Tailwind Shortcuts
```tsx
<div className="p-6">           {/* padding: 24px */}
<div className="rounded-lg">    {/* border-radius: 12px */}
<div className="text-text">     {/* color: var(--color-text) */}
<div className="bg-surface-0">  {/* background: var(--color-surface-0) */}
```

---

## 💡 Häufige Aufgaben (Snippets)

### Neue Tauri Command hinzufügen
1. Command in `src/commands/*.rs` implementieren
2. In `src/main.rs` zu `invoke_handler` hinzufügen
3. Wrapper in `src-ui/lib/tauri.ts` erstellen
4. Hook in `src-ui/hooks/*.ts` (falls nötig)

### Neue Page hinzufügen
1. Komponente in `src-ui/pages/` erstellen
2. Route in `src-ui/App.tsx` eintragen
3. Sidebar-Item in `NAV_ITEMS` hinzufügen (falls nötig)

### Neue UI-Komponente
1. Datei in `src-ui/components/ui/` erstellen
2. Props-Interface definieren
3. Styling mit Tailwind + CSS-Variablen
4. Export in `src-ui/components/ui/index.ts`

### Toast anzeigen
```typescript
import { toast } from 'sonner';

toast.success('Erfolgreich!');
toast.error('Fehler!');
toast.info('Information');
toast.warning('Warnung');
```

---

## 🏁 Erfolgskriterien

Das Projekt ist "fertig", wenn:
- [ ] Alle Phasen der Checkliste abgeschlossen
- [ ] Alle Features aus `v2.1-tauri.md` implementiert
- [ ] Tests laufen durch (>80% Backend, >70% Frontend)
- [ ] UI entspricht Design-System (`tauri-design-system.md`)
- [ ] Build funktioniert für alle Plattformen (Windows, macOS, Linux)
- [ ] README.md und USER_GUIDE.md sind vollständig
- [ ] Keine bekannten kritischen Bugs
- [ ] Performance: App startet < 3s, UI reagiert < 100ms

---

## 🤝 Zusammenarbeit

**Bei Fragen:**
1. Zuerst `v2.1-tauri.md` lesen (Abschnitt zu deiner Aufgabe)
2. `tauri-design-system.md` bei UI-Fragen
3. Code-Kommentare und Inline-Docs
4. Diese Anweisung für Workflow-Fragen

**Bei Spec-Unklarheiten:**
- NICHT raten oder abweichen
- Frage nach konkreter Anforderung
- Dokumentiere Entscheidung im Code/Commit

**Bei Bugs:**
1. Reproduzierbaren Test-Case erstellen
2. Error-Output vollständig kopieren
3. Relevanten Code-Kontext bereitstellen
4. Erwartetes vs. tatsächliches Verhalten beschreiben

---

## 📝 Commit-Message-Template

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:** feat, fix, refactor, style, test, docs, chore  
**Scope:** dashboard, layout, wizards, backend, ui, etc.  
**Subject:** Kurzbeschreibung (max 50 Zeichen)  
**Body:** Detaillierte Beschreibung (optional)  
**Footer:** Breaking changes, Issue-Refs (optional)

**Beispiele:**
```
feat(dashboard): Add occupancy trend chart

- Implement LineChart with Recharts
- Display last 30 days of history
- Add loading and error states

Closes #12
```

```
fix(wizard): Validate form fields before submission

The rent wizard was allowing submission with empty
required fields. Added Zod schema validation.

Fixes #45
```

---

## 🚀 Los geht's!

1. **Lies diese Anweisung vollständig** ✅
2. **Lies `v2.1-tauri.md` Abschnitt 1-3** (Überblick, Stack, DB-Schema)
3. **Starte mit Phase 1** (Projekt-Setup)
4. **Arbeite Phase für Phase** ab
5. **Committe oft** (atomare Commits)
6. **Teste kontinuierlich** (nicht erst am Ende)
7. **Frage bei Unklarheiten** (nicht raten)

**Viel Erfolg! 🎉**
