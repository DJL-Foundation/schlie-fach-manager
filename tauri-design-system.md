# Schließfach-Manager – Tauri GUI Design System
# v2.1 – Modern Desktop Application

**Version:** 2.1-tauri  
**Datum:** 2025-01-20  
**Basis:** v2.1 Ratatui TUI Spec  
**Framework:** Tauri 2.0 + React + TypeScript + Tailwind CSS

---

## 1. Einführung & Konzept

### 1.1 Vision

Der Schließfach-Manager als **moderne Desktop-Anwendung** mit Tauri:
- **Native Performance** durch Rust Backend
- **Modern UI** mit React + Tailwind CSS
- **Kleine Binary** (~5-10MB statt 150MB bei Electron)
- **Sidebar Navigation** statt TUI Window Switcher
- **Buttons & Forms** statt Keyboard-only Navigation
- **Identische Funktionalität** wie TUI-Version

### 1.2 Layout-Philosophie

**Von TUI zu GUI Mapping:**

| TUI Element | GUI Element | Beschreibung |
|-------------|-------------|--------------|
| Header | Top Bar | App-Name + Breadcrumb |
| Window Switcher (^) | Sidebar | Immer sichtbar, kategorie-basiert |
| Tabs | Content Pages | Seiten innerhalb Kategorien |
| Keybind Bar | Tooltips + Shortcuts | Keyboard bleibt unterstützt |
| Status Bar | Bottom Status Bar | Notifications + Status |
| Escape Indicator | N/A | Nicht nötig in GUI |

---

## 2. Globales Layout

### 2.1 Layout-Struktur

```
┌─────────────────────────────────────────────────────────────────┐
│ TOP BAR (fixed height: 64px)                                    │
│ ├─ Logo + App Name                                              │
│ └─ Breadcrumb Navigation                                        │
├──────┬──────────────────────────────────────────────────────────┤
│      │                                                           │
│ SIDE │  MAIN CONTENT AREA                                       │
│ BAR  │  ├─ Dynamic content based on selection                   │
│      │  ├─ Cards, Tables, Forms, Wizards                        │
│ 240px│  └─ Action buttons                                       │
│      │                                                           │
│      │                                                           │
├──────┴──────────────────────────────────────────────────────────┤
│ STATUS BAR (fixed height: 32px)                                 │
│ ├─ Status messages (left)                                       │
│ └─ App info (right: version, DB status, etc.)                   │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Responsive Breakpoints

```css
/* Tailwind Breakpoints */
sm: 640px   // Minimum supported width
md: 768px   // Tablet
lg: 1024px  // Desktop
xl: 1280px  // Large Desktop
2xl: 1536px // Ultra-wide
```

**Minimum Window Size:** 900x600px  
**Optimal:** 1280x800px oder größer

---

## 3. Theme System

### 3.1 Color Palette (Catppuccin Mocha inspired)

```css
/* Base Colors */
--color-bg-base: #1e1e2e;        /* Main background */
--color-bg-surface: #313244;     /* Cards, sidebar */
--color-bg-overlay: #45475a;     /* Overlays, modals */

--color-text-primary: #cdd6f4;   /* Main text */
--color-text-secondary: #a6adc8; /* Secondary text */
--color-text-muted: #6c7086;     /* Muted text */

/* Accent Colors */
--color-primary: #89b4fa;        /* Blue - Primary actions */
--color-secondary: #94e2d5;      /* Teal - Secondary actions */
--color-accent: #f38ba8;         /* Pink - Highlights */

/* Status Colors */
--color-success: #a6e3a1;        /* Green - Success states */
--color-warning: #f9e2af;        /* Yellow - Warnings */
--color-error: #f38ba8;          /* Red - Errors */
--color-info: #89dceb;           /* Sky - Info messages */

/* UI Elements */
--color-border: #45475a;         /* Default borders */
--color-border-focus: #89b4fa;   /* Focused borders */
--color-border-hover: #6c7086;   /* Hover borders */

/* Sidebar */
--sidebar-bg: #313244;
--sidebar-item-hover: #45475a;
--sidebar-item-active: #89b4fa;
--sidebar-item-text: #cdd6f4;
--sidebar-item-text-active: #ffffff;

