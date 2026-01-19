# Schließfach-Manager v2.1 – TypeScript Edition
# Vollständige Spezifikation

**Version:** 2.1-ts  
**Datum:** 2025-01-20  
**Status:** Implementierungsspezifikation  
**Basis:** v2.1 Rust Spec  
**Technologie:** TypeScript + OpenTUI + Bun

---

## 1. Einführung & Zielsetzung

### 1.1 Überblick

Der Schließfach-Manager ist eine Terminal-basierte Anwendung zur Verwaltung von Schließfächern, Verleihvorgängen, Finanzen und Standorten. Diese TypeScript-Version portiert die komplette v2.1 Spezifikation auf moderne Web-Technologien:

- **OpenTUI:** Terminal UI Framework für Node.js/Bun
- **Bun Build:** Kompilierung zu einer einzelnen ausführbaren Datei
- **TypeScript:** Typsicherheit und moderne JavaScript-Features
- **Same UI:** Identisches Look & Feel wie Rust-Version

**Kernfeatures:**
- Verbesserung der Usability: Globale Keybind-Architektur, Window Switcher, Dialog-artige Wizards
- Fixed-Height Dashboard Layout: Stabiles Layout bei Terminal-Resize
- Konfigurierbare Preise: Pfand, Jahresgebühren, Berechnungszeitraum
- Erweiterte Export/Import-Formate: TOML als Default, JSON, CSV, Markdown
- Audit-Logging: Vollständige Nachverfolgbarkeit aller Änderungen
- Historische Analysen: Belegungstrend, occupancy history
- 3x Escape zum Dashboard: Quick-Navigation zurück zur Hauptansicht
- Screensaver: Automatische Aktivierung nach Inaktivität
- Robuste Tests: Unit-, Integration- und UI-Tests mit >85% Coverage
- Vollständige Dokumentation: Inline-Dokumentation, README, User Guide

### 1.2 Abwärtskompatibilität

- Datenbank-Format identisch zur Rust-Version
- Export-Formate bleiben kompatibel
- Migration von Rust- zu TypeScript-Version möglich

### 1.3 Zielplattformen

- **Betriebssysteme:** Linux, macOS, Windows
- **Runtime:** Bun >= 1.0.0
- **Terminal:** Jeder moderne Terminal-Emulator mit UTF-8 Support
- **Mindestgröße:** 80x24 Zeichen (optimal: 120x40+)

---

## 2. Technologie-Stack

### 2.1 Kern-Dependencies

```json
{
  "name": "schliessfach-manager",
  "version": "2.1.0",
  "type": "module",
  "dependencies": {
    "@opentui/core": "^0.1.0",
    "better-sqlite3": "^9.2.0",
    "date-fns": "^3.0.0",
    "zod": "^3.22.0"
  },
  "devDependencies": {
    "@types/better-sqlite3": "^7.6.8",
    "@types/bun": "latest",
    "bun-types": "latest",
    "typescript": "^5.3.0"
  },
  "scripts": {
    "dev": "bun run src/index.ts",
    "build": "bun build src/index.ts --compile --outfile dist/schliessfach-manager",
    "test": "bun test",
    "test:watch": "bun test --watch",
    "typecheck": "tsc --noEmit"
  }
}
```

### 2.2 Projekt-Struktur

```
schließfach-manager/
├── src/
│   ├── index.ts                    # Entry point, Event loop
│   ├── app.ts                      # App state, screen management
│   ├── config.ts                   # Configuration, paths, settings
│   │
│   ├── db/
│   │   ├── index.ts                # DB module exports
│   │   ├── connection.ts           # Connection handling
│   │   ├── migrations.ts           # Schema migrations
│   │   ├── lockers.ts              # Locker queries
│   │   ├── rentals.ts              # Rental queries
│   │   ├── payments.ts             # Payment queries
│   │   ├── locations.ts            # Location queries
│   │   ├── audit.ts                # Audit log functions
│   │   ├── history.ts              # Historical data queries
│   │   └── schema.ts               # Database schema types
│   │
│   ├── ui/
│   │   ├── index.ts
│   │   ├── theme.ts                # Color scheme, styles
│   │   ├── state.ts                # UI state management
│   │   │
│   │   ├── screens/
│   │   │   ├── index.ts
│   │   │   ├── dashboard.ts        # Main dashboard
│   │   │   ├── rental-management.ts
│   │   │   ├── finances.ts
│   │   │   ├── management.ts
│   │   │   └── screensaver.ts      # Screensaver animations
│   │   │
│   │   └── widgets/
│   │       ├── index.ts
│   │       ├── header.ts           # Global header with window switcher
│   │       ├── keybind-bar.ts      # Global + context keybind display
│   │       ├── status-bar.ts       # Status messages + escape indicator
│   │       ├── wizard.ts           # Dialog-style wizard renderer
│   │       ├── table.ts            # Enhanced table widget
│   │       ├── graph.ts            # Trend visualization
│   │       └── message-box.ts      # Dialog/confirmation boxes
│   │
│   ├── workflows/
│   │   ├── index.ts
│   │   ├── rent.ts                 # Rental workflow
│   │   ├── extend.ts               # Extension workflow
│   │   ├── return-locker.ts        # Return workflow
│   │   ├── create-bulk.ts          # Bulk creation
│   │   ├── damage.ts               # Damage reporting & repair
│   │   └── common.ts               # Shared workflow utilities
│   │
│   ├── export/
│   │   ├── index.ts
│   │   ├── json.ts
│   │   ├── csv.ts
│   │   ├── toml.ts                 # Default format
│   │   └── markdown.ts             # Reports
│   │
│   ├── import/
│   │   ├── index.ts
│   │   ├── json.ts
│   │   ├── csv.ts
│   │   └── toml.ts
│   │
│   ├── screensaver/
│   │   ├── index.ts
│   │   ├── animations.ts           # ASCII animation definitions
│   │   └── renderer.ts             # Animation rendering logic
│   │
│   └── types/
│       ├── index.ts
│       ├── app.ts                  # Application types
│       ├── database.ts             # Database types
│       └── ui.ts                   # UI types
│
├── tests/
│   ├── integration/
│   │   └── workflows.test.ts       # End-to-end workflows
│   ├── ui/
│   │   └── interaction.test.ts     # UI interaction tests
│   └── helpers/
│       └── test-utils.ts
│
├── docs/
│   ├── README.md
│   ├── USER_GUIDE.md
│   ├── KEYBINDINGS.md
│   └── CHANGELOG.md
│
├── package.json
├── tsconfig.json
└── bunfig.toml
```

### 2.3 TypeScript Configuration

```json
{
  "compilerOptions": {
    "target": "ESNext",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "lib": ["ESNext"],
    "types": ["bun-types"],
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "allowSyntheticDefaultImports": true,
    "isolatedModules": true,
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    "outDir": "./dist",
    "rootDir": "./src",
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

### 2.4 Bun Build Configuration

```toml
# bunfig.toml
[build]
target = "bun"
outdir = "./dist"
minify = true

[test]
preload = ["./tests/helpers/test-utils.ts"]
```

---

## 3. Datenbank-Schema

### 3.1 Database Connection

```typescript
// src/db/connection.ts
import Database from 'better-sqlite3';
import { join } from 'path';
import { mkdirSync, existsSync } from 'fs';

export class DatabaseConnection {
  private static instance: Database.Database | null = null;
  
  static getConnection(): Database.Database {
    if (!this.instance) {
      const dbPath = this.getDatabasePath();
      const dbDir = join(dbPath, '..');
      
      if (!existsSync(dbDir)) {
        mkdirSync(dbDir, { recursive: true });
      }
      
      this.instance = new Database(dbPath);
      this.instance.pragma('journal_mode = WAL');
      this.instance.pragma('foreign_keys = ON');
    }
    
    return this.instance;
  }
  
  private static getDatabasePath(): string {
    // Platform-specific data directory
    const home = process.env.HOME || process.env.USERPROFILE || '';
    
    if (process.platform === 'darwin') {
      return join(home, 'Library', 'Application Support', 'schliessfach-manager', 'data.db');
    } else if (process.platform === 'win32') {
      return join(process.env.APPDATA || '', 'schliessfach-manager', 'data.db');
    } else {
      return join(home, '.local', 'share', 'schliessfach-manager', 'data.db');
    }
  }
  
  static close(): void {
    if (this.instance) {
      this.instance.close();
      this.instance = null;
    }
  }
}
```

### 3.2 Schema Types

```typescript
// src/db/schema.ts
import { z } from 'zod';

// Locker
export const LockerSchema = z.object({
  id: z.number().int().positive(),
  number: z.string(),
  location: z.string(),
  size: z.enum(['S', 'M', 'L', 'XL']),
  is_damaged: z.boolean(),
  notes: z.string().nullable(),
  created_at: z.string().datetime()
});

export type Locker = z.infer<typeof LockerSchema>;

// Rental
export const RentalSchema = z.object({
  id: z.number().int().positive(),
  locker_id: z.number().int().positive(),
  renter_name: z.string(),
  renter_email: z.string().email().nullable(),
  renter_phone: z.string().nullable(),
  start_date: z.string().datetime(),
  end_date: z.string().datetime().nullable(),
  deposit_paid: z.boolean(),
  deposit_returned: z.boolean(),
  notes: z.string().nullable(),
  created_at: z.string().datetime()
});

