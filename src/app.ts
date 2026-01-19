/**
 * Application State Management
 * @module app
 */

import type { AppScreen, InactivityState, StatusMessage, WindowInfo } from './types/app';
import { DatabaseConnection } from './db/connection';
import { runMigrations } from './db/migrations';
import { Header, createDefaultHeaderProps } from './ui/widgets/header';
import { KeybindBar, DEFAULT_GLOBAL_KEYBINDS, DASHBOARD_KEYBINDS, RENTAL_KEYBINDS, FINANCES_KEYBINDS, MANAGEMENT_KEYBINDS } from './ui/widgets/keybind-bar';
import { StatusBar } from './ui/widgets/status-bar';
import { DashboardScreen } from './ui/screens/dashboard';
import { ScreensaverScreen } from './ui/screens/screensaver';
import { RentalManagementScreen } from './ui/screens/rental-management';
import { FinancesScreen } from './ui/screens/finances';
import { ManagementScreen } from './ui/screens/management';
import { calculateLayout, getTerminalSize, clearScreen, hideCursor, showCursor, enterAlternateScreen, exitAlternateScreen, moveCursor } from './ui';
import type { Keybind } from './types/ui';

/**
 * Window list for window switcher
 */
const WINDOW_LIST: WindowInfo[] = [
  { name: 'Dashboard', screen: 'dashboard' },
  { name: 'Verleih', screen: 'rental' },
  { name: 'Finanzen', screen: 'finances' },
  { name: 'Verwaltung', screen: 'management' },
];

/**
 * Main application class
 */
export class App {
  // Current screen
  private currentScreen: AppScreen = 'dashboard';

  // UI Components
  private header: Header;
  private keybindBar: KeybindBar;
  private statusBar: StatusBar;

  // Screens
  private dashboardScreen: DashboardScreen;
  private screensaverScreen: ScreensaverScreen;
  private rentalScreen: RentalManagementScreen;
  private financesScreen: FinancesScreen;
  private managementScreen: ManagementScreen;

  // Window switcher state
  private windowSwitcherActive = false;
  private selectedWindowIndex = 0;

  // Screensaver state
  private lastActivity = Date.now();
  private screensaverTimeout = 60000; // 60 seconds
  private countdownDuration = 15000; // 15 seconds
  private screensaverActive = false;

  // Application state
  private running = true;

  constructor() {
    // Initialize database
    const db = DatabaseConnection.getConnection();
    runMigrations(db);

    // Load screensaver timeout from settings
    try {
      const timeoutSetting = db
        .prepare("SELECT value FROM settings WHERE key = 'screensaver_timeout_seconds'")
        .get() as { value: string } | undefined;

      if (timeoutSetting) {
        this.screensaverTimeout = parseInt(timeoutSetting.value, 10) * 1000;
      }
    } catch {
      // Use default timeout
    }

    // Initialize UI components
    this.header = new Header(
      createDefaultHeaderProps('Dashboard', WINDOW_LIST.map((w) => ({ name: w.name, screen: w.screen })))
    );

    this.keybindBar = new KeybindBar(DEFAULT_GLOBAL_KEYBINDS, DASHBOARD_KEYBINDS, 'Dashboard');
    this.statusBar = new StatusBar();

    // Initialize screens
    this.dashboardScreen = new DashboardScreen();
    this.screensaverScreen = new ScreensaverScreen();
    this.rentalScreen = new RentalManagementScreen();
    this.financesScreen = new FinancesScreen();
    this.managementScreen = new ManagementScreen();

    // Initial status message
    this.statusBar.setMessage('Willkommen zum Schließfach-Manager v2.1', 'info');
  }

