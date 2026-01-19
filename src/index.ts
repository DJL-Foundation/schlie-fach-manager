/**
 * Schließfach-Manager v2.1 - TypeScript Edition
 * Main Entry Point
 * @module index
 */

import { App } from './app';
import { DatabaseConnection } from './db/connection';
import {
  clearScreen,
  hideCursor,
  showCursor,
  enterAlternateScreen,
  exitAlternateScreen,
  moveCursor,
} from './ui';

/**
 * Parse key input from raw buffer
 * @param data Raw input data
 * @returns Parsed key name
 */
function parseKeyInput(data: Buffer): string {
  const str = data.toString();

  // Handle escape sequences
  if (str === '\x1b' || str === '\u001b') {
    return 'escape';
  }

  // Arrow keys
  if (str === '\x1b[A') return 'up';
  if (str === '\x1b[B') return 'down';
  if (str === '\x1b[C') return 'right';
  if (str === '\x1b[D') return 'left';

  // Tab
  if (str === '\t') return 'tab';
  if (str === '\x1b[Z') return 'shift+tab';

  // Enter
  if (str === '\r' || str === '\n') return 'enter';

  // Backspace
  if (str === '\x7f' || str === '\b') return 'backspace';

  // Ctrl+C
  if (str === '\x03') return 'ctrl+c';

  // Single character
  if (str.length === 1) {
    return str;
  }

  return str;
}

/**
 * Sleep for specified milliseconds
 * @param ms Milliseconds to sleep
 */
function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Main application entry point
 */
async function main(): Promise<void> {
  const app = new App();

  // Setup terminal
  process.stdout.write(enterAlternateScreen());
  process.stdout.write(hideCursor());
  process.stdout.write(clearScreen());

  // Setup raw mode for keyboard input
  if (process.stdin.isTTY) {
    process.stdin.setRawMode(true);
  }
  process.stdin.resume();

  // Handle keyboard input
  process.stdin.on('data', (data: Buffer) => {
    const key = parseKeyInput(data);

    // Handle Ctrl+C
    if (key === 'ctrl+c') {
      cleanup();
      process.exit(0);
    }

    app.handleKey(key);
  });

  // Handle terminal resize
  process.stdout.on('resize', () => {
    // Re-render on resize
  });

  // Cleanup function
  function cleanup(): void {
    // Restore terminal
    process.stdout.write(showCursor());
    process.stdout.write(exitAlternateScreen());
    process.stdout.write(clearScreen());

    // Close database
    DatabaseConnection.close();

    // Restore stdin
    if (process.stdin.isTTY) {
      process.stdin.setRawMode(false);
    }
  }

  // Handle exit signals
  process.on('SIGINT', () => {
    cleanup();
    process.exit(0);
  });

  process.on('SIGTERM', () => {
    cleanup();
    process.exit(0);
  });

  process.on('exit', () => {
    cleanup();
  });

  // Main loop
  const targetFPS = 30;
  const frameTime = 1000 / targetFPS;

  while (app.isRunning()) {
    const startTime = Date.now();

    // Update
    app.update();

    // Render
    const output = app.render();
    process.stdout.write(moveCursor(0, 0));
    process.stdout.write(output);

    // Frame limiting
    const elapsed = Date.now() - startTime;
    const remaining = frameTime - elapsed;

    if (remaining > 0) {
      await sleep(remaining);
    }
  }

  // Cleanup
  cleanup();
  process.exit(0);
}

// Run main
main().catch((error) => {
  // eslint-disable-next-line no-console
  console.error('Fatal error:', error);
  process.exit(1);
});