export type Rental = z.infer<typeof RentalSchema>;

// Payment
export const PaymentSchema = z.object({
  id: z.number().int().positive(),
  rental_id: z.number().int().positive(),
  amount_cents: z.number().int(),
  payment_date: z.string().datetime(),
  payment_type: z.enum(['cash', 'card', 'transfer', 'other']),
  notes: z.string().nullable(),
  created_at: z.string().datetime()
});

export type Payment = z.infer<typeof PaymentSchema>;

// Location
export const LocationSchema = z.object({
  id: z.number().int().positive(),
  name: z.string(),
  created_at: z.string().datetime()
});

export type Location = z.infer<typeof LocationSchema>;

// Settings
export const SettingSchema = z.object({
  key: z.string(),
  value: z.string(),
  description: z.string().nullable(),
  updated_at: z.string().datetime()
});

export type Setting = z.infer<typeof SettingSchema>;

// Audit Log
export const AuditLogSchema = z.object({
  id: z.number().int().positive(),
  timestamp: z.string().datetime(),
  action: z.enum(['create', 'update', 'delete', 'export', 'import']),
  entity_type: z.enum(['locker', 'rental', 'payment', 'location', 'settings']),
  entity_id: z.number().int().nullable(),
  details: z.string().nullable(),
  username: z.string()
});

export type AuditLog = z.infer<typeof AuditLogSchema>;

// Occupancy History
export const OccupancyHistorySchema = z.object({
  id: z.number().int().positive(),
  snapshot_date: z.string(),
  total_lockers: z.number().int(),
  occupied_lockers: z.number().int(),
  occupancy_percent: z.number(),
  notes: z.string().nullable()
});

export type OccupancyHistory = z.infer<typeof OccupancyHistorySchema>;
```

### 3.3 Migrations

```typescript
// src/db/migrations.ts
import type Database from 'better-sqlite3';

export function runMigrations(db: Database.Database): void {
  const currentVersion = getSchemaVersion(db);
  
  if (currentVersion < 1) {
    createInitialSchema(db);
  }
  
  if (currentVersion < 2) {
    addSettingsTable(db);
    addAuditLogTable(db);
  }
  
  if (currentVersion < 3) {
    addOccupancyHistoryTable(db);
  }
  
  setSchemaVersion(db, 3);
}

function getSchemaVersion(db: Database.Database): number {
  try {
    const result = db.prepare('SELECT value FROM settings WHERE key = ?').get('schema_version');
    return result ? parseInt((result as any).value, 10) : 0;
  } catch {
    return 0;
  }
}

function setSchemaVersion(db: Database.Database, version: number): void {
  db.prepare(`
    INSERT OR REPLACE INTO settings (key, value, description, updated_at)
    VALUES (?, ?, ?, datetime('now'))
  `).run('schema_version', version.toString(), 'Database schema version');
}

function createInitialSchema(db: Database.Database): void {
  db.exec(`
    CREATE TABLE IF NOT EXISTS locations (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL UNIQUE,
      created_at TEXT NOT NULL DEFAULT (datetime('now'))
    );
    
    CREATE TABLE IF NOT EXISTS lockers (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      number TEXT NOT NULL UNIQUE,
      location TEXT NOT NULL,
      size TEXT NOT NULL CHECK(size IN ('S', 'M', 'L', 'XL')),
      is_damaged BOOLEAN NOT NULL DEFAULT 0,
      notes TEXT,
      created_at TEXT NOT NULL DEFAULT (datetime('now'))
    );
    
    CREATE TABLE IF NOT EXISTS rentals (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      locker_id INTEGER NOT NULL,
      renter_name TEXT NOT NULL,
      renter_email TEXT,
      renter_phone TEXT,
      start_date TEXT NOT NULL,
      end_date TEXT,
      deposit_paid BOOLEAN NOT NULL DEFAULT 0,
      deposit_returned BOOLEAN NOT NULL DEFAULT 0,
      notes TEXT,
      created_at TEXT NOT NULL DEFAULT (datetime('now')),
      FOREIGN KEY (locker_id) REFERENCES lockers(id)
    );
    
    CREATE TABLE IF NOT EXISTS payments (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      rental_id INTEGER NOT NULL,
      amount_cents INTEGER NOT NULL,
      payment_date TEXT NOT NULL,
      payment_type TEXT NOT NULL CHECK(payment_type IN ('cash', 'card', 'transfer', 'other')),
      notes TEXT,
      created_at TEXT NOT NULL DEFAULT (datetime('now')),
      FOREIGN KEY (rental_id) REFERENCES rentals(id)
    );
    
    CREATE INDEX IF NOT EXISTS idx_rentals_locker ON rentals(locker_id);
    CREATE INDEX IF NOT EXISTS idx_rentals_dates ON rentals(start_date, end_date);
    CREATE INDEX IF NOT EXISTS idx_payments_rental ON payments(rental_id);
  `);
}

function addSettingsTable(db: Database.Database): void {
  db.exec(`
    CREATE TABLE IF NOT EXISTS settings (
      key TEXT PRIMARY KEY NOT NULL,
      value TEXT NOT NULL,
      description TEXT,
      updated_at TEXT NOT NULL DEFAULT (datetime('now'))
    );
    
    INSERT OR IGNORE INTO settings (key, value, description) VALUES
      ('deposit_cents', '1000', 'Pfandbetrag in Cents (Standard: 10.00€)'),
      ('yearly_fee_cents', '1000', 'Jahresgebühr in Cents (Standard: 10.00€)'),
      ('billing_period', 'yearly', 'Berechnungszeitraum: monthly oder yearly'),
      ('currency', 'EUR', 'Währung (ISO 4217 Code)'),
      ('screensaver_timeout_seconds', '60', 'Sekunden Inaktivität bis Screensaver'),
      ('app_version', '2.1.0', 'Anwendungsversion');
  `);
}

function addAuditLogTable(db: Database.Database): void {
  db.exec(`
    CREATE TABLE IF NOT EXISTS audit_log (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      timestamp TEXT NOT NULL DEFAULT (datetime('now')),
      action TEXT NOT NULL CHECK(action IN ('create', 'update', 'delete', 'export', 'import')),
      entity_type TEXT NOT NULL CHECK(entity_type IN ('locker', 'rental', 'payment', 'location', 'settings')),
      entity_id INTEGER,
      details TEXT,
      username TEXT NOT NULL DEFAULT 'system'
    );
    
    CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON audit_log(timestamp);
    CREATE INDEX IF NOT EXISTS idx_audit_entity ON audit_log(entity_type, entity_id);
  `);
}

function addOccupancyHistoryTable(db: Database.Database): void {
  db.exec(`
    CREATE TABLE IF NOT EXISTS occupancy_history (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      snapshot_date TEXT NOT NULL UNIQUE,
      total_lockers INTEGER NOT NULL,
      occupied_lockers INTEGER NOT NULL,
      occupancy_percent REAL NOT NULL,
      notes TEXT
    );
    
    CREATE INDEX IF NOT EXISTS idx_occupancy_date ON occupancy_history(snapshot_date);
  `);
}
```

### 3.4 Audit Logging

```typescript
// src/db/audit.ts
import { DatabaseConnection } from './connection';
import type { AuditLog } from './schema';

export function logAction(
  action: AuditLog['action'],
  entityType: AuditLog['entity_type'],
  entityId: number | null,
  details?: Record<string, any>,
  username = 'system'
): void {
  const db = DatabaseConnection.getConnection();
  
  db.prepare(`
    INSERT INTO audit_log (action, entity_type, entity_id, details, username)
    VALUES (?, ?, ?, ?, ?)
  `).run(
    action,
    entityType,
    entityId,
    details ? JSON.stringify(details) : null,
    username
  );
}

export function getAuditLog(limit = 100): AuditLog[] {
  const db = DatabaseConnection.getConnection();
  
  return db.prepare(`
    SELECT * FROM audit_log
    ORDER BY timestamp DESC
    LIMIT ?
  `).all(limit) as AuditLog[];
}

export function getAuditLogForEntity(
  entityType: AuditLog['entity_type'],
  entityId: number
): AuditLog[] {
  const db = DatabaseConnection.getConnection();
  
  return db.prepare(`
    SELECT * FROM audit_log
    WHERE entity_type = ? AND entity_id = ?
    ORDER BY timestamp DESC
  `).all(entityType, entityId) as AuditLog[];
}
```

---

## 4. Globale UI-Architektur

### 4.1 OpenTUI Setup

```typescript
// src/ui/index.ts
import { Terminal } from '@opentui/core';

export function createTerminal() {
  const terminal = new Terminal({
    title: 'Schließfach-Manager v2.1',
    cursor: false,
    alternateScreen: true,
    mouseEnabled: false
  });
  
  return terminal;
}

// Layout types
export interface Layout {
  header: Rect;
  content: Rect;
  keybindBar: Rect;
  statusBar: Rect;
}