/* Button States */
--btn-primary-bg: #89b4fa;
--btn-primary-hover: #7aa3e5;
--btn-primary-active: #6b93d6;
--btn-primary-text: #1e1e2e;

--btn-secondary-bg: #45475a;
--btn-secondary-hover: #585b70;
--btn-secondary-active: #6c7086;
--btn-secondary-text: #cdd6f4;

--btn-danger-bg: #f38ba8;
--btn-danger-hover: #e47996;
--btn-danger-active: #d56784;
--btn-danger-text: #1e1e2e;
```

### 3.2 Typography

```css
/* Font Stack */
--font-sans: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
--font-mono: 'JetBrains Mono', 'Fira Code', 'Monaco', monospace;

/* Font Sizes */
--text-xs: 0.75rem;    /* 12px */
--text-sm: 0.875rem;   /* 14px */
--text-base: 1rem;     /* 16px */
--text-lg: 1.125rem;   /* 18px */
--text-xl: 1.25rem;    /* 20px */
--text-2xl: 1.5rem;    /* 24px */
--text-3xl: 1.875rem;  /* 30px */

/* Font Weights */
--font-normal: 400;
--font-medium: 500;
--font-semibold: 600;
--font-bold: 700;

/* Line Heights */
--leading-tight: 1.25;
--leading-normal: 1.5;
--leading-relaxed: 1.75;
```

### 3.3 Spacing System

```css
/* Based on 4px scale */
--space-1: 0.25rem;  /* 4px */
--space-2: 0.5rem;   /* 8px */
--space-3: 0.75rem;  /* 12px */
--space-4: 1rem;     /* 16px */
--space-5: 1.25rem;  /* 20px */
--space-6: 1.5rem;   /* 24px */
--space-8: 2rem;     /* 32px */
--space-10: 2.5rem;  /* 40px */
--space-12: 3rem;    /* 48px */
--space-16: 4rem;    /* 64px */
```

### 3.4 Border Radius

```css
--radius-sm: 0.25rem;  /* 4px - Small elements */
--radius-md: 0.5rem;   /* 8px - Buttons, inputs */
--radius-lg: 0.75rem;  /* 12px - Cards */
--radius-xl: 1rem;     /* 16px - Large cards */
--radius-full: 9999px; /* Full rounded */
```

### 3.5 Shadows

```css
--shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.05);
--shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.1);
--shadow-lg: 0 10px 15px -3px rgb(0 0 0 / 0.1);
--shadow-xl: 0 20px 25px -5px rgb(0 0 0 / 0.1);
```

---

## 4. Top Bar

### 4.1 Struktur

```tsx
<TopBar>
  <LeftSection>
    <Logo /> {/* Icon + "Schließfach-Manager" */}
  </LeftSection>
  
  <CenterSection>
    <Breadcrumb>
      <BreadcrumbItem>Dashboard</BreadcrumbItem>
      <BreadcrumbSeparator />
      <BreadcrumbItem>Übersicht</BreadcrumbItem>
    </Breadcrumb>
  </CenterSection>
  
  <RightSection>
    <QuickActions>
      <SearchButton />
      <NotificationButton />
      <SettingsButton />
    </QuickActions>
  </RightSection>
