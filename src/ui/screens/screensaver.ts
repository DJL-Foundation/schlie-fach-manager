/**
 * Screensaver Screen
 * @module ui/screens/screensaver
 */

import { AnimationPlayer, getRandomAnimation } from '../../screensaver/animations';
import { Theme, colorize, ANSI } from '../theme';

/**
 * Screensaver screen component
 */
export class ScreensaverScreen {
  private player: AnimationPlayer;
  private theme: Theme;

  constructor() {
    const animation = getRandomAnimation();
    this.player = new AnimationPlayer(animation);
    this.theme = Theme.defaultDark();
  }

  /**
   * Update animation frame
   */
  update(): void {
    this.player.update();
  }

  /**
   * Reset with a new random animation
   */
  reset(): void {
    const animation = getRandomAnimation();
    this.player = new AnimationPlayer(animation);
  }

  /**
   * Render the screensaver to string array
   * @param width Terminal width
   * @param height Terminal height
   * @returns Rendered lines (fullscreen)
   */
  render(width: number, height: number): string[] {
    const lines: string[] = [];
    const animation = this.player.getAnimation();
    const frame = this.player.getCurrentFrame();

    // Calculate center position
    const frameLines = frame.split('\n');
    const startY = Math.floor((height - frameLines.length) / 2);
    const startX = Math.floor((width - animation.width) / 2);

    // Fill with background
    for (let y = 0; y < height; y++) {
      if (y >= startY && y < startY + frameLines.length) {
        const frameLine = frameLines[y - startY] || '';
        const leftPad = ' '.repeat(Math.max(0, startX));
        const rightPad = ' '.repeat(Math.max(0, width - startX - frameLine.length));
        lines.push(
          colorize(leftPad, this.theme.get('screensaverBg'), this.theme.get('screensaverBg')) +
          colorize(frameLine, this.theme.get('screensaverFg'), this.theme.get('screensaverBg')) +
          colorize(rightPad, this.theme.get('screensaverBg'), this.theme.get('screensaverBg'))
        );
      } else {
        lines.push(' '.repeat(width));
      }
    }

    return lines;
  }

  /**
   * Get current animation name
   */
  getAnimationName(): string {
    return this.player.getAnimation().name;
  }
}