export interface Rect {
  x: number;
  y: number;
  width: number;
  height: number;
}

export function calculateLayout(termWidth: number, termHeight: number): Layout {
  const headerHeight = 3;
  const keybindBarHeight = 2;
  const statusBarHeight = 1;
  
  const contentHeight = termHeight - headerHeight - keybindBarHeight - statusBarHeight;
  
  return {
    header: { x: 0, y: 0, width: termWidth, height: headerHeight },
    content: { x: 0, y: headerHeight, width: termWidth, height: contentHeight },
    keybindBar: { 
      x: 0, 
      y: headerHeight + contentHeight, 
      width: termWidth, 
      height: keybindBarHeight 
    },
    statusBar: { 
      x: 0, 
      y: termHeight - statusBarHeight, 
      width: termWidth, 
      height: statusBarHeight 
    }
  };
}
```

### 4.2 Header
 Component

```typescript
// src/ui/widgets/header.ts
import type { Terminal } from '@opentui/core';
import { Theme } from '../theme';

export interface HeaderProps {
  version: string;
  currentScreen: string;
  windowSwitcherActive: boolean;
  windowList: WindowInfo[];
  selectedWindowIndex: number;
}

export interface WindowInfo {
  name: string;
  screen: string;
}

export class Header {
  constructor(private props: HeaderProps) {}
  
  render(terminal: Terminal, x: number, y: number, width: number): void {
    const theme = Theme.defaultDark();
    
    if (this.props.windowSwitcherActive) {
      this.renderWindowSwitcher(terminal, x, y, width);
    } else {
      this.renderBasicHeader(terminal, x, y, width);
    }
  }
  
  private renderBasicHeader(terminal: Terminal, x: number, y: number, width: number): void {
    const theme = Theme.defaultDark();
    
    // Top border
    terminal.moveTo(x, y);
    terminal.write('╔' + '═'.repeat(width - 2) + '╗', theme.headerBg, theme.headerFg);
    
    // Title line
    terminal.moveTo(x, y + 1);
    terminal.write('║', theme.headerBg, theme.headerFg);
    
    const title = ` Schließfach-Manager v${this.props.version} `;
    const screenName = ` [${this.props.currentScreen}] `;
    const padding = width - title.length - screenName.length - 2;
    
    terminal.write(title, theme.headerBg, theme.primary);
    terminal.write(' '.repeat(padding), theme.headerBg, theme.headerFg);
    terminal.write(screenName, theme.headerBg, theme.accent);
    terminal.write('║', theme.headerBg, theme.headerFg);
    
    // Bottom border
    terminal.moveTo(x, y + 2);
    terminal.write('╚' + '═'.repeat(width - 2) + '╝', theme.headerBg, theme.headerFg);
  }
  
  private renderWindowSwitcher(terminal: Terminal, x: number, y: number, width: number): void {
    const theme = Theme.defaultDark();
    
    // Top border
    terminal.moveTo(x, y);
    terminal.write('╔' + '═'.repeat(width - 2) + '╗', theme.headerBg, theme.headerFg);
    
    // Window list line (3-part preview)
    terminal.moveTo(x, y + 1);
    terminal.write('║ ', theme.headerBg, theme.headerFg);
    
    const selected = this.props.selectedWindowIndex;
    const prevIdx = selected > 0 ? selected - 1 : this.props.windowList.length - 1;
    const nextIdx = (selected + 1) % this.props.windowList.length;
    
    const prev = this.props.windowList[prevIdx];
    const curr = this.props.windowList[selected];
    const next = this.props.windowList[nextIdx];
    
    const sectionWidth = Math.floor((width - 4) / 3);
    
    // Previous (dim)
    terminal.write(
      this.padCenter(prev.name, sectionWidth),
      theme.headerBg,
      theme.windowAdjacent
    );
    
    // Current (bright)
    terminal.write(
      this.padCenter(`[ ${curr.name} ]`, sectionWidth),
      theme.headerBg,
      theme.windowCurrent
    );
    
    // Next (dim)
    terminal.write(
      this.padCenter(next.name, sectionWidth),
      theme.headerBg,
      theme.windowAdjacent
    );
    
    terminal.write(' ║', theme.headerBg, theme.headerFg);
    
    // Bottom border with instructions
    terminal.moveTo(x, y + 2);
    const instructions = ' Tab/Shift+Tab: Navigate | Enter: Select | Esc: Cancel ';
    const bottomPadding = Math.floor((width - instructions.length - 2) / 2);
    terminal.write('╚' + '═'.repeat(bottomPadding), theme.headerBg, theme.headerFg);
    terminal.write(instructions, theme.headerBg, theme.info);
    terminal.write('═'.repeat(width - bottomPadding - instructions.length - 2) + '╝', theme.headerBg, theme.headerFg);
  }
  
  private padCenter(text: string, width: number): string {
    if (text.length >= width) {
      return text.substring(0, width);
    }
    const padding = width - text.length;
    const leftPad = Math.floor(padding / 2);
    const rightPad = padding - leftPad;
    return ' '.repeat(leftPad) + text + ' '.repeat(rightPad);
  }
}
```

### 4.3 Keybind Bar

```typescript
// src/ui/widgets/keybind-bar.ts
import type { Terminal } from '@opentui/core';
import { Theme } from '../theme';

export interface Keybind {
  key: string;
  description: string;
  scope: 'global' | 'context';
}

export class KeybindBar {
  constructor(
    private globalBinds: Keybind[],
    private contextBinds: Keybind[] = [],
    private contextMessage?: string
  ) {}
  
  render(terminal: Terminal, x: number, y: number, width: number, height: number): void {
    const theme = Theme.defaultDark();
    
    // Line 1: Global keybinds
    terminal.moveTo(x, y);
    const globalText = this.formatKeybinds(this.globalBinds, 'global');
    terminal.write(this.truncate(globalText, width), theme.background, theme.keybindGlobal);
    
    // Line 2: Context keybinds
    if (height > 1) {
      terminal.moveTo(x, y + 1);
      
      if (this.contextMessage) {
        terminal.write(`[${this.contextMessage}] `, theme.background, theme.text);
      }
      
      const contextText = this.formatKeybinds(this.contextBinds, 'context');
      const availableWidth = width - (this.contextMessage ? this.contextMessage.length + 3 : 0);
      terminal.write(this.truncate(contextText, availableWidth), theme.background, theme.keybindContext);
    }
  }
  
  private formatKeybinds(binds: Keybind[], scope: string): string {
    return binds
      .map(b => `[${b.key}] ${b.description}`)
      .join('  ');
  }
  
  private truncate(text: string, maxWidth: number): string {
    if (text.length <= maxWidth) {
      return text.padEnd(maxWidth);
    }
    return text.substring(0, maxWidth - 3) + '...';
  }
  
  updateContext(binds: Keybind[], message?: string): void {
    this.contextBinds = binds;
    this.contextMessage = message;
  }
}
```

### 4.4 Status Bar

```typescript
// src/ui/widgets/status-bar.ts
import type { Terminal } from '@opentui/core';
import { Theme } from '../theme';

export type StatusLevel = 'info' | 'success' | 'warning' | 'error';

export interface StatusMessage {
  text: string;
  level: StatusLevel;
  timestamp: number;
}

export class StatusBar {
  private message: StatusMessage | null = null;
  private escapeCount = 0;
  private lastEscapeTime = 0;
  private readonly resetDelay = 1000; // 1 second
  
  setMessage(text: string, level: StatusLevel = 'info'): void {
    this.message = {
      text,
      level,
      timestamp: Date.now()
    };
  }
  
  incrementEscapeCount(): void {
    const now = Date.now();
    
    // Reset if more than 1 second passed
    if (now - this.lastEscapeTime > this.resetDelay) {
      this.escapeCount = 0;
    }
    
    this.escapeCount++;
    this.lastEscapeTime = now;
  }
  
  resetEscapeCount(): void {
    this.escapeCount = 0;
    this.lastEscapeTime = 0;
  }
  
  shouldReturnToDashboard(): boolean {
    return this.escapeCount >= 3;
  }
  
  render(terminal: Terminal, x: number, y: number, width: number): void {
    const theme = Theme.defaultDark();
    
    // Left side: Status message
    let messageText = '';
    let messageColor = theme.text;
    
    if (this.message) {
      messageText = this.message.text;
      
      switch (this.message.level) {
        case 'success':
          messageColor = theme.statusSuccess;
          break;
        case 'warning':
          messageColor = theme.statusWarning;
          break;
        case 'error':
          messageColor = theme.statusError;
          break;
        default:
          messageColor = theme.statusInfo;
      }
    }
    
    // Right side: Escape indicator
    const indicator = this.renderEscapeIndicator();
    const indicatorWidth = indicator.length;
    
    // Calculate message max width
    const maxMessageWidth = width - indicatorWidth - 2;
    const truncatedMessage = messageText.substring(0, maxMessageWidth).padEnd(maxMessageWidth);
    
    terminal.moveTo(x, y);
    terminal.write(truncatedMessage, theme.background, messageColor);
    terminal.write(' ' + indicator, theme.background, theme.text);
  }
  