</TopBar>
```

### 4.2 Design Specs

**Height:** 64px  
**Background:** `var(--color-bg-surface)`  
**Border Bottom:** 1px solid `var(--color-border)`  
**Padding:** 0 24px

```css
.top-bar {
  height: 64px;
  background: var(--color-bg-surface);
  border-bottom: 1px solid var(--color-border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  z-index: 50;
}

.logo {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: var(--text-xl);
  font-weight: var(--font-semibold);
  color: var(--color-text-primary);
}

.breadcrumb {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}

.breadcrumb-item:last-child {
  color: var(--color-text-primary);
  font-weight: var(--font-medium);
}
```

---

## 5. Sidebar Navigation

### 5.1 Struktur (entspricht TUI "Windows")

```tsx
<Sidebar>
  <SidebarHeader>
    {/* Optional: User info or app logo */}
  </SidebarHeader>
  
  <SidebarNav>
    <SidebarCategory title="Hauptmenü">
      <SidebarItem 
        icon={<HomeIcon />} 
        label="Dashboard" 
        active={true}
      />
    </SidebarCategory>
    
    <SidebarCategory title="Verleih">
      <SidebarItem icon={<PlusIcon />} label="Neu verleihen" />
      <SidebarItem icon={<ListIcon />} label="Aktive Verleihungen" />
      <SidebarItem icon={<ClockIcon />} label="Verlängerungen" />
      <SidebarItem icon={<CheckIcon />} label="Rückgaben" />
      <SidebarItem icon={<AlertIcon />} label="Defekte melden" />
    </SidebarCategory>
    
    <SidebarCategory title="Finanzen">
      <SidebarItem icon={<DollarIcon />} label="Übersicht" />
      <SidebarItem icon={<ChartIcon />} label="Statistiken" />
      <SidebarItem icon={<FileIcon />} label="Berichte" />
    </SidebarCategory>
    
    <SidebarCategory title="Verwaltung">
      <SidebarItem icon={<LockIcon />} label="Schließfächer" />
      <SidebarItem icon={<MapIcon />} label="Standorte" />
      <SidebarItem icon={<SettingsIcon />} label="Einstellungen" />
      <SidebarItem icon={<DownloadIcon />} label="Export/Import" />
      <SidebarItem icon={<HistoryIcon />} label="Audit Log" />
    </SidebarCategory>
  </SidebarNav>
  
  <SidebarFooter>
    <DBStatus />
    <VersionInfo />
  </SidebarFooter>
</Sidebar>
```

### 5.2 Design Specs

**Width:** 240px (fixed)  
**Background:** `var(--sidebar-bg)`  
**Border Right:** 1px solid `var(--color-border)`

```css
.sidebar {
  width: 240px;
  height: calc(100vh - 64px - 32px); /* Full height minus top/bottom bars */
  background: var(--sidebar-bg);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  overflow-y: auto;
}

.sidebar-category {
  margin-top: 24px;
  padding: 0 12px;
}

.sidebar-category-title {
  font-size: var(--text-xs);
  font-weight: var(--font-semibold);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--color-text-muted);
  padding: 8px 12px;
  margin-bottom: 4px;
}

.sidebar-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  border-radius: var(--radius-md);
  color: var(--sidebar-item-text);
  font-size: var(--text-sm);
  cursor: pointer;
  transition: all 0.15s ease;
}

.sidebar-item:hover {
  background: var(--sidebar-item-hover);
}

.sidebar-item.active {
  background: var(--sidebar-item-active);
  color: var(--sidebar-item-text-active);
  font-weight: var(--font-medium);
}