  /**
   * Render the application
   * @returns Rendered output string
   */
  render(): string {
    const { columns: width, rows: height } = getTerminalSize();

    if (this.screensaverActive) {
      this.screensaverScreen.update();
      const lines = this.screensaverScreen.render(width, height);
      return lines.join('\n');
    }

    // Calculate layout
    const layout = calculateLayout(width, height);

    // Update header
    this.header.update({
      currentScreen: this.getScreenName(this.currentScreen),
      windowSwitcherActive: this.windowSwitcherActive,
      selectedWindowIndex: this.selectedWindowIndex,
    });

    // Render all components
    const output: string[] = [];

    // Header
    const headerLines = this.header.render(layout.header.width);
    output.push(...headerLines);

    // Content
    const contentLines = this.renderCurrentScreen(layout.content.width, layout.content.height);
    output.push(...contentLines);

    // Keybind bar
    const keybindLines = this.keybindBar.render(layout.keybindBar.width);
    output.push(...keybindLines);

    // Status bar
    const statusLine = this.statusBar.render(layout.statusBar.width);
    output.push(statusLine);

    return output.join('\n');
  }

  /**
   * Render current screen content
   */
  private renderCurrentScreen(width: number, height: number): string[] {
    switch (this.currentScreen) {
      case 'dashboard':
        return this.dashboardScreen.render(width, height);
      case 'rental':
        return this.rentalScreen.render(width, height);
      case 'finances':
        return this.financesScreen.render(width, height);
      case 'management':
        return this.managementScreen.render(width, height);
      default:
        return this.dashboardScreen.render(width, height);
    }
  }

  /**
   * Handle keyboard input
   * @param key Pressed key
   */
  handleKey(key: string): void {
    this.updateActivity();

    // Handle screensaver exit
    if (this.screensaverActive) {
      this.exitScreensaver();
      return;
    }

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
        this.updateKeybinds();
      }