  private renderEscapeIndicator(): string {
    const pipes = ['│', '│', '│'];
    const theme = Theme.defaultDark();
    
    // Color pipes based on escape count
    for (let i = 0; i < this.escapeCount && i < 3; i++) {
      pipes[i] = '║'; // Filled pipe
    }
    
    return `[${pipes.join('')}]`;
  }
  
  showScreensaverCountdown(seconds: number): void {
    this.setMessage(`⏱ Screensaver in ${seconds} Sekunden...`, 'info');
  }
}
```

---

## 5. Screensaver

### 5.1 Animation System

```typescript
// src/screensaver/animations.ts

export interface Animation {
  name: string;
  frames: string[];
  frameDelayMs: number;
  width: number;
  height: number;
}

export class AnimationPlayer {
  private currentFrame = 0;
  private lastFrameTime = Date.now();
  
  constructor(private animation: Animation) {}
  
  update(): void {
    const elapsed = Date.now() - this.lastFrameTime;
    
    if (elapsed >= this.animation.frameDelayMs) {
      this.currentFrame = (this.currentFrame + 1) % this.animation.frames.length;
      this.lastFrameTime = Date.now();
    }
  }
  
  getCurrentFrame(): string {
    return this.animation.frames[this.currentFrame];
  }
  
  getAnimation(): Animation {
    return this.animation;
  }
}

// Spinning Clock Animation
export const SpinningClock: Animation = {
  name: 'Spinning Clock',
  frames: ['🕐', '🕑', '🕒', '🕓', '🕔', '🕕', '🕖', '🕗', '🕘', '🕙', '🕚', '🕛'],
  frameDelayMs: 150,
  width: 2,
  height: 1
};

// Bouncing Box Animation
export const BouncingBox: Animation = {
  name: 'Bouncing Box',
  frames: [
    `┌─────────────┐
│             │
│   ┌─────┐   │
│   │  ■  │   │
│   └─────┘   │
│             │
└─────────────┘`,
    `┌─────────────┐
│  ┌─────┐    │
│  │  ■  │    │
│  └─────┘    │
│             │
│             │
└─────────────┘`,
    `┌─────────────┐
│┌─────┐      │
││  ■  │      │
│└─────┘      │
│             │
│             │
└─────────────┘`,
    `┌─────────────┐
││  ■  │       │
│└─────┘       │
│             │
│             │
│             │
└─────────────┘`,
    // Bounce back
    `┌─────────────┐
│┌─────┐       │
││  ■  │       │
│└─────┘       │
│             │
│             │
└─────────────┘`,
    `┌─────────────┐
│  ┌─────┐     │
│  │  ■  │     │
│  └─────┘     │
│             │
│             │
└─────────────┘`,
    `┌─────────────┐
│             │
│   ┌─────┐   │
│   │  ■  │   │
│   └─────┘   │
│             │
└─────────────┘`,
    `┌─────────────┐
│             │
│             │
│   ┌─────┐   │
│   │  ■  │   │
│   └─────┘   │
└─────────────┘`,
    // Bounce up
    `┌─────────────┐
│             │
│   ┌─────┐   │
│   │  ■  │   │
│   └─────┘   │
│             │
└─────────────┘`
  ],
  frameDelayMs: 100,
  width: 15,
  height: 7
};

// Matrix Rain Animation
export const MatrixRain: Animation = {
  name: 'Matrix Rain',
  frames: [
    `  1   0     1
    0   1     0
  1       0
      1       1
0       1
    1       0
        0   1
  1   1       `,
    `    0   1     
  1       0   
      1       
0       1   1 
    1       0 
        0   1 
  1   1       
      0       `,
    `  1     0   1
      1       
    0   1     
  1       1   
      0       
1       0   1 
    1       
        1   0 `,
    `      1     0
1   0       1
    1   0     
        1     
  1       0   
      1       
0       1   1 
  0       1   `
  ],
  frameDelayMs: 200,
  width: 20,
  height: 8
};

// Loading Spinner Animation
export const LoadingSpinner: Animation = {
  name: 'Loading Spinner',
  frames: [
    `╔════════════╗
║ ⠋ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠙ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠹ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠸ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠼ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠴ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠦ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠧ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠇ Loading  ║
╚════════════╝`,
    `╔════════════╗
║ ⠏ Loading  ║
╚════════════╝`
  ],
  frameDelayMs: 100,
  width: 14,
  height: 3
};

// Waving Text Animation
export const WavingText: Animation = {
  name: 'Waving Text',
  frames: [
    `╔═══════════════════════╗
║                       ║
║   Schließfach-Manager ║
║                       ║
╚═══════════════════════╝`,
    `╔═══════════════════════╗
║  Schließfach-Manager  ║
║                       ║
║                       ║
╚═══════════════════════╝`,
    `╔═══════════════════════╗
║                       ║
║  Schließfach-Manager  ║
║                       ║
╚═══════════════════════╝`,
    `╔═══════════════════════╗
║                       ║
║                       ║
║  Schließfach-Manager  ║
╚═══════════════════════╝`,
    `╔═══════════════════════╗
║                       ║
║  Schließfach-Manager  ║
║                       ║
╚═══════════════════════╝`,
    `╔═══════════════════════╗
║  Schließfach-Manager  ║
║                       ║
║                       ║
╚═══════════════════════╝`
  ],
  frameDelayMs: 300,
  width: 25,
  height: 5
};

export function getRandomAnimation(): Animation {
  const animations = [
    SpinningClock,
    BouncingBox,
    MatrixRain,
    LoadingSpinner,
    WavingText
  ];
  
  return animations[Math.floor(Math.random() * animations.length)];
}
```

### 5.2 Screensaver Screen

```typescript
// src/ui/screens/screensaver.ts
import type { Terminal } from '@opentui/core';
import { AnimationPlayer, getRandomAnimation } from '../../screensaver/animations';
import { Theme } from '../theme';

export class ScreensaverScreen {
  private player: AnimationPlayer;
  
  constructor() {
    const animation = getRandomAnimation();
    this.player = new AnimationPlayer(animation);
  }
  
  update(): void {
    this.player.update();
  }
  
  render(terminal: Terminal, width: number, height: number): void {
    const theme = Theme.defaultDark();
    
    // Clear screen (all black)
    terminal.clear();
    
    // Get animation
    const animation = this.player.getAnimation();
    const frame = this.player.getCurrentFrame();
    
    // Calculate center position
    const x = Math.floor((width - animation.width) / 2);
    const y = Math.floor((height - animation.height) / 2);
    
    // Render frame centered
    const lines = frame.split('\n');
    
    for (let i = 0; i < lines.length; i++) {
      terminal.moveTo(x, y + i);
      terminal.write(lines[i], theme.screensaverBg, theme.screensaverFg);
    }
  }
}
```

### 5.3 Inactivity Tracking

```typescript
// src/app.ts (excerpt)
export type InactivityState = 
  | { type: 'active' }
  | { type: 'countdown'; seconds: number }
  | { type: 'screensaver' };

export class App {
  private lastActivity = Date.now();
  private screensaverTimeout = 60000; // 60 seconds (from settings)
  private countdownDuration = 15000; // 15 seconds
  private screensaverActive = false;
  private previousStatusMessage: StatusMessage | null = null;
  private previousScreen: AppScreen | null = null;
  
  updateActivity(): void {
    this.lastActivity = Date.now();
    
    if (this.screensaverActive) {
      this.exitScreensaver();
    }
  }
  
  checkInactivity(): InactivityState {
    const inactiveDuration = Date.now() - this.lastActivity;
    
    if (inactiveDuration >= this.screensaverTimeout + this.countdownDuration) {
      return { type: 'screensaver' };
    } else if (inactiveDuration >= this.screensaverTimeout) {
      const remaining = Math.ceil((this.countdownDuration - (inactiveDuration - this.screensaverTimeout)) / 1000);
      return { type: 'countdown', seconds: remaining };
    } else {
      return { type: 'active' };
    }
  }
  
  activateScreensaver(): void {
    if (!this.screensaverActive) {
      this.previousStatusMessage = this.statusBar.getMessage();
      this.previousScreen = this.currentScreen;
      this.screensaverActive = true;
      this.currentScreen = 'screensaver';
    }
  }
  
  exitScreensaver(): void {
    if (this.screensaverActive) {
      this.screensaverActive = false;
      
      // Always return to Dashboard after screensaver
      this.currentScreen = 'dashboard';
      this.previousScreen = null;
      
      // Restore previous status message
      if (this.previousStatusMessage) {
        this.statusBar.setMessage(
          this.previousStatusMessage.text,
          this.previousStatusMessage.level
        );
      }
      
      this.updateActivity();
    }
  }
}
```

---

## 6. Dashboard

### 6.1 Dashboard Data

```typescript
// src/ui/screens/dashboard.ts
import type { Terminal } from '@opentui/core';
import { DatabaseConnection } from '../../db/connection';
import { Theme } from '../theme';

export interface DashboardData {
  totalLockers: number;
  occupiedLockers: number;
  occupancyPercent: number;
  bySize: Record<string, { total: number; occupied: number }>;
  byLocation: Record<string, { total: number; occupied: number }>;
  overdueReturns: number;
  expiringSoon: number;
  damagedLockers: number;
  locations: string[];
  pendingPaymentsCents: number;
  revenue30dCents: number;
  occupancyHistory: Array<{ date: string; percent: number }>;
}