.sidebar-item-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}
```

### 5.3 Navigation Mapping (TUI → GUI)

| TUI Screen | GUI Sidebar Category | GUI Pages |
|------------|---------------------|-----------|
| Dashboard | Hauptmenü | Dashboard |
| Rental Management (Tab 1) | Verleih | Neu verleihen |
| Rental Management (Tab 2) | Verleih | Aktive Verleihungen |
| Rental Management (Tab 3) | Verleih | Verlängerungen |
| Rental Management (Tab 4) | Verleih | Rückgaben |
| Rental Management (Tab 5) | Verleih | Defekte melden |
| Finances | Finanzen | Übersicht, Statistiken, Berichte |
| Management (Lockers) | Verwaltung | Schließfächer |
| Management (Locations) | Verwaltung | Standorte |
| Management (Settings) | Verwaltung | Einstellungen |
| Management (Export) | Verwaltung | Export/Import |
| Management (Audit) | Verwaltung | Audit Log |

---

## 6. Main Content Area

### 6.1 Layout Patterns

#### 6.1.1 Dashboard Layout

```tsx
<MainContent>
  <PageHeader>
    <Title>Dashboard</Title>
    <Actions>
      <Button variant="secondary" icon={<RefreshIcon />}>
        Aktualisieren
      </Button>
    </Actions>
  </PageHeader>
  
  <StatsGrid>
    <StatCard 
      title="Gesamt Schließfächer"
      value="150"
      icon={<LockIcon />}
      color="primary"
    />
    <StatCard 
      title="Belegt"
      value="87 (58%)"
      icon={<CheckIcon />}
      color="success"
      trend="+5%"
    />
    <StatCard 
      title="Frei"
      value="63"
      icon={<BoxIcon />}
      color="info"
    />
    <StatCard 
      title="Defekt"
      value="3"
      icon={<AlertIcon />}
      color="error"
    />
  </StatsGrid>
  
  <ContentGrid>
    <Card title="Belegungstrend" span={2}>
      <OccupancyChart data={...} />
    </Card>
    
    <Card title="Alarme">
      <AlertList>
        <AlertItem severity="error">
          5 überfällige Rückgaben
        </AlertItem>
        <AlertItem severity="warning">
          12 Verleihungen laufen bald ab
        </AlertItem>
      </AlertList>
    </Card>
    
    <Card title="Statistiken nach Größe">
      <SizeBreakdown data={...} />
    </Card>
    
    <Card title="Statistiken nach Standort">
      <LocationBreakdown data={...} />
    </Card>
  </ContentGrid>
