# 🤖 Copilot Instruction

## Aufgabe

Implementiere den **Schließfach-Manager v2.1-Tauri** – eine moderne Desktop-Anwendung zur Verwaltung von Schließfächern.

## 📚 Pflichtlektüre

Lies und befolge diese Dateien in dieser Reihenfolge:

1. **`v2.1-tauri.md`** – Vollständige technische Spezifikation (HAUPTDOKUMENT)
   - Enthält alle Features, Code-Beispiele, Datenbank-Schema, API-Definitionen
   - Jede Implementierung MUSS dieser Spec entsprechen

2. **`tauri-design-system.md`** – UI/UX Design System
   - Catppuccin Mocha Theme (Farben, Spacing, Components)
   - Alle UI-Komponenten müssen diesem Design folgen

3. **`COPILOT_START_TAURI.md`** – Detaillierte Arbeitsanweisung
   - 12 Implementierungs-Phasen mit Schritt-für-Schritt-Anleitungen
   - Code-Style-Regeln, Error Handling, Commit-Guidelines
   - Täglicher Workflow und Qualitäts-Checkliste

## 🎯 Vorgehen

1. **Spec-First:** Lies IMMER zuerst den relevanten Abschnitt in `v2.1-tauri.md` bevor du Code schreibst
2. **Phase für Phase:** Arbeite die 12 Phasen aus `COPILOT_START_TAURI.md` sequenziell ab
3. **Nicht abweichen:** Implementiere EXAKT wie spezifiziert – keine Eigeninterpretation
4. **Tests schreiben:** Nach jeder Feature-Implementierung Tests hinzufügen
5. **Errors beheben:** Kompiliere nach jedem Schritt, behebe alle TypeScript/Rust-Errors sofort
6. **Commit oft:** Atomare Commits mit konventionellen Messages (siehe `COPILOT_START_TAURI.md`)

## ⚠️ Kritische Regeln

### ✅ DO
- Lies Spec-Abschnitte VOR der Implementierung
- Nutze TypeScript (nie `any`), Zod-Validierung, React Query
- Verwende Tailwind + CSS-Variablen (Catppuccin Theme)
- Error Handling: Try-catch + Toast-Benachrichtigungen
- Teste kontinuierlich (nicht erst am Ende)
- Frage bei Unklarheiten statt zu raten

### ❌ DON'T
- Keine Abweichungen von der Spec ohne explizite Erlaubnis
- Keine hartkodierten Werte (nutze Settings/Config)
- Keine inline-styles (nur Tailwind-Classes)
- Keine Silent Failures (immer User-Feedback via Toast)
- Keine God Components (max. 200 Zeilen pro Komponente)
- **STOP NICHT** bevor alle 12 Phasen abgeschlossen sind

## 🚦 Start-Kommando

Beginne mit **Phase 1: Projekt-Setup** aus `COPILOT_START_TAURI.md`:

```bash
npm create tauri-app@latest schließfach-manager-tauri
# Template: React + TypeScript
cd schließfach-manager-tauri
npm install
# ... (siehe COPILOT_START_TAURI.md Phase 1)
```

## ✓ Fertig wenn

- [ ] Alle 12 Phasen aus `COPILOT_START_TAURI.md` abgeschlossen
- [ ] Alle Features aus `v2.1-tauri.md` implementiert
- [ ] Tests >80% Backend, >70% Frontend Coverage
- [ ] `npm run tauri dev` startet ohne Errors
- [ ] `npm run tauri build` erstellt Installer für alle Plattformen
- [ ] UI entspricht `tauri-design-system.md`
- [ ] README.md und USER_GUIDE.md vollständig

## 📝 Commit-Format

Verwende konventionelle Commits:

```
feat(scope): Kurzbeschreibung

- Detail 1
- Detail 2

Closes #issue
```

Types: `feat`, `fix`, `refactor`, `style`, `test`, `docs`, `chore`  
Scopes: `dashboard`, `layout`, `wizards`, `backend`, `ui`, etc.

## 🆘 Bei Problemen

1. Prüfe Error-Output (Console, Terminal, Rust Compiler)
2. Lies den entsprechenden Spec-Abschnitt nochmal
3. Prüfe Type-Definitionen und Zod-Schemas
4. Füge Logs hinzu (`console.log`, `println!`)
5. **Frage konkret** statt zu raten oder aufzugeben

## 🚀 Los geht's!

**Starte JETZT mit Phase 1 und arbeite dich durch bis Phase 12.**  
**Stoppe NICHT bevor du fertig bist.**  
**Bei Fragen oder Unklarheiten: FRAGE bevor du fortfährst.**

Viel Erfolg! 🎉