export function loadDashboardData(): DashboardData {
  const db = DatabaseConnection.getConnection();
  
  // Total and occupied lockers
  const totalLockers = db.prepare('SELECT COUNT(*) as count FROM lockers').get() as { count: number };
  
  const occupiedLockers = db.prepare(`
    SELECT COUNT(DISTINCT locker_id) as count
    FROM rentals
    WHERE end_date IS NULL OR date(end_date) >= date('now')
  `).get() as { count: number };
  
  const occupancyPercent = totalLockers.count > 0
    ? (occupiedLockers.count / totalLockers.count) * 100
    : 0;
  
  // By size
  const bySize = db.prepare(`
    SELECT 
      size,
      COUNT(*) as total,
      COUNT(CASE WHEN r.id IS NOT NULL THEN 1 END) as occupied
    FROM lockers l
    LEFT JOIN rentals r ON l.id = r.locker_id 
      AND (r.end_date IS NULL OR date(r.end_date) >= date('now'))
    GROUP BY size
  `).all() as Array<{ size: string; total: number; occupied: number }>;
  
  const bySizeMap: Record<string, { total: number; occupied: number }> = {};
  for (const row of bySize) {
    bySizeMap[row.size] = { total: row.total, occupied: row.occupied };
  }
  
  // By location
  const byLocation = db.prepare(`
    SELECT 
      location,
      COUNT(*) as total,
      COUNT(CASE WHEN r.id IS NOT NULL THEN 1 END) as occupied
    FROM lockers l
    LEFT JOIN rentals r ON l.id = r.locker_id 
      AND (r.end_date IS NULL OR date(r.end_date) >= date('now'))
    GROUP BY location
  `).all() as Array<{ location: string; total: number; occupied: number }>;
  
  const byLocationMap: Record<string, { total: number; occupied: number }> = {};
  for (const row of byLocation) {
    byLocationMap[row.location] = { total: row.total, occupied: row.occupied };
  }
  
  // Overdue returns
  const overdueReturns = db.prepare(`
    SELECT COUNT(*) as count
    FROM rentals
    WHERE end_date IS NOT NULL AND date(end_date) < date('now')
  `).get() as { count: number };
  
  // Expiring soon (next 30 days)
  const expiringSoon = db.prepare(`
    SELECT COUNT(*) as count
    FROM rentals
    WHERE end_date IS NOT NULL 
      AND date(end_date) >= date('now')
      AND date(end_date) <= date('now', '+30 days')
  `).get() as { count: number };
  
  // Damaged lockers
  const damagedLockers = db.prepare(`
    SELECT COUNT(*) as count
    FROM lockers
    WHERE is_damaged = 1
  `).get() as { count: number };
  
  // Locations
  const locations = db.prepare(`
    SELECT DISTINCT location FROM lockers ORDER BY location
  `).all() as Array<{ location: string }>;
  
  // Pending payments (deposit not paid)
  const pendingPayments = db.prepare(`
    SELECT COALESCE(SUM(
      (SELECT value FROM settings WHERE key = 'deposit_cents')
    ), 0) as total
    FROM rentals
    WHERE deposit_paid = 0 AND (end_date IS NULL OR date(end_date) >= date('now'))
  `).get() as { total: number };
  
  // Revenue last 30 days
  const revenue30d = db.prepare(`
    SELECT COALESCE(SUM(amount_cents), 0) as total
    FROM payments
    WHERE date(payment_date) >= date('now', '-30 days')
  `).get() as { total: number };
  
  // Occupancy history (last 30 days)
  const occupancyHistory = db.prepare(`
    SELECT snapshot_date as date, occupancy_percent as percent
    FROM occupancy_history
    WHERE date(snapshot_date) >= date('now', '-30 days')
    ORDER BY snapshot_date ASC
  `).all() as Array<{ date: string; percent: number }>;
  
  return {
    totalLockers: totalLockers.count,
    occupiedLockers: occupiedLockers.count,
    occupancyPercent,
    bySize: bySizeMap,
    byLocation: byLocationMap,
    overdueReturns: overdueReturns.count,
    expiringSoon: expiringSoon.count,
    damagedLockers: damagedLockers.count,
    locations: locations.map(l => l.location),
    pendingPaymentsCents: pendingPayments.total,
    revenue30dCents: revenue30d.total,
    occupancyHistory
  };
}

export class DashboardScreen {
  render(terminal: Terminal, x: number, y: number, width: number, height: number): void {
    const data = loadDashboardData();
    const theme = Theme.defaultDark();
    
    // Use fixed height layout
    const sections = this.calculateLayout(height);
    
    // Render sections
    this.renderOverview(terminal, x, y, width, sections.overview, data);
    this.renderStatistics(terminal, x, y + sections.overview, width, sections.statistics, data);
    this.renderAlerts(terminal, x, y + sections.overview + sections.statistics, width, sections.alerts, data);
    this.renderGraph(terminal, x, y + sections.overview + sections.statistics + sections.alerts, width, sections.graph, data);
  }
  
  private calculateLayout(totalHeight: number) {
    return {
      overview: 5,        // Fixed height
      statistics: 8,      // Fixed height
      alerts: 4,          // Fixed height
      graph: Math.max(10, totalHeight - 17) // Flexible (minimum 10)
    };
  }
  
  private renderOverview(terminal: Terminal, x: number, y: number, width: number, height: number, data: DashboardData): void {
    const theme = Theme.defaultDark();
    
    terminal.moveTo(x, y);
    terminal.write('╔═══ Übersicht ' + '═'.repeat(width - 16) + '╗', theme.border, theme.text);
    
    terminal.moveTo(x, y + 1);
    terminal.write('║', theme.border, theme.text);
    terminal.write(` Gesamt: ${data.totalLockers} Schließfächer`, theme.background, theme.text);
    terminal.write(' '.repeat(width - 30), theme.background, theme.text);
    terminal.write('║', theme.border, theme.text);
    
    terminal.moveTo(x, y + 2);
    terminal.write('║', theme.border, theme.text);
    terminal.write(` Belegt: ${data.occupiedLockers} (${data.occupancyPercent.toFixed(1)}%)`, theme.background, theme.success);
    terminal.write(' '.repeat(width - 35), theme.background, theme.text);
    terminal.write('║', theme.border, theme.text);
    
    terminal.moveTo(x, y + 3);
    terminal.write('║', theme.border, theme.text);
    terminal.write(` Frei: ${data.totalLockers - data.occupiedLockers}`, theme.background, theme.info);
    terminal.write(' '.repeat(width - 20), theme.background, theme.text);
    terminal.write('║', theme.border, theme.text);
    
    terminal.moveTo(x, y + 4);
    terminal.write('╚' + '═'.repeat(width - 2) + '╝', theme.border, theme.text);
  }
  
  private renderStatistics(terminal: Terminal, x: number, y: number, width: number, height: number, data: DashboardData): void {
    const theme = Theme.defaultDark();
    
    terminal.moveTo(x, y);
    terminal.write('╔═══ Statistiken ' + '═'.repeat(width - 18) + '╗', theme.border, theme.text);
    
    // By size
    terminal.moveTo(x, y + 1);
    terminal.write('║', theme.border, theme.text);
    terminal.write(' Nach Größe:', theme.background, theme.textDim);
    terminal.write(' '.repeat(width - 14), theme.background, theme.text);
    terminal.write('║', theme.border, theme.text);
    
    let currentY = y + 2;
    for (const [size, stats] of Object.entries(data.bySize)) {
      terminal.moveTo(x, currentY++);
      terminal.write('║', theme.border, theme.text);
      terminal.write(`   ${size}: ${stats.occupied}/${stats.total}`, theme.background, theme.text);
      terminal.write(' '.repeat(width - 15), theme.background, theme.text);
      terminal.write('║', theme.border, theme.text);
    }
    
    terminal.moveTo(x, y + height - 1);
    terminal.write('╚' + '═'.repeat(width - 2) + '╝', theme.border, theme.text);
  }
  
  private renderAlerts(terminal: Terminal, x: number, y: number, width: number, height: number, data: DashboardData): void {
    const theme = Theme.defaultDark();
    
    terminal.moveTo(x, y);
    terminal.write('╔═══ Alarme ' + '═'.repeat(width - 13) + '╗', theme.border, theme.text);
    
    terminal.moveTo(x, y + 1);
    terminal.write('║', theme.border, theme.text);
    const overdueColor = data.overdueReturns > 0 ? theme.error : theme.success;
    terminal.write(` Überfällig: ${data.overdueReturns}`, theme.background, overdueColor);
    terminal.write(' '.repeat(width - 20), theme.background, theme.text);
    terminal.write('║', theme.border, theme.text);
    
    terminal.moveTo(x, y + 2);
    terminal.write('║', theme.border, theme.text);
    const expiringColor = data.expiringSoon > 0 ? theme.warning : theme.success;
    terminal.write(` Bald fällig: ${data.expiringSoon}`, theme.background, expiringColor);
    terminal.write(' '.repeat(width - 20), theme.background, theme.text);
    terminal.write('║', theme.border, theme.text);
    
    terminal.moveTo(x, y + 3);
    terminal.write('╚' + '═'.repeat(width - 2) + '╝', theme.border, theme.text);
  }
  