      return;
    }

    // Reset escape count on other keys
    this.statusBar.resetEscapeCount();

    // Global keybinds
    switch (key) {
      case '^':
        this.activateWindowSwitcher();
        return;
      case 'q':
        this.running = false;
        return;
    }

    // Screen-specific keybinds
    this.handleScreenKey(key);
  }

  /**
   * Handle window switcher keys
   */
  private handleWindowSwitcherKey(key: string): void {
    switch (key) {
      case 'tab':
        this.selectedWindowIndex = (this.selectedWindowIndex + 1) % WINDOW_LIST.length;
        break;
      case 'shift+tab':
        this.selectedWindowIndex = (this.selectedWindowIndex - 1 + WINDOW_LIST.length) % WINDOW_LIST.length;
        break;
      case 'enter':
        this.switchToSelectedWindow();
        break;
      case 'escape':
        this.deactivateWindowSwitcher();
        break;
    }
  }

  /**
   * Handle screen-specific keys
   */
  private handleScreenKey(key: string): void {
    switch (this.currentScreen) {
      case 'dashboard':
        this.handleDashboardKey(key);
        break;
      case 'rental':
        this.handleRentalKey(key);
        break;
      case 'finances':
        this.handleFinancesKey(key);
        break;
      case 'management':
        this.handleManagementKey(key);
        break;
    }
  }

  /**
   * Handle dashboard keys
   */
  private handleDashboardKey(key: string): void {
    switch (key) {
      case '1':
        this.currentScreen = 'rental';
        this.updateKeybinds();
        this.statusBar.setMessage('Verleihverwaltung', 'info');
        break;
      case '2':
        this.currentScreen = 'finances';
        this.updateKeybinds();
        this.statusBar.setMessage('Finanzen', 'info');
        break;
      case '3':
        this.currentScreen = 'management';
        this.updateKeybinds();
        this.statusBar.setMessage('Verwaltung', 'info');
        break;
      case 'r':
        this.dashboardScreen.refresh();
        this.statusBar.setMessage('Dashboard aktualisiert', 'success');
        break;
    }
  }

  /**
   * Handle rental screen keys
   */
  private handleRentalKey(key: string): void {
    switch (key) {
      case 'up':
      case 'k':
        this.rentalScreen.selectPrevious();
        break;
      case 'down':
      case 'j':
        this.rentalScreen.selectNext(100);
        break;
      case '1':
        this.rentalScreen.setTab('active');
        break;
      case '2':
        this.rentalScreen.setTab('overdue');
        break;
      case '3':
        this.rentalScreen.setTab('expiring');
        break;
    }
  }

  /**
   * Handle finances screen keys
   */
  private handleFinancesKey(key: string): void {
    // Finances screen key handling
  }

  /**
   * Handle management screen keys
   */
  private handleManagementKey(key: string): void {
    switch (key) {
      case 'l':
        this.managementScreen.setOption('locations');
        break;
      case 's':
        this.managementScreen.setOption('settings');
        break;
      case 'a':
        this.managementScreen.setOption('audit');
        break;
      case 'i':
        this.managementScreen.setOption('import');
        break;
      case 'x':
        this.managementScreen.setOption('export');
        break;
    }
  }

  /**
   * Update application state
   */
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

  /**
   * Check if application is running
   */
  isRunning(): boolean {
    return this.running;
  }

  /**
   * Stop the application
   */
  stop(): void {
    this.running = false;
  }

  /**
   * Get human-readable screen name
   */
  private getScreenName(screen: AppScreen): string {
    const names: Record<AppScreen, string> = {
      dashboard: 'Dashboard',
      rental: 'Verleih',
      finances: 'Finanzen',
      management: 'Verwaltung',
      screensaver: 'Screensaver',
    };
    return names[screen] || screen;
  }

  /**
   * Update keybind bar based on current screen
   */
  private updateKeybinds(): void {
    let contextBinds: Keybind[] = [];

    switch (this.currentScreen) {
      case 'dashboard':
        contextBinds = DASHBOARD_KEYBINDS;
        break;
      case 'rental':
        contextBinds = RENTAL_KEYBINDS;
        break;
      case 'finances':
        contextBinds = FINANCES_KEYBINDS;
        break;
      case 'management':
        contextBinds = MANAGEMENT_KEYBINDS;
        break;
    }

    this.keybindBar.updateContext(contextBinds, this.getScreenName(this.currentScreen));
  }

  /**
   * Activate window switcher
   */
  private activateWindowSwitcher(): void {
    this.windowSwitcherActive = true;
    this.selectedWindowIndex = WINDOW_LIST.findIndex((w) => w.screen === this.currentScreen);
    if (this.selectedWindowIndex === -1) {
      this.selectedWindowIndex = 0;
    }
  }

  /**
   * Deactivate window switcher
   */
  private deactivateWindowSwitcher(): void {
    this.windowSwitcherActive = false;
  }

  /**
   * Switch to selected window
   */
  private switchToSelectedWindow(): void {
    const window = WINDOW_LIST[this.selectedWindowIndex];
    this.currentScreen = window.screen as AppScreen;
    this.deactivateWindowSwitcher();
    this.updateKeybinds();
    this.statusBar.setMessage(`Wechsel zu ${window.name}`, 'info');
  }

  /**
   * Update activity timestamp
   */
  private updateActivity(): void {
    this.lastActivity = Date.now();

    if (this.screensaverActive) {
      this.exitScreensaver();
    }
  }

  /**
   * Check inactivity state
   */
  private checkInactivity(): InactivityState {
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

  /**
   * Activate screensaver
   */
  private activateScreensaver(): void {
    this.screensaverActive = true;
    this.screensaverScreen.reset();
  }

  /**
   * Exit screensaver
   */
  private exitScreensaver(): void {
    this.screensaverActive = false;
    this.currentScreen = 'dashboard';
    this.updateKeybinds();
    this.statusBar.setMessage('Willkommen zurück!', 'info');
    this.lastActivity = Date.now();
  }

  /**
   * Get current screen
   */
  getCurrentScreen(): AppScreen {
    return this.currentScreen;
  }

  /**
   * Check if screensaver is active
   */
  isScreensaverActive(): boolean {
    return this.screensaverActive;
  }

  /**
   * Check if window switcher is active
   */
  isWindowSwitcherActive(): boolean {
    return this.windowSwitcherActive;
  }
}
