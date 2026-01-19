# Schließfach-Manager v2.1

> Terminal-basierte Verwaltung von Schließfächern für Schulen, Vereine und Organisationen

## Features

- 📦 **Schließfachverwaltung** - Erstellen, verwalten und verfolgen von Schließfächern
- 🔑 **Verleihsystem** - Einfacher Verleih mit Wizard-Interface
- 💰 **Finanzverwaltung** - Pfand- und Zahlungstracking
- 📊 **Dashboard** - Übersicht über Belegung und Alarme
- 📈 **Statistiken** - Belegungstrend und Standortauswertung
- 🖥️ **Screensaver** - 5 verschiedene Animationen nach Inaktivität
- 📤 **Export/Import** - JSON, CSV, TOML und Markdown-Berichte
- 📝 **Audit-Log** - Vollständige Protokollierung aller Aktionen

## Requirements

- [Bun](https://bun.sh/) v1.0+ Runtime
- Terminal mit ANSI-Farbunterstützung

## Installation

```bash
# Clone the repository
git clone https://github.com/DJL-Foundation/schlie-fach-manager.git
cd schlie-fach-manager

# Install dependencies
bun install

# Run the application
bun run dev
```

## Build

```bash
# Build standalone binary
bun run build

# Run the binary
./dist/schliessfach-manager
```

## Usage

### Starting the Application

```bash
bun run dev
```

### Navigation

- **^** (Circumflex) - Öffnet den Window Switcher
- **1-3** - Schnellnavigation zu Screens (auf Dashboard)
- **Esc×3** - Zurück zum Dashboard von jedem Screen
- **q** - Anwendung beenden

### Window Switcher

- **Tab** / **Shift+Tab** - Zwischen Fenstern wechseln
- **Enter** - Fenster auswählen
- **Esc** - Abbrechen

## Architecture

```
src/
├── db/           # Database layer (bun:sqlite)
├── ui/           # UI components
│   ├── screens/  # Screen components
│   └── widgets/  # Reusable widgets
├── workflows/    # Business logic workflows
├── export/       # Export modules (JSON, CSV, TOML, Markdown)
├── import/       # Import modules (JSON, TOML)
├── screensaver/  # Screensaver animations
└── types/        # TypeScript type definitions
```

## Technology Stack

- **TypeScript 5.3+** - Type safety
- **Bun** - Runtime and build tool
- **bun:sqlite** - SQLite database
- **Zod** - Runtime schema validation
- **date-fns** - Date handling

## Testing

```bash
# Run all tests
bun test

# Run tests in watch mode
bun test --watch
```

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

---

Made with ❤️ by DJL Foundation