  private renderGraph(terminal: Terminal, x: number, y: number, width: number, height: number, data: DashboardData): void {
    const theme = Theme.defaultDark();
    
    terminal.moveTo(x, y);
    terminal.write('╔═══ Belegungstrend (30 Tage) ' + '═'.repeat(width - 32) + '╗', theme.border, theme.text);
    
    // Simple ASCII bar graph
    const graphHeight = height - 2;
    const maxPercent = 100;
    
    for (let i = 0; i < graphHeight; i++) {
      terminal.moveTo(x, y + 1 + i);
      terminal.write('║', theme.border, theme.text);
      
      // Draw graph bars (simplified)
      if (data.occupancyHistory.length > 0 && i < data.occupancyHistory.length) {
        const percent = data.occupancyHistory[i].percent;
        const barWidth = Math.floor((percent / maxPercent) * (width - 10));
        terminal.write(' ' + '█'.repeat(barWidth), theme.background, theme.accent);
        terminal.write(` ${percent.toFixed(1)}%`, theme.background, theme.text);
      }
      
      terminal.write(' '.repeat(width - 20), theme.background, theme.text);
      terminal.write('║', theme.border, theme.text);
    }
    
    terminal.moveTo(x, y + height - 1);
    terminal.write('╚' + '═'.repeat(width - 2) + '╝', theme.border, theme.text);
  }
}
```

---

## 7. Wizard System

### 7.1 Wizard Types

```typescript
// src/ui/widgets/wizard.ts
import type { Terminal } from '@opentui/core';
import { Theme } from '../theme';

export type MessageSender = 'system' | 'user';

export interface WizardOption {
  label: string;
  value: string;
  metadata?: Record<string, any>;
}

export type MessageContent =
  | { type: 'question'; text: string; options: WizardOption[] }
  | { type: 'answer'; text: string }
  | { type: 'info'; text: string };

export interface WizardMessage {
  sender: MessageSender;
  content: MessageContent;
  timestamp: number;
}

export type WizardAction = 
  | { type: 'none' }
  | { type: 'next' }
  | { type: 'complete'; data: Record<string, any> }
  | { type: 'cancel' };

export class WizardRenderer {
  private messages: WizardMessage[] = [];
  private currentQuestionIndex = -1;
  private selectedOptionIndex = 0;
  
  addMessage(message: WizardMessage): void {
    this.messages.push(message);
    
    if (message.content.type === 'question') {
      this.currentQuestionIndex = this.messages.length - 1;
      this.selectedOptionIndex = 0;
    }
  }
  
  render(terminal: Terminal, x: number, y: number, width: number, height: number): void {
    const theme = Theme.defaultDark();
    
    // Draw border
    terminal.moveTo(x, y);
    terminal.write('╔═══ Dialog ' + '═'.repeat(width - 13) + '╗', theme.border, theme.text);
    
    // Render messages (chat-style)
    let currentY = y + 1;
    const maxMessages = height - 3;
    const visibleMessages = this.messages.slice(-maxMessages);
    
    for (const message of visibleMessages) {
      if (currentY >= y + height - 2) break;
      
      currentY = this.renderMessage(terminal, message, x, currentY, width);
    }
    
    // Render current question options if present
    const currentQuestion = this.getCurrentQuestion();
    if (currentQuestion && currentQuestion.content.type === 'question') {
      currentY = this.renderQuestion(terminal, currentQuestion, x, currentY, width, y + height - 2);
    }
    
    // Bottom border
    terminal.moveTo(x, y + height - 1);
    terminal.write('╚' + '═'.repeat(width - 2) + '╝', theme.border, theme.text);
  }
  
  private renderMessage(terminal: Terminal, message: WizardMessage, x: number, y: number, width: number): number {
    const theme = Theme.defaultDark();
    const isSystem = message.sender === 'system';
    
    terminal.moveTo(x, y);
    terminal.write('║', theme.border, theme.text);
    
    // Render sender icon and message
    const icon = isSystem ? '🖥 ' : '👤 ';
    const textColor = isSystem ? theme.wizardSystem : theme.wizardUser;
    
    let text = '';
    if (message.content.type === 'question') {
      text = message.content.text;
    } else if (message.content.type === 'answer') {
      text = message.content.text;
    } else {
      text = message.content.text;
    }
    
    const maxTextWidth = width - 6;
    const truncatedText = text.substring(0, maxTextWidth).padEnd(maxTextWidth);
    
    terminal.write(icon + truncatedText, theme.background, textColor);
    terminal.write('║', theme.border, theme.text);
    
    return y + 1;
  }
  
  private renderQuestion(terminal: Terminal, message: WizardMessage, x: number, y: number, width: number, maxY: number): number {
    const theme = Theme.defaultDark();
    
    if (message.content.type !== 'question') return y;
    
    const options = message.content.options;
    let currentY = y;
    
    terminal.moveTo(x, currentY++);
    terminal.write('║' + ' '.repeat(width - 2) + '║', theme.border, theme.text);
    
    for (let i = 0; i < options.length && currentY < maxY; i++) {
      const option = options[i];
      const isSelected = i === this.selectedOptionIndex;
      
      terminal.moveTo(x, currentY++);
      terminal.write('║', theme.border, theme.text);
      
      const prefix = isSelected ? '▶ ' : '  ';
      const color = isSelected ? theme.wizardOptionSelected : theme.wizardOptionNormal;
      const label = (prefix + option.label).substring(0, width - 4).padEnd(width - 4);
      
      terminal.write(label, theme.background, color);
      terminal.write('║', theme.border, theme.text);
    }
    
    return currentY;
  }
  
  handleKey(key: string): WizardAction {
    const currentQuestion = this.getCurrentQuestion();
    
    if (!currentQuestion || currentQuestion.content.type !== 'question') {
      return { type: 'none' };
    }
    
    const options = currentQuestion.content.options;
    
    switch (key) {
      case 'up':
      case 'k':
        this.selectedOptionIndex = (this.selectedOptionIndex - 1 + options.length) % options.length;
        return { type: 'none' };
        
      case 'down':
      case 'j':
        this.selectedOptionIndex = (this.selectedOptionIndex + 1) % options.length;
        return { type: 'none' };
        
      case 'enter':
        const selectedOption = options[this.selectedOptionIndex];
        this.addMessage({
          sender: 'user',
          content: { type: 'answer', text: selectedOption.label },
          timestamp: Date.now()
        });
        return { type: 'next' };
        
      case 'escape':
        return { type: 'cancel' };
        
      default:
        return { type: 'none' };
    }
  }
  
  private getCurrentQuestion(): WizardMessage | null {
    if (this.currentQuestionIndex >= 0 && this.currentQuestionIndex < this.messages.length) {
      return this.messages[this.currentQuestionIndex];
    }
    return null;
  }
  
  getSelectedOption(): WizardOption | null {
    const question = this.getCurrentQuestion();
    
    if (question && question.content.type === 'question') {
      return question.content.options[this.selectedOptionIndex];
    }
    
    return null;
  }
}
```

---

## 8. Theme System

```typescript
// src/ui/theme.ts

export interface ThemeColors {
  // Base
  background: string;
  foreground: string;
  text: string;
  textDim: string;
  
  // Accent
  primary: string;
  secondary: string;
  accent: string;
  
  // Status
  success: string;
  warning: string;
  error: string;
  info: string;
  
  // UI Elements
  border: string;
  borderActive: string;
  headerBg: string;
  headerFg: string;
  
  // Keybinds
  keybindGlobal: string;
  keybindContext: string;
  
  // Window Switcher
  windowCurrent: string;
  windowAdjacent: string;
  
  // Wizard
  wizar
dSystem: string;
  wizardUser: string;
  wizardOptionSelected: string;
  wizardOptionNormal: string;
  
  // Status Bar
  statusInfo: string;
  statusSuccess: string;
  statusWarning: string;
  statusError: string;
  
  // Escape Indicator
  escapeIndicatorActive: string;
  escapeIndicatorInactive: string;
  
  // Screensaver
  screensaverBg: string;
  screensaverFg: string;
}

export class Theme {
  private constructor(private colors: ThemeColors) {}
  
