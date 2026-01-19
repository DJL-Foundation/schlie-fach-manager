/**
 * Header Widget with Window Switcher
 * @module ui/widgets/header
 */

import type { HeaderProps, WindowInfoUI } from '../../types/ui';
import { Theme, colorize, ANSI } from '../theme';
import { padText } from '../index';

/**
 * Header component for displaying application title and window switcher
 */
export class Header {
  private props: HeaderProps;
  private theme: Theme;

  constructor(props: HeaderProps) {
    this.props = props;
    this.theme = Theme.defaultDark();
  }

  /**
   * Update header properties
   * @param props New properties
   */
  update(props: Partial<HeaderProps>): void {
    this.props = { ...this.props, ...props };
  }

  /**
   * Render the header to string
   * @param width Available width
   * @returns Rendered header lines
   */
  render(width: number): string[] {
    if (this.props.windowSwitcherActive) {
      return this.renderWindowSwitcher(width);
    }
    return this.renderBasicHeader(width);
  }

  /**
   * Render basic header (title + screen name)
   */
  private renderBasicHeader(width: number): string[] {
    const lines: string[] = [];
    const theme = this.theme;

    // Top border
    const topBorder = colorize('╔' + '═'.repeat(width - 2) + '╗', theme.get('border'));
    lines.push(topBorder);

    // Title line
    const title = ` Schließfach-Manager v${this.props.version} `;
    const screenName = ` [${this.props.currentScreen}] `;
    const padding = width - title.length - screenName.length - 2;

    let middleLine = colorize('║', theme.get('border'));
    middleLine += colorize(title, theme.get('primary'));
    middleLine += ' '.repeat(Math.max(0, padding));
    middleLine += colorize(screenName, theme.get('accent'));
    middleLine += colorize('║', theme.get('border'));
    lines.push(middleLine);

    // Bottom border
    const bottomBorder = colorize('╚' + '═'.repeat(width - 2) + '╝', theme.get('border'));
    lines.push(bottomBorder);

    return lines;
  }

  /**
   * Render window switcher view
   */
  private renderWindowSwitcher(width: number): string[] {
    const lines: string[] = [];
    const theme = this.theme;

    // Top border
    const topBorder = colorize('╔' + '═'.repeat(width - 2) + '╗', theme.get('borderActive'));
    lines.push(topBorder);

    // Window list line (3-part preview)
    const windowList = this.props.windowList;
    const selected = this.props.selectedWindowIndex;
    const prevIdx = selected > 0 ? selected - 1 : windowList.length - 1;
    const nextIdx = (selected + 1) % windowList.length;

    const prev = windowList[prevIdx];
    const curr = windowList[selected];
    const next = windowList[nextIdx];

    const sectionWidth = Math.floor((width - 4) / 3);

    let middleLine = colorize('║ ', theme.get('borderActive'));

    // Previous (dim)
    middleLine += colorize(padText(prev.name, sectionWidth, 'center'), theme.get('windowAdjacent'));

    // Current (highlighted)
    const currText = `[ ${curr.name} ]`;
    middleLine += colorize(padText(currText, sectionWidth, 'center'), theme.get('windowCurrent'));

    // Next (dim)
    middleLine += colorize(padText(next.name, sectionWidth, 'center'), theme.get('windowAdjacent'));

    middleLine += colorize(' ║', theme.get('borderActive'));
    lines.push(middleLine);

    // Bottom border with instructions
    const instructions = ' Tab/Shift+Tab: Navigate | Enter: Select | Esc: Cancel ';
    const instructionsWidth = instructions.length;
    const bottomPadding = Math.floor((width - instructionsWidth - 2) / 2);
    const rightPadding = width - instructionsWidth - bottomPadding - 2;

    let bottomLine = colorize('╚' + '═'.repeat(bottomPadding), theme.get('borderActive'));
    bottomLine += colorize(instructions, theme.get('info'));
    bottomLine += colorize('═'.repeat(rightPadding) + '╝', theme.get('borderActive'));
    lines.push(bottomLine);

    return lines;
  }

  /**
   * Get current screen name
   */
  getCurrentScreen(): string {
    return this.props.currentScreen;
  }

  /**
   * Check if window switcher is active
   */
  isWindowSwitcherActive(): boolean {
    return this.props.windowSwitcherActive;
  }
}

/**
 * Create default header props
 * @param currentScreen Current screen name
 * @param windowList List of available windows
 * @returns Default header props
 */
export function createDefaultHeaderProps(
  currentScreen: string,
  windowList: WindowInfoUI[]
): HeaderProps {
  return {
    version: '2.1.0',
    currentScreen,
    windowSwitcherActive: false,
    windowList,
    selectedWindowIndex: 0,
  };
}
