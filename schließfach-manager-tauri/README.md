# Schließfach-Manager v2.1-Tauri

Eine moderne Desktop-Anwendung zur Verwaltung von Schließfächern, gebaut mit Tauri, React und TypeScript.

![Version](https://img.shields.io/badge/version-2.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)

## 🚀 Features

- **Dashboard**: Übersicht über Belegung, Umsatz und Alarme
- **Verleih-Management**: Neue Verleihe, Verlängerungen, Rückgaben
- **Schließfach-Verwaltung**: CRUD für Schließfächer mit Größen und Standorten
- **Finanz-Übersicht**: Zahlungen und Berichte
- **Export/Import**: TOML, JSON, CSV Formate
- **Audit-Log**: Protokollierung aller Aktionen
- **Keyboard-Shortcuts**: Schnelle Navigation

## 🎨 Design

Das UI basiert auf dem **Catppuccin Mocha** Farbschema - einem modernen, augenschonenden Dark Theme.

## 📦 Installation

### Voraussetzungen

- Node.js 18+ 
- Rust 1.70+
- Tauri CLI

### Entwicklung starten

```bash
# Dependencies installieren
npm install

# Tauri Entwicklungsserver starten
npm run tauri dev
```

### Produktion-Build

```bash
# Für alle Plattformen bauen
npm run tauri build
```

## 🏗️ Projekt-Struktur

```
schließfach-manager-tauri/
├── src-ui/                 # React Frontend
│   ├── components/         # UI-Komponenten
│   │   ├── layout/         # TopBar, Sidebar, StatusBar
│   │   ├── ui/             # Button, Input, Card, etc.
│   │   ├── wizards/        # Multi-Step Formulare
│   │   └── charts/         # Diagramme
│   ├── pages/              # Seiten-Komponenten
│   ├── hooks/              # React Query Hooks
│   ├── lib/                # Utilities und Tauri API
│   ├── types/              # TypeScript Typen
│   ├── schemas/            # Zod Validierung
│   └── styles/             # CSS und Theme
├── src-tauri/              # Rust Backend
│   └── src/
│       ├── commands/       # Tauri Commands
│       ├── db/             # Datenbank-Layer
│       └── services/       # Business-Logik
├── package.json
└── README.md
```

## 🔧 Technologie-Stack

### Frontend
- **React 18** - UI Framework
- **TypeScript** - Type Safety
- **Tailwind CSS** - Styling
- **React Router** - Navigation
- **TanStack Query** - Server State
- **TanStack Table** - Tabellen
- **React Hook Form** - Formulare
- **Zod** - Validierung
- **Recharts** - Diagramme
- **Sonner** - Toast Notifications
- **Lucide React** - Icons

### Backend
- **Tauri 2.0** - Desktop Framework
- **Rust** - Backend Language
- **SQLite** - Datenbank
- **rusqlite** - SQLite Binding
- **serde** - Serialisierung
- **chrono** - Datum/Zeit

## ⌨️ Keyboard Shortcuts

| Shortcut | Aktion |
|----------|--------|
| `Strg+H` | Dashboard |
| `Strg+N` | Neuer Verleih |
| `Strg+,` | Einstellungen |
| `Strg+?` | Shortcuts anzeigen |
| `Esc` | Dialog schließen |

## 📊 Datenbank-Schema

### Tabellen
- `lockers` - Schließfächer (id, number, location, size, is_damaged, notes)
- `rentals` - Verleihe (id, locker_id, renter_*, start_date, end_date, deposit_*)
- `payments` - Zahlungen (id, rental_id, amount_cents, payment_type)
- `settings` - Einstellungen (key, value)
- `audit_log` - Audit-Protokoll (id, timestamp, action, entity_*, details)

## 🔒 Sicherheit

- Lokale SQLite-Datenbank (keine Cloud)
- Pfad: `~/.local/share/schließfach-manager/lockers.db`
- Verschlüsselung optional aktivierbar

## 📝 Lizenz

MIT License - siehe [LICENSE](LICENSE)

## 👥 Beitragende

- DJL Foundation

## 📞 Support

Bei Fragen oder Problemen erstellen Sie bitte ein [Issue](../../issues).