  static defaultDark(): Theme {
    return new Theme({
      // Base
      background: '#1e1e2e',
      foreground: '#cdd6f4',
      text: '#cdd6f4',
      textDim: '#6c7086',
      
      // Accent
      primary: '#89b4fa',
      secondary: '#94e2d5',
      accent: '#f38ba8',
      
      // Status
      success: '#a6e3a1',
      warning: '#f9e2af',
      error: '#f38ba8',
      info: '#89dceb',
      
      // UI Elements
      border: '#45475a',
      borderActive: '#89b4fa',
      headerBg: '#313244',
      headerFg: '#cdd6f4',
      
      // Keybinds
      keybindGlobal: '#89b4fa',
      keybindContext: '#f9e2af',
      
      // Window Switcher
      windowCurrent: '#a6e3a1',
      windowAdjacent: '#6c7086',
      
      // Wizard
      wizardSystem: '#89dceb',
      wizardUser: '#f5c2e7',
      wizardOptionSelected: '#a6e3a1',
      wizardOptionNormal: '#cdd6f4',
      
      // Status Bar
      statusInfo: '#89dceb',
      statusSuccess: '#a6e3a1',
      statusWarning: '#f9e2af',
      statusError: '#f38ba8',
      
      // Escape Indicator
      escapeIndicatorActive: '#f38ba8',
      escapeIndicatorInactive: '#45475a',
      
      // Screensaver
      screensaverBg: '#000000',
      screensaverFg: '#a6e3a1'
    });
  }
  
  get(key: keyof ThemeColors): string {
    return this.colors[key];
  }
}
```

---

## 9. Export/Import System

### 9.1 TOML Export

```typescript
// src/export/toml.ts
import { DatabaseConnection } from '../db/connection';
import type { Locker, Rental } from '../db/schema';

export function exportToTOML(): string {
  const db = DatabaseConnection.getConnection();
  
  // Metadata
  const metadata = {
    version: '2.1.0',
    export_date: new Date().toISOString(),
    database_version: '3'
  };
  
  // Lockers
  const lockers = db.prepare('SELECT * FROM lockers').all() as Locker[];
  
  // Rentals
  const rentals = db.prepare('SELECT * FROM rentals').all() as Rental[];
  
  // Build TOML
  let toml = '[metadata]\n';
  toml += `version = "${metadata.version}"\n`;
  toml += `export_date = "${metadata.export_date}"\n`;
  toml += `database_version = "${metadata.database_version}"\n\n`;
  
  // Lockers
  for (const locker of lockers) {
    toml += '[[lockers]]\n';
    toml += `id = ${locker.id}\n`;
    toml += `number = "${locker.number}"\n`;
    toml += `location = "${locker.location}"\n`;
    toml += `size = "${locker.size}"\n`;
    toml += `is_damaged = ${locker.is_damaged}\n`;
    if (locker.notes) toml += `notes = "${locker.notes}"\n`;
    toml += `created_at = "${locker.created_at}"\n\n`;
  }
  
  // Rentals
  for (const rental of rentals) {
    toml += '[[rentals]]\n';
    toml += `id = ${rental.id}\n`;
    toml += `locker_id = ${rental.locker_id}\n`;
    toml += `renter_name = "${rental.renter_name}"\n`;
    if (rental.renter_email) toml += `renter_email = "${rental.renter_email}"\n`;
    if (rental.renter_phone) toml += `renter_phone = "${rental.renter_phone}"\n`;
    toml += `start_date = "${rental.start_date}"\n`;
    if (rental.end_date) toml += `end_date = "${rental.end_date}"\n`;
    toml += `deposit_paid = ${rental.deposit_paid}\n`;
    toml += `deposit_returned = ${rental.deposit_returned}\n`;
    if (rental.notes) toml += `notes = "${rental.notes}"\n`;
    toml += `created_at = "${rental.created_at}"\n\n`;
  }
  
  return toml;
}
```

---

## 10. Main Application

### 10.1 Application State

```typescript
// src/app.ts
import type { Terminal } from '@opentui/core';
import { DatabaseConnection } from './db/connection';
import { runMigrations } from './db/migrations';
import { Header } from './ui/widgets/header';
import { KeybindBar, type Keybind } from './ui/widgets/keybind-bar';
import { StatusBar } from './ui/widgets/status-bar';
import { DashboardScreen } from './ui/screens/dashboard';
import { ScreensaverScreen } from './ui/screens/screensaver';
import { calculateLayout } from './ui';

export type AppScreen = 
  | 'dashboard' 
  | 'rental' 
  | 'finances' 
  | 'management' 
  | 'screensaver';

export class App {
  private currentScreen: AppScreen = 'dashboard';
  private header: Header;
  private keybindBar: KeybindBar;
  private statusBar: StatusBar;
  private dashboardScreen: DashboardScreen;
  private screensaverScreen: ScreensaverScreen;
  
  // Window switcher
  private windowSwitcherActive = false;
  private selectedWindowIndex = 0;
  
  // Screensaver
  private lastActivity = Date.now();
  private screensaverTimeout = 60000;
  private countdownDuration = 15000;
  private screensaverActive = false;
  
  // State
  private running = true;
  
  constructor() {
    // Initialize database
    const db = DatabaseConnection.getConnection();
    runMigrations(db);
    
    // Load screensaver timeout from settings
    const timeoutSetting = db.prepare('SELECT value FROM settings WHERE key = ?')
      .get('screensaver_timeout_seconds') as { value: string } | undefined;
    
    if (timeoutSetting) {
      this.screensaverTimeout = parseInt(timeoutSetting.value, 10) * 1000;
    }
    
    //
 Initialize UI components
    this.header = new Header({
      version: '2.1.0',
      currentScreen: 'Dashboard',
      windowSwitcherActive: false,
      windowList: this.getWindowList(),
      selectedWindowIndex: 0
    });
    
    this.keybindBar = new KeybindBar(this.getGlobalKeybinds());
    this.statusBar = new StatusBar();
    this.dashboardScreen = new DashboardScreen();
    this.screensaverScreen = new ScreensaverScreen();
    
    this.updateKeybinds();
  }
  
  render(terminal: Terminal): void {
    const width = terminal.width();
    const height = terminal.height();
    
    terminal.clear();
    
    if (this.screensaverActive) {
      this.screensaverScreen.render(terminal, width, height);
      return;
    }
    
    // Calculate layout
    const layout = calculateLayout(width, height);
    
    // Render components
    this.header.render(terminal, layout.header.x, layout.header.y, layout.header.width);
    
    // Render current screen
    switch (this.currentScreen) {
      case 'dashboard':
        this.dashboardScreen.render(
          terminal,
          layout.content.x,
          layout.content.y,
          layout.content.width,
          layout.content.height
        );
        break;
      // ... other screens
    }
    
    this.keybindBar.render(
      terminal,
      layout.keybindBar.x,
      layout.keybindBar.y,
      layout.keybindBar.width,
      layout.keybindBar.height
    );
    
    this.statusBar.render(
      terminal,
      layout.statusBar.x,
      layout.statusBar.y,
      layout.statusBar.width
    );
  }
  
  handleKey(key: string): void {
    this.updateActivity();
    
    // Handle window switcher
    if (this.windowSwitcherActive) {
      this.handleWindowSwitcherKey(key);
      return;
    }
    
    // Handle escape key (triple escape to dashboard)
    if (key === 'escape') {
      this.statusBar.incrementEscapeCount();
      
      if (this.statusBar.shouldReturnToDashboard()) {
        this.currentScreen = 'dashboard';
        this.statusBar.resetEscapeCount();
        this.statusBar.setMessage('Zurück zum Dashboard', 'info');
      }
      
      return;
    }
    
    // Reset escape count on other keys
    this.statusBar.resetEscapeCount();
    
    // Global keybinds
    switch (key) {
      case '^':
        this.activateWindowSwitcher();
        break;
      case 'q':
        this.running = false;
        break;
      default:
        this.handleScreenKey(key);
    }
  }
  
  private handleWindowSwitcherKey(key: string): void {
    switch (key) {
      case 'tab':
        this.selectedWindowIndex = (this.selectedWindowIndex + 1) % this.getWindowList().length;
        break;
      case 'shift+tab':
        this.selectedWindowIndex = 
          (this.selectedWindowIndex - 1 + this.getWindowList().length) % this.getWindowList().length;
        break;
      case 'enter':
        this.switchToSelectedWindow();
        break;
      case 'escape':
        this.deactivateWindowSwitcher();
        break;
    }
  }
  
  private handleScreenKey(key: string): void {
    // Screen-specific key handling
    if (this.currentScreen === 'dashboard') {
      switch (key) {
        case '1':
          this.currentScreen = 'rental';
          break;
        case '2':
          this.currentScreen = 'finances';
          break;
        case '3':
          this.currentScreen = 'management';
          break;
      }
    }
  }
  
  update(): void {
    // Check inactivity
    const state = this.checkInactivity();
    
    if (state.type === 'countdown') {
      this.statusBar.showScreensaverCountdown(state.seconds);
    } else if (state.type === 'screensaver' && !this.screensaverActive) {
      this.activateScreensaver();
    }
    
    // Update screensaver animation
    if (this.screensaverActive) {
      this.screensaverScreen.update();
    }
  }
  
  isRunning(): boolean {
    return this.running;
  }
  
  // ... helper methods (getWindowList, updateKeybinds, etc.)
  
  private getWindowList() {
    return [
      { name: 'Dashboard', screen: 'dashboard' },
      { name: 'Verleih', screen: 'rental' },
      { name: 'Finanzen', screen: 'finances' },
      { name: 'Verwaltung', screen: 'management' }
    ];
  }
  
  private getGlobalKeybinds(): Keybind[] {
    return [
      { key: '^', description: 'Fenster wechseln', scope: 'global' },
      { key: 'Esc×3', description: 'Dashboard', scope: 'global' },
      { key: 'q', description: 'Beenden', scope: 'global' }
    ];
  }
  