</MainContent>
```

#### 6.1.2 List View Layout

```tsx
<MainContent>
  <PageHeader>
    <Title>Aktive Verleihungen</Title>
    <Actions>
      <SearchInput placeholder="Suchen..." />
      <FilterButton />
      <Button variant="primary" icon={<PlusIcon />}>
        Neu verleihen
      </Button>
    </Actions>
  </PageHeader>
  
  <TableContainer>
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead sortable>Schließfach-Nr.</TableHead>
          <TableHead sortable>Mieter</TableHead>
          <TableHead sortable>Standort</TableHead>
          <TableHead sortable>Start</TableHead>
          <TableHead sortable>Ende</TableHead>
          <TableHead>Pfand</TableHead>
          <TableHead>Aktionen</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {rentals.map(rental => (
          <TableRow key={rental.id}>
            <TableCell>{rental.lockerNumber}</TableCell>
            <TableCell>{rental.renterName}</TableCell>
            <TableCell>{rental.location}</TableCell>
            <TableCell>{formatDate(rental.startDate)}</TableCell>
            <TableCell>{formatDate(rental.endDate)}</TableCell>
            <TableCell>
              <Badge variant={rental.depositPaid ? 'success' : 'error'}>
                {rental.depositPaid ? 'Bezahlt' : 'Ausstehend'}
              </Badge>
            </TableCell>
            <TableCell>
              <DropdownMenu>
                <MenuItem icon={<EyeIcon />}>Details</MenuItem>
                <MenuItem icon={<ClockIcon />}>Verlängern</MenuItem>
                <MenuItem icon={<CheckIcon />}>Zurückgeben</MenuItem>
                <MenuItem icon={<TrashIcon />} danger>Löschen</MenuItem>
              </DropdownMenu>
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  </TableContainer>
  
  <Pagination 
    currentPage={1}
    totalPages={5}
    onPageChange={...}
  />
</MainContent>
```

#### 6.1.3 Form/Wizard Layout (ersetzt TUI Dialog-Wizards)

```tsx
<MainContent>
  <WizardContainer>
    <WizardHeader>
      <Title>Neuer Verleih</Title>
      <WizardSteps currentStep={2} totalSteps={4}>
        <Step completed>Schließfach wählen</Step>
        <Step active>Mieter-Daten</Step>
        <Step>Konditionen</Step>
        <Step>Bestätigung</Step>
      </WizardSteps>
    </WizardHeader>
    
    <WizardContent>
      <FormCard>
        <FormSection title="Persönliche Daten">
          <FormField label="Name *" required>
            <Input 
              placeholder="Max Mustermann"
              error={errors.name}
            />
          </FormField>
          
          <FormField label="E-Mail">
            <Input 
              type="email"
              placeholder="max@example.com"
            />
          </FormField>
          
          <FormField label="Telefon">
            <Input 
              type="tel"
              placeholder="+49 123 456789"
            />
          </FormField>
        </FormSection>
        
        <FormSection title="Notizen">
          <FormField>
            <Textarea 
              rows={4}
              placeholder="Optional: Zusätzliche Informationen..."
            />
          </FormField>
        </FormSection>
      </FormCard>
    </WizardContent>
    
    <WizardFooter>
      <Button variant="secondary" onClick={handleBack}>
        Zurück
      </Button>
      <Button variant="primary" onClick={handleNext}>
        Weiter
      </Button>
    </WizardFooter>
  </WizardContainer>
</MainContent>
```

### 6.2 Content Padding & Spacing

```css
.main-content {
  padding: 24px;
  overflow-y: auto;
  height: calc(100vh - 64px - 32px);
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.content-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 24px;
  margin-top: 24px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
  margin-bottom: 24px;
}
```

---

## 7. Component Library

### 7.1 Buttons

```tsx
// Primary Button
<Button variant="primary" size="md" icon={<PlusIcon />}>
  Hinzufügen
</Button>

// Secondary Button
<Button variant="secondary" size="md">
  Abbrechen
</Button>

// Danger Button
<Button variant="danger" size="md" icon={<TrashIcon />}>
  Löschen
</Button>

// Icon Only Button
<IconButton icon={<SettingsIcon />} />
```

**Variants:**
- `primary`: Blue, for main actions
- `secondary`: Gray, for secondary actions
- `danger`: Red, for destructive actions
- `ghost`: Transparent, for subtle actions

**Sizes:**
- `sm`: 32px height, 12px padding
- `md`: 40px height, 16px padding (default)
- `lg`: 48px height, 20px padding

```css
.button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  border-radius: var(--radius-md);
  font-weight: var(--font-medium);
  transition: all 0.15s ease;
  cursor: pointer;
  border: none;
}

.button-primary {
  background: var(--btn-primary-bg);
  color: var(--btn-primary-text);
}

.button-primary:hover {
  background: var(--btn-primary-hover);
}

.button-primary:active {
  background: var(--btn-primary-active);
}

.button-md {
  height: 40px;
  padding: 0 16px;
  font-size: var(--text-sm);
}
```

### 7.2 Cards

```tsx
<Card>
  <CardHeader>
    <CardTitle>Titel</CardTitle>
    <CardActions>
      <IconButton icon={<MoreIcon />} />
    </CardActions>
  </CardHeader>
  <CardContent>
    {/* Content */}
  </CardContent>
  <CardFooter>
    {/* Optional footer */}
  </CardFooter>
</Card>
```

```css
.card {
  background: var(--color-bg-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.card-header {
  padding: 16px 20px;
  border-bottom: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.card-title {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--color-text-primary);
}

.card-content {
  padding: 20px;
}
```

### 7.3 Form Elements

```tsx
// Text Input
<Input 
  label="Name"
  placeholder="Eingabe..."
  error="Dieses Feld ist erforderlich"
  helperText="Optional: Hilfetext"
/>

// Select
<Select 
  label="Größe"
  options={[
    { value: 'S', label: 'Klein' },
    { value: 'M', label: 'Mittel' },
    { value: 'L', label: 'Groß' },
  ]}
/>

// Checkbox
<Checkbox label="Pfand bezahlt" />

// Radio Group
<RadioGroup label="Standort">
  <Radio value="building-a" label="Gebäude A" />
  <Radio value="building-b" label="Gebäude B" />
</RadioGroup>

// Date Picker
<DatePicker 
  label="Enddatum"
  value={endDate}
  onChange={setEndDate}
/>
```

```css
.form-field {
  margin-bottom: 16px;
}

.form-label {
  display: block;
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  color: var(--color-text-primary);
  margin-bottom: 6px;
}

.input {
  width: 100%;
  height: 40px;
  padding: 0 12px;
  background: var(--color-bg-base);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  color: var(--color-text-primary);
  font-size: var(--text-sm);
  transition: all 0.15s ease;
}

.input:focus {
  outline: none;
  border-color: var(--color-border-focus);
  box-shadow: 0 0 0 3px rgba(137, 180, 250, 0.1);
}

.input-error {
  border-color: var(--color-error);
}

.input-helper-text {
  font-size: var(--text-xs);
  color: var(--color-text-muted);
  margin-top: 4px;
}

.input-error-text {
  font-size: var(--text-xs);
  color: var(--color-error);
  margin-top: 4px;
}
```

### 7.4 Table

```tsx
<Table>
  <TableHeader>
    <TableRow>
      <TableHead sortable onSort={...}>Name</TableHead>
      <TableHead>Status</TableHead>
      <TableHead align="right">Aktionen</TableHead>
    </TableRow>
  </TableHeader>
  <TableBody>
    <TableRow hover clickable onClick={...}>
      <TableCell>Beispiel</TableCell>
      <TableCell>
        <Badge variant="success">Aktiv</Badge>
      </TableCell>
      <TableCell align="right">
        <IconButton icon={<EditIcon />} />
      </TableCell>
    </TableRow>
  </TableBody>
</Table>
```

```css
.table {
  width: 100%;
  border-collapse: collapse;
}

.table-header {
  background: var(--color-bg-surface);
  border-bottom: 2px solid var(--color-border);
}

.table-head {
  padding: 12px 16px;
  text-align: left;
  font-size: var(--text-sm);
  font-weight: var(--font-semibold);
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.table-row {
  border-bottom: 1px solid var(--color-border);
}

.table-row:hover {
  background: var(--color-bg-surface);
}

.table-cell {
  padding: 16px;
  font-size: var(--text-sm);
  color: var(--color-text-primary);
}
```

### 7.5 Badges & Status Indicators

```tsx
<Badge variant="success">Bezahlt</Badge>
<Badge variant="warning">Ausstehend</Badge>
<Badge variant="error">Überfällig</Badge>
<Badge variant="info">In Bearbeitung</Badge>
```

```css
.badge {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: var(--font-medium);
}

.badge-success {
  background: rgba(166, 227, 161, 0.15);
  color: var(--color-success);
}

.badge-warning {
  background: rgba(249, 226, 175, 0.15);
  color: var(--color-warning);
}

.badge-error {
  background: rgba(243, 139, 168, 0.15);
  color: var(--color-error);
}

.badge-info {
  background: rgba(137, 220, 235, 0.15);
  color: var(--color-info);
}
```

### 7.6 Modals & Dialogs

```tsx
<Modal open={isOpen} onClose={handleClose}>
  <ModalHeader>
    <ModalTitle>Bestätigung erforderlich</ModalTitle>
    <IconButton icon={<XIcon />} onClick={handleClose} />
  </ModalHeader>
  
  <ModalContent>
    Möchten Sie diesen Verleih wirklich löschen?
    Diese Aktion kann nicht rückgängig gemacht werden.
  </ModalContent>
  
  <ModalFooter>
    <Button variant="secondary" onClick={handleClose}>
      Abbrechen
    </Button>
    <Button variant="danger" onClick={handleConfirm}>
      Löschen
    </Button>
  </ModalFooter>
</Modal>
```

```css
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--color-bg-surface);
  border-radius: var(--radius-xl);
  border: 1px solid var(--color-border);
  box-shadow: var(--shadow-xl);
  max-width: 500px;
  width: 90%;
  max-height: 90vh;
  overflow: hidden;
}

.modal-header {
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-content {
  padding: 24px;
}

.modal-footer {
  padding: 16px 24px;
  border-top: 1px solid var(--color-border);
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
```

### 7.7 Toast Notifications (ersetzt TUI Status Bar Messages)

```tsx
// Success Toast
toast.success('Verleih erfolgreich erstellt');

// Error Toast
toast.error('Fehler beim Speichern');

// Warning Toast
toast.warning('Pfand wurde noch nicht bezahlt');

// Info Toast
toast.info('Datenbank aktualisiert');
```

```css
.toast {
  min-width: 300px;
  padding: 16px;
  background: var(--color-bg-overlay);
  border-left: 4px solid;
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  display: flex;
  align-items: center;
  gap: 12px;
}

.toast-success {
  border-left-color: var(--color-success);
}

.toast-error {
  border-left-color: var(--color-error);
}

.toast-warning {
  border-left-color: var(--color-warning);
}

.toast-info {
  border-left-color: var(--color-info);
}
```

---

## 8. Status Bar (Bottom)

### 8.1 Struktur

```tsx
<StatusBar>
  <LeftSection>
    <StatusMessage level="success">
      Letzter Speichervorgang: Erfolg
    </StatusMessage>
  </LeftSection>
  
  <RightSection>
    <StatusItem icon={<DatabaseIcon />}>
      DB: Verbunden
    </StatusItem>
    <StatusItem>
      Version 2.1.0
    </StatusItem>
    <StatusItem icon={<ClockIcon />}>
      {currentTime}
    </StatusItem>
  </RightSection>
</StatusBar>
```

### 8.2 Design Specs

```css
.status-bar {
  height: 32px;
  background: var(--color-bg-surface);
  border-top: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 16px;
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
}

.status-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.status-separator {
  width: 1px;
  height: 16px;
  background: var(--color-border);
  margin: 0 12px;
}
```

---

## 9. Keyboard Shortcuts

**Obwohl GUI, bleiben Shortcuts unterstützt!**

### 9.1 Globale Shortcuts

| Shortcut | Aktion | Beschreibung |
|----------|--------|--------------|
| `Ctrl+K` | Suche öffnen | Globale Suche |
| `Ctrl+N` | Neu | Kontext-abhängig neues Element |
| `Ctrl+S` | Speichern | Aktuelles Formular speichern |
| `Ctrl+,` | Einstellungen | Einstellungen öffnen |
| `Ctrl+Q` | Beenden | App schließen |
| `F5` | Aktualisieren | Daten neu laden |
| `Esc` | Schließen | Dialog/Modal schließen |

### 9.2 Navigation Shortcuts

| Shortcut | Aktion |
|----------|--------|
| `Ctrl+1` | Dashboard |
| `Ctrl+2` | Verleih |
| `Ctrl+3` | Finanzen |
| `Ctrl+4` | Verwaltung |
| `Alt+←` | Zurück |
| `Alt+→` | Vorwärts |

### 9.3 Tabellen-Shortcuts

| Shortcut | Aktion |
|----------|--------|
| `↑↓` | Zeile wechseln |
| `Enter` | Details öffnen |
| `Space` | Auswählen |
| `Ctrl+A` | Alle auswählen |
| `/` | Suche fokussieren |

**Shortcuts werden als Tooltips angezeigt:**
```tsx
<Button tooltip="Neu verleihen (Ctrl+N)">
  Hinzufügen
</Button>
```

---

## 10. Animationen & Transitions

### 10.1 Micro-Animations

```css
/* Page Transitions */
.page-enter {
  opacity: 0;
  transform: translateY(8px);
}

.page-enter-active {
  opacity: 1;
  transform: translateY(0);
  transition: all 0.2s ease-out;
}

/* Modal Transitions */
.modal-enter {
  opacity: 0;
  transform: scale(0.95);
}

.modal-enter-active {
  opacity: 1;
  transform: scale(1);
  transition: all 0.2s ease-out;
}

/* Hover Effects */
.hover-lift {
  transition: transform 0.15s ease;
}

.hover-lift:hover {
  transform: translateY(-2px);
}
```

### 10.2 Loading States

```tsx
<LoadingSpinner size="lg" />

<Button loading>
  Speichern...
</Button>

<Skeleton height={40} />
```

---

## 11. Responsive Behavior

### 11.1 Breakpoint Adaptations

**< 768px (Tablets):**
- Sidebar wird zu Drawer (über Content)
- Hamburger Menu in Top Bar
- Stats Grid: 2 Spalten
- Reduzierte Paddings

**< 640px (Phones):**
- App nicht unterstützt
- Zeige "Bitte verwenden Sie ein größeres Gerät"

### 11.2 Dark Mode Toggle

```tsx
<SettingsPage>
  <SettingItem>
    <Label>Design</Label>
    <ToggleGroup>
      <Toggle value="dark" active>Dunkel</Toggle>
      <Toggle value="light">Hell</Toggle>
      <Toggle value="auto">System</Toggle>
    </ToggleGroup>
  </SettingItem>
</SettingsPage>
```

Light Mode kann später hinzugefügt werden, aktuell nur Dark Mode.

---

## 12. Implementation Stack

### 12.1 Frontend

```json
{
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "@tanstack/react-query": "^5.0.0",
    "@tanstack/react-table": "^8.10.0",
    "react-hook-form": "^7.48.0",
    "zod": "^3.22.0",
    "@hookform/resolvers": "^3.3.0",
    "date-fns": "^3.0.0",
    "recharts": "^2.10.0",
    "lucide-react": "^0.300.0",
    "sonner": "^1.3.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0",
    "tailwindcss": "^3.4.0",
    "autoprefixer": "^10.4.0",
    "postcss": "^8.4.0"
  }
}
```

### 12.2 Backend (Tauri)

```toml
[dependencies]
tauri = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rusqlite = { version = "0.32", features = ["bundled"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
```

### 12.3 File Structure

```
schliessfach-manager-tauri/
├── src-tauri/              # Rust Backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands.rs     # Tauri commands
│   │   ├── db/
│   │   │   ├── mod.rs
│   │   │   ├── connection.rs
│   │   │   ├── migrations.rs
│   │   │   ├── lockers.rs
│   │   │   ├── rentals.rs
│   │   │   └── ...
│   │   └── ...
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                    # React Frontend
│   ├── components/
│   │   ├── ui/             # Base components (Button, Input, etc.)
│   │   ├── layout/         # Layout components (Sidebar, TopBar, etc.)
│   │   └── features/       # Feature-specific components
│   ├── pages/
│   │   ├── Dashboard.tsx
│   │   ├── RentalNew.tsx
│   │   ├── RentalList.tsx
│   │   └── ...
│   ├── hooks/              # Custom React hooks
│   ├── lib/
│   │   ├── tauri.ts        # Tauri API wrappers
│   │   └── utils.ts
│   ├── styles/
│   │   └── globals.css
│   ├── App.tsx
│   └── main.tsx
│
├── package.json
├── tsconfig.json
├── tailwind.config.js
├── vite.config.ts
└── README.md
```

---

## 13. Zusammenfassung: TUI → GUI Mapping

| Aspekt | TUI (Ratatui) | GUI (Tauri) |
|--------|---------------|-------------|
| **Navigation** | Window Switcher (^) + Tabs | Sidebar + Pages |
| **Screens** | Fullscreen views | Main content area |
| **Keybinds** | Keybind Bar (visible) | Shortcuts (tooltips) |
| **Status** | Status Bar mit Escape Counter | Bottom bar + Toast notifications |
| **Forms** | Dialog-style Wizards (chat) | Multi-step forms/wizards |
| **Tables** | Terminal tables | Rich HTML tables |
| **Actions** | Keyboard only | Mouse + Keyboard |
| **Feedback** | Status messages | Toasts + Modals |
| **Layout** | Fixed character grid | Flexible CSS Grid/Flexbox |

---

## 14. Nächste Schritte

1. **Tauri Projekt initialisieren**
   ```bash
   npm create tauri-app@latest
   # Choose: React + TypeScript
   ```

2. **Tailwind CSS Setup**
   ```bash
   npm install -D tailwindcss postcss autoprefixer
   npx tailwindcss init -p
   ```

3. **Basis-Components erstellen**
   - Button, Input, Card, Table
   - Sidebar, TopBar, StatusBar
   - Layout-Wrapper

4. **Rust Backend migrieren**
   - DB-Schema von v2.1 übernehmen
   - Tauri Commands definieren
   - CRUD-Operationen implementieren

5. **Pages schrittweise aufbauen**
   - Dashboard
   - Rental List
   - Rental New (Wizard)
   - usw.

**Ziel:** Moderne, performante Desktop-App mit identischen Features wie TUI-Version! 🚀