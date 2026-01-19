/**
 * Wizard Widget for Dialog-Style Interactions
 * @module ui/widgets/wizard
 */

import type { WizardMessage, WizardOption, WizardAction, MessageContent } from '../../types/ui';
import { Theme, colorize } from '../theme';

/**
 * Wizard renderer for chat-style dialog interactions
 */
export class WizardRenderer {
  private messages: WizardMessage[] = [];
  private currentQuestionIndex = -1;
  private selectedOptionIndex = 0;
  private theme: Theme;

  constructor() {
    this.theme = Theme.defaultDark();
  }

  /**
   * Add a message to the wizard
   * @param message Wizard message to add
   */
  addMessage(message: WizardMessage): void {
    this.messages.push(message);

    if (message.content.type === 'question') {
      this.currentQuestionIndex = this.messages.length - 1;
      this.selectedOptionIndex = 0;
    }
  }

  /**
   * Add a system question
   * @param text Question text
   * @param options Available options
   */
  addQuestion(text: string, options: WizardOption[]): void {
    this.addMessage({
      sender: 'system',
      content: { type: 'question', text, options },
      timestamp: Date.now(),
    });
  }

  /**
   * Add an info message
   * @param text Info text
   * @param sender Message sender
   */
  addInfo(text: string, sender: 'system' | 'user' = 'system'): void {
    this.addMessage({
      sender,
      content: { type: 'info', text },
      timestamp: Date.now(),
    });
  }

  /**
   * Clear all messages and reset state
   */
  clear(): void {
    this.messages = [];
    this.currentQuestionIndex = -1;
    this.selectedOptionIndex = 0;
  }

  /**
   * Render the wizard to string array
   * @param width Available width
   * @param height Available height
   * @returns Rendered wizard lines
   */
  render(width: number, height: number): string[] {
    const lines: string[] = [];

    // Draw border
    lines.push(colorize('╔═══ Dialog ' + '═'.repeat(width - 13) + '╗', this.theme.get('border')));

    // Calculate available content height
    const contentHeight = height - 2;
    const maxMessages = Math.floor(contentHeight / 2);

    // Get visible messages
    const visibleMessages = this.messages.slice(-maxMessages);

    // Render messages
    for (const message of visibleMessages) {
      const messageLine = this.renderMessageLine(message, width);
      lines.push(messageLine);
    }

    // Render current question options
    const currentQuestion = this.getCurrentQuestion();
    if (currentQuestion && currentQuestion.content.type === 'question') {
      lines.push(this.renderEmptyLine(width));

      const options = currentQuestion.content.options;
      for (let i = 0; i < options.length && lines.length < height - 1; i++) {
        const optionLine = this.renderOptionLine(options[i], i === this.selectedOptionIndex, width);
        lines.push(optionLine);
      }
    }

    // Fill remaining space
    while (lines.length < height - 1) {
      lines.push(this.renderEmptyLine(width));
    }

    // Bottom border
    lines.push(colorize('╚' + '═'.repeat(width - 2) + '╝', this.theme.get('border')));

    return lines;
  }

  /**
   * Render a message line
   */
  private renderMessageLine(message: WizardMessage, width: number): string {
    const isSystem = message.sender === 'system';
    const icon = isSystem ? '🖥 ' : '👤 ';
    const textColor = isSystem ? this.theme.get('wizardSystem') : this.theme.get('wizardUser');

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

    let line = colorize('║', this.theme.get('border'));
    line += icon;
    line += colorize(truncatedText, textColor);
    line += colorize('║', this.theme.get('border'));

    return line;
  }

  /**
   * Render an option line
   */
  private renderOptionLine(option: WizardOption, isSelected: boolean, width: number): string {
    const prefix = isSelected ? '▶ ' : '  ';
    const color = isSelected
      ? this.theme.get('wizardOptionSelected')
      : this.theme.get('wizardOptionNormal');

    const label = (prefix + option.label).substring(0, width - 4).padEnd(width - 4);

    let line = colorize('║', this.theme.get('border'));
    line += colorize(label, color);
    line += colorize('║', this.theme.get('border'));

    return line;
  }

  /**
   * Render an empty line
   */
  private renderEmptyLine(width: number): string {
    return (
      colorize('║', this.theme.get('border')) +
      ' '.repeat(width - 2) +
      colorize('║', this.theme.get('border'))
    );
  }

  /**
   * Handle key input
   * @param key Key pressed
   * @returns Wizard action
   */
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
          timestamp: Date.now(),
        });
        return { type: 'next' };

      case 'escape':
        return { type: 'cancel' };

      default:
        return { type: 'none' };
    }
  }

  /**
   * Get the current question
   */
  private getCurrentQuestion(): WizardMessage | null {
    if (this.currentQuestionIndex >= 0 && this.currentQuestionIndex < this.messages.length) {
      return this.messages[this.currentQuestionIndex];
    }
    return null;
  }

  /**
   * Get the currently selected option
   */
  getSelectedOption(): WizardOption | null {
    const question = this.getCurrentQuestion();

    if (question && question.content.type === 'question') {
      return question.content.options[this.selectedOptionIndex];
    }

    return null;
  }

  /**
   * Get all messages
   */
  getMessages(): WizardMessage[] {
    return [...this.messages];
  }

  /**
   * Get current selected index
   */
  getSelectedIndex(): number {
    return this.selectedOptionIndex;
  }
}