  private updateKeybinds(): void {
    const contextBinds: Keybind[] = [];
    
    if (this.currentScreen === 'dashboard') {
      contextBinds.push(
        { key: '1', description: 'Verleih', scope: 'context' },
        { key: '2', description: 'Finanzen', scope: 'context' },
        { key: '3', description: 'Verwaltung', scope: 'context' }
      );
    }
    
    this.keybindBar.updateContext(contextBinds, this.currentScreen);
  }
  
  private activateWindowSwitcher(): void {
    this.windowSwitcherActive = true;
    this.selectedWindowIndex = this.getWindowList()
      .findIndex(w => w.screen === this.currentScreen);
  }
  
  private deactivateWindowSwitcher(): void {
    this.windowSwitcherActive = false;
  }
  
  private switchToSelectedWindow(): void {
    const windows = this.getWindowList();
    this.currentScreen = windows[this.selectedWindowIndex].screen as AppScreen;
    this.deactivateWindowSwitcher();
  }
  
  private updateActivity(): void {
    this.lastActivity = Date.now();
    
    if (this.screensaverActive) {
      this.exitScreensaver();
    }
  }
  
  private checkInactivity() {
    const inactiveDuration = Date.now() - this.lastActivity;
    
    if (inactiveDuration >= this.screensaverTimeout + this.countdownDuration) {
      return { type: 'screensaver' as const };
    } else if (inactiveDuration >= this.screensaverTimeout) {
      const remaining = Math.ceil(
        (this.countdownDuration - (inactiveDuration - this.screensaverTimeout)) / 1000
      );
      return { type: 'countdown' as const, seconds: remaining };
    } else {
      return { type: 'active' as const };
    }
  }
  
  private activateScreensaver(): void {
    this.screensaverActive = true;
    this.currentScreen = 'screensaver';
  }
  
  private exitScreensaver(): void {
    this.screensaverActive = false;
    this.currentScreen = 'dashboard';
  }
}
```

### 10.2 Main Entry Point

```typescript
// src/index.ts
import { createTerminal } from './ui';
import { App } from './app';

async function main() {
  const terminal = createTerminal();
  const app = new App();
  
  // Setup input handling
  terminal.on('key', (key: string) => {
    app.handleKey(key);
  });
  
  // Main loop
  const targetFPS = 60;
  const frameTime = 1000 / targetFPS;
  
  while (app.isRunning()) {
    const startTime = Date.now();
    
    // Update
    app.update();
    
    // Render
    app.render(terminal);
    
    // Frame limiting
    const elapsed = Date.now() - startTime;
    const remaining = frameTime - elapsed;
    
    if (remaining > 0) {
      await Bun.sleep(remaining);
    }
  }
  
  // Cleanup
  terminal.clear();
  terminal.restore();
  process.exit(0);
}

main().catch(console.error);
```

---

## 11. Tests

### 11.1 Test Setup

```typescript
// tests/helpers/test-utils.ts
import { beforeEach, afterEach } from 'bun:test';
import { DatabaseConnection } from '../../src/db/connection';
import { runMigrations } from '../../src/db/migrations';

export function setupTestDatabase() {
  beforeEach(() => {
    // Use in-memory database for tests
    process.env.TEST_DB = ':memory:';
    const db = DatabaseConnection.getConnection();
    runMigrations(db);
  });
  
  afterEach(() => {
    DatabaseConnection.close();
  });
}
```

### 11.2 Example Tests

```typescript
// tests/ui/interaction.test.ts
import { describe, it, expect } from 'bun:test';
import { App } from '../../src/app';
import { setupTestDatabase } from '../helpers/test-utils';

describe('UI Interaction', () => {
  setupTestDatabase();
  
  it('should activate window switcher on ^', () => {
    const app = new App();
    
    app.handleKey('^');
    
    // Assert window switcher is active
    expect(app['windowSwitcherActive']).toBe(true);
  });
  
  it('should return to dashboard after 3 escapes', () => {
    const app = new App();
    app['currentScreen'] = 'rental';
    
    app.handleKey('escape');
    app.handleKey('escape');
    app.handleKey('escape');
    
    expect(app['currentScreen']).toBe('dashboard');
  });
  
  it('should reset escape counter on other key', () => {
    const app = new App();
    
    app.handleKey('escape');
    app.handleKey('escape');
    app.handleKey('a'); // Different key
    
    expect(app['statusBar']['escapeCount']).toBe(0);
  });
  
  it('should activate screensaver after timeout', async () => {
    const app = new App();
    app['screensaverTimeout'] = 100; // 100ms for testing
    app['countdownDuration'] = 50;
    
    await Bun.sleep(160);
    app.update();
    
    expect(app['screensaverActive']).toBe(true);
  });
});
```

---

## 12. Build & Deployment

### 12.1 Building with Bun

```bash
# Development
bun run dev

# Build single executable
bun build src/index.ts --compile --outfile dist/schliessfach-manager

# The --compile flag creates a standalone executable with Bun runtime embedded
```

### 12.2 Distribution

```bash
# Linux
./dist/schliessfach-manager

# macOS
./dist/schliessfach-manager

# Windows
./dist/schliessfach-manager.exe
```

The compiled binary includes:
- All TypeScript code (transpiled)
- Bun runtime
- SQLite (better-sqlite3 native module)
- All dependencies

**File size:** ~50-70MB (self-contained)

---

## 13. Implementierungs-Checkliste

### Phase 1: Basis-Setup (Priorität: Hoch)
- [ ] Projekt initialisieren (package.json, tsconfig.json, bunfig.toml)
- [ ] Datenbank-Layer (connection, migrations, schema types)
- [ ] OpenTUI Setup und Layout-System
- [ ] Theme-System implementieren

### Phase 2: Globale UI (Priorität: Hoch)
- [ ] Header-Komponente
- [ ] Keybind Bar
- [ ] Status Bar mit Escape-Indikator
- [ ] Window Switcher
- [ ] 3x Escape = Dashboard Feature

### Phase 3: Dashboard (Priorität: Hoch)
- [ ] Dashboard-Datenabfragen
- [ ] Fixed-Height Layout
- [ ] Statistik-Widgets
- [ ] Trend-Graph

### Phase 4: Screensaver (Priorität: Mittel)
- [ ] Inaktivitäts-Tracking
- [ ] 5 ASCII-Animationen
- [ ] AnimationPlayer
- [ ] Screensaver Screen
- [ ] Integration in Main Loop

### Phase 5: Wizards (Priorität: Mittel)
- [ ] WizardRenderer
- [ ] Dialog-Style Messages
- [ ] Keyboard Navigation
- [ ] Rental Workflow
- [ ] Extend Workflow
- [ ] Return Workflow

### Phase 6: Weitere Screens (Priorität: Mittel)
- [ ] Rental Management Screen
- [ ] Finances Screen
- [ ] Management Screen
- [ ] Settings Screen

### Phase 7: Export/Import (Priorität: Niedrig)
- [ ] TOML Export/Import
- [ ] JSON Export/Import
- [ ] CSV Export/Import
- [ ] Markdown Reports

### Phase 8: Audit & History (Priorität: Niedrig)
- [ ] Audit Logging
- [ ] Audit Log Viewer
- [ ] Occupancy History
- [ ] Historical Analytics

### Phase 9: Tests (Priorität: Hoch)
- [ ] Unit Tests (DB, Workflows)
- [ ] Integration Tests
- [ ] UI Interaction Tests
- [ ] Coverage >85%

### Phase 10: Dokumentation (Priorität: Mittel)
- [ ] README.md
- [ ] USER_GUIDE.md
- [ ] KEYBINDINGS.md
- [ ] CHANGELOG.md
- [ ] Inline-Dokumentation

### Phase 11: Build & Polish (Priorität: Mittel)
- [ ] Bun Build konfigurieren
- [ ] Cross-platform Tests
- [ ] Performance-Optimierung
- [ ] Error Handling
- [ ] Loading States

---

## 14. Zusammenfassung

Diese TypeScript-Spezifikation portiert die komplette v2.1 Rust-Spezifikation auf:
- **TypeScript** für Typsicherheit
- **OpenTUI** für Terminal UI
- **Bun** für Build und Runtime
- **better-sqlite3** für Datenbank
- **Zod** für Runtime-Validierung

**Identisches UI:** Alle Screens, Widgets, Layouts und Features bleiben gleich.

**Identische Features:**
- ✅ Fixed-Height Dashboard
- ✅ 3x Escape = Dashboard
- ✅ Window Switcher (^)
- ✅ Screensaver (5 Animationen)
- ✅ Dialog-Style Wizards
- ✅ Global Keybind Bar
- ✅ Status Bar mit Escape-Indikator
- ✅ TOML/JSON/CSV Export/Import
- ✅ Audit Logging
- ✅ Occupancy History
- ✅ Konfigurierbare Einstellungen

**Build-Output:** Einzelne ausführbare Datei (~50-70MB) mit embedded Runtime.

**Viel Erfolg bei der Implementierung! 🚀**