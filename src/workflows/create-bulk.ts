/**
 * Bulk Locker Creation Workflow
 * @module workflows/create-bulk
 */

import { WizardRenderer } from '../ui/widgets/wizard';
import { createLockersBulk, getUniqueLocations } from '../db/lockers';
import { getOrCreateLocation } from '../db/locations';
import { createOptions, createSizeOptions } from './common';
import type { WizardAction } from '../types/ui';
import type { LockerSize } from '../types/database';

/**
 * Bulk creation workflow state
 */
export interface BulkCreateWorkflowState {
  step: 'location' | 'size' | 'count' | 'prefix' | 'confirm' | 'complete';
  location: string | null;
  size: LockerSize | null;
  count: number;
  prefix: string;
  startNumber: number;
}

/**
 * Create initial bulk creation workflow state
 */
export function createBulkCreateWorkflowState(): BulkCreateWorkflowState {
  return {
    step: 'location',
    location: null,
    size: null,
    count: 0,
    prefix: 'L',
    startNumber: 1,
  };
}

/**
 * Bulk Creation Workflow class
 */
export class BulkCreateWorkflow {
  private wizard: WizardRenderer;
  private state: BulkCreateWorkflowState;

  constructor() {
    this.wizard = new WizardRenderer();
    this.state = createBulkCreateWorkflowState();
    this.startWorkflow();
  }

  /**
   * Start the workflow
   */
  private startWorkflow(): void {
    this.wizard.addInfo('Schließfächer anlegen');
    this.showLocationStep();
  }

  /**
   * Show location selection step
   */
  private showLocationStep(): void {
    const locations = getUniqueLocations();

    const options = createOptions([
      ...locations.map((loc) => ({ label: loc, value: loc })),
      { label: '+ Neuer Standort: Gebäude A', value: 'Gebäude A' },
      { label: '+ Neuer Standort: Gebäude B', value: 'Gebäude B' },
      { label: '+ Neuer Standort: Hauptgebäude', value: 'Hauptgebäude' },
    ]);

    this.wizard.addQuestion('Standort auswählen:', options);
    this.state.step = 'location';
  }

  /**
   * Show size selection step
   */
  private showSizeStep(): void {
    this.wizard.addQuestion('Größe auswählen:', createSizeOptions());
    this.state.step = 'size';
  }

  /**
   * Show count selection step
   */
  private showCountStep(): void {
    const options = createOptions([
      { label: '5 Schließfächer', value: '5' },
      { label: '10 Schließfächer', value: '10' },
      { label: '20 Schließfächer', value: '20' },
      { label: '50 Schließfächer', value: '50' },
    ]);

    this.wizard.addQuestion('Anzahl auswählen:', options);
    this.state.step = 'count';
  }

  /**
   * Show prefix selection step
   */
  private showPrefixStep(): void {
    const options = createOptions([
      { label: 'L001, L002, ...', value: 'L' },
      { label: 'A001, A002, ...', value: 'A' },
      { label: 'B001, B002, ...', value: 'B' },
    ]);

    this.wizard.addQuestion('Nummerierung auswählen:', options);
    this.state.step = 'prefix';
  }

  /**
   * Show confirmation step
   */
  private showConfirmStep(): void {
    this.wizard.addInfo(`--- Zusammenfassung ---`);
    this.wizard.addInfo(`Standort: ${this.state.location}`);
    this.wizard.addInfo(`Größe: ${this.state.size}`);
    this.wizard.addInfo(`Anzahl: ${this.state.count}`);
    this.wizard.addInfo(`Nummern: ${this.state.prefix}001 - ${this.state.prefix}${this.state.count.toString().padStart(3, '0')}`);

    this.wizard.addQuestion('Schließfächer erstellen?', [
      { label: 'Erstellen', value: 'confirm' },
      { label: 'Abbrechen', value: 'cancel' },
    ]);
    this.state.step = 'confirm';
  }

  /**
   * Execute the bulk creation
   */
  private executeBulkCreate(): void {
    try {
      // Ensure location exists
      getOrCreateLocation(this.state.location!);

      // Create locker data
      const lockers = [];
      for (let i = 1; i <= this.state.count; i++) {
        lockers.push({
          number: `${this.state.prefix}${i.toString().padStart(3, '0')}`,
          location: this.state.location!,
          size: this.state.size!,
        });
      }

      // Create lockers
      const created = createLockersBulk(lockers);

      this.wizard.addInfo(`✓ ${created.length} Schließfächer erstellt!`, 'system');
    } catch (error) {
      this.wizard.addInfo(`✗ Fehler: ${error}`, 'system');
    }

    this.state.step = 'complete';
  }

  /**
   * Handle wizard action
   * @param action Wizard action
   * @returns True if workflow is complete
   */
  handleAction(action: WizardAction): boolean {
    if (action.type === 'cancel') {
      return true;
    }

    if (action.type !== 'next') {
      return false;
    }

    const selected = this.wizard.getSelectedOption();
    if (!selected) return false;

    switch (this.state.step) {
      case 'location':
        this.state.location = selected.value;
        this.showSizeStep();
        break;

      case 'size':
        this.state.size = selected.value as LockerSize;
        this.showCountStep();
        break;

      case 'count':
        this.state.count = parseInt(selected.value, 10);
        this.showPrefixStep();
        break;

      case 'prefix':
        this.state.prefix = selected.value;
        this.showConfirmStep();
        break;

      case 'confirm':
        if (selected.value === 'confirm') {
          this.executeBulkCreate();
        }
        return true;

      case 'complete':
        return true;
    }

    return false;
  }

  /**
   * Handle key input
   * @param key Key pressed
   * @returns True if workflow is complete
   */
  handleKey(key: string): boolean {
    const action = this.wizard.handleKey(key);
    return this.handleAction(action);
  }

  /**
   * Render the workflow
   * @param width Available width
   * @param height Available height
   * @returns Rendered lines
   */
  render(width: number, height: number): string[] {
    return this.wizard.render(width, height);
  }

  /**
   * Check if workflow is complete
   */
  isComplete(): boolean {
    return this.state.step === 'complete';
  }
}
