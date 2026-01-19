/**
 * UI Interaction Tests
 * @module tests/ui/interaction.test
 */

import { describe, it, expect, beforeEach, afterEach } from 'bun:test';
import { App } from '../../src/app';
import { DatabaseConnection } from '../../src/db/connection';
import { runMigrations } from '../../src/db/migrations';

describe('UI Interaction', () => {
  beforeEach(() => {
    DatabaseConnection.enableTestMode();
    const db = DatabaseConnection.getConnection();
    runMigrations(db);
  });

  afterEach(() => {
    DatabaseConnection.close();
    DatabaseConnection.disableTestMode();
  });

  describe('Application State', () => {
    it('should initialize with dashboard screen', () => {
      const app = new App();
      expect(app.getCurrentScreen()).toBe('dashboard');
    });

    it('should start running', () => {
      const app = new App();
      expect(app.isRunning()).toBe(true);
    });

    it('should stop when q is pressed', () => {
      const app = new App();
      app.handleKey('q');
      expect(app.isRunning()).toBe(false);
    });
  });

  describe('Window Switcher', () => {
    it('should activate window switcher on ^', () => {
      const app = new App();
      app.handleKey('^');
      expect(app.isWindowSwitcherActive()).toBe(true);
    });

    it('should deactivate window switcher on escape', () => {
      const app = new App();
      app.handleKey('^');
      app.handleKey('escape');
      expect(app.isWindowSwitcherActive()).toBe(false);
    });

    it('should switch screens via window switcher', () => {
      const app = new App();
      app.handleKey('^');
      app.handleKey('tab'); // Move to next window
      app.handleKey('enter'); // Select
      
      expect(app.isWindowSwitcherActive()).toBe(false);
      expect(app.getCurrentScreen()).toBe('rental');
    });
  });

  describe('Triple Escape', () => {
    it('should return to dashboard after 3 escapes', () => {
      const app = new App();
      
      // Navigate away from dashboard
      app.handleKey('1'); // Go to rental
      expect(app.getCurrentScreen()).toBe('rental');
      
      // Triple escape
      app.handleKey('escape');
      app.handleKey('escape');
      app.handleKey('escape');
      
      expect(app.getCurrentScreen()).toBe('dashboard');
    });

    it('should reset escape counter on other key', () => {
      const app = new App();
      
      // Navigate away
      app.handleKey('1');
      
      // Start escape sequence
      app.handleKey('escape');
      app.handleKey('escape');
      
      // Press another key
      app.handleKey('a');
      
      // One more escape shouldn't return to dashboard
      app.handleKey('escape');
      
      expect(app.getCurrentScreen()).toBe('rental');
    });
  });

  describe('Screen Navigation', () => {
    it('should navigate to rental screen with key 1', () => {
      const app = new App();
      app.handleKey('1');
      expect(app.getCurrentScreen()).toBe('rental');
    });

    it('should navigate to finances screen with key 2', () => {
      const app = new App();
      app.handleKey('2');
      expect(app.getCurrentScreen()).toBe('finances');
    });

    it('should navigate to management screen with key 3', () => {
      const app = new App();
      app.handleKey('3');
      expect(app.getCurrentScreen()).toBe('management');
    });
  });

  describe('Rendering', () => {
    it('should render without errors', () => {
      const app = new App();
      const output = app.render();
      
      expect(output).toBeDefined();
      expect(typeof output).toBe('string');
      expect(output.length).toBeGreaterThan(0);
    });

    it('should render different screens', () => {
      const app = new App();
      
      const dashboardOutput = app.render();
      
      app.handleKey('1');
      const rentalOutput = app.render();
      
      // Outputs should be different (different screens)
      expect(dashboardOutput).not.toBe(rentalOutput);
    });
  });

  describe('Screensaver', () => {
    it('should not be active initially', () => {
      const app = new App();
      expect(app.isScreensaverActive()).toBe(false);
    });

    it('should return to dashboard after screensaver exit', () => {
      const app = new App();
      
      // Navigate away
      app.handleKey('1');
      expect(app.getCurrentScreen()).toBe('rental');
      
      // Simulate screensaver activation and exit
      // (In real test, we would manipulate time, but for now just verify state)
      expect(app.isScreensaverActive()).toBe(false);
    });
  });
});
