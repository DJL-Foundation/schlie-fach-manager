/**
 * Extend Rental Workflow
 * @module workflows/extend
 */

import { WizardRenderer } from '../ui/widgets/wizard';
import { getActiveRentals, extendRental } from '../db/rentals';
import { createOptions, formatDate, calculateEndDate } from './common';
import type { WizardAction } from '../types/ui';
import type { RentalWithLocker } from '../types/database';

/**
 * Extend workflow state
 */
export interface ExtendWorkflowState {
  step: 'select' | 'duration' | 'confirm' | 'complete';
  selectedRental: RentalWithLocker | null;
  extensionMonths: number;
  newEndDate: string | null;
}

/**
 * Create initial extend workflow state
 */
export function createExtendWorkflowState(): ExtendWorkflowState {
  return {
    step: 'select',
    selectedRental: null,
    extensionMonths: 0,
    newEndDate: null,
  };
}

/**
 * Extend Rental Workflow class
 */
export class ExtendWorkflow {
  private wizard: WizardRenderer;
  private state: ExtendWorkflowState;
  private activeRentals: RentalWithLocker[] = [];

  constructor() {
    this.wizard = new WizardRenderer();
    this.state = createExtendWorkflowState();
    this.activeRentals = getActiveRentals();
    this.startWorkflow();
  }

  /**
   * Start the workflow
   */
  private startWorkflow(): void {
    this.wizard.addInfo('Verlängerungs-Wizard');
    this.showSelectStep();
  }

  /**
   * Show rental selection step
   */
  private showSelectStep(): void {
    if (this.activeRentals.length === 0) {
      this.wizard.addInfo('Keine aktiven Verleih gefunden.');
      this.state.step = 'complete';
      return;
    }

    const options = createOptions(
      this.activeRentals.map((rental) => ({
        label: `${rental.locker_number} - ${rental.renter_name} (bis ${formatDate(rental.end_date || 'unbegrenzt')})`,
        value: rental.id.toString(),
        metadata: { rental },
      }))
    );

    this.wizard.addQuestion('Verleih auswählen:', options);
    this.state.step = 'select';
  }

  /**
   * Show duration selection step
   */
  private showDurationStep(): void {
    const options = createOptions([
      { label: '1 Monat', value: '1' },
      { label: '3 Monate', value: '3' },
      { label: '6 Monate', value: '6' },
      { label: '12 Monate (1 Jahr)', value: '12' },
    ]);

    this.wizard.addQuestion('Verlängerungszeitraum auswählen:', options);
    this.state.step = 'duration';
  }

  /**
   * Show confirmation step
   */
  private showConfirmStep(): void {
    const rental = this.state.selectedRental!;
    const currentEndDate = rental.end_date || new Date().toISOString().split('T')[0];
    this.state.newEndDate = calculateEndDate(currentEndDate, this.state.extensionMonths);

    this.wizard.addInfo(`--- Verlängerung ---`);
    this.wizard.addInfo(`Schließfach: ${rental.locker_number}`);
    this.wizard.addInfo(`Mieter: ${rental.renter_name}`);
    this.wizard.addInfo(`Aktuelles Ende: ${formatDate(currentEndDate)}`);
    this.wizard.addInfo(`Neues Ende: ${formatDate(this.state.newEndDate)}`);

    this.wizard.addQuestion('Verlängerung bestätigen?', [
      { label: 'Bestätigen', value: 'confirm' },
      { label: 'Abbrechen', value: 'cancel' },
    ]);
    this.state.step = 'confirm';
  }

  /**
   * Execute the extension
   */
  private executeExtension(): void {
    const rental = this.state.selectedRental!;

    try {
      extendRental(rental.id, this.state.newEndDate!);

      this.wizard.addInfo('✓ Verlängerung erfolgreich!', 'system');
      this.wizard.addInfo(`Neues Ende: ${formatDate(this.state.newEndDate!)}`);
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
      case 'select':
        const rentalId = parseInt(selected.value, 10);
        this.state.selectedRental = this.activeRentals.find((r) => r.id === rentalId) || null;
        this.showDurationStep();
        break;

      case 'duration':
        this.state.extensionMonths = parseInt(selected.value, 10);
        this.showConfirmStep();
        break;

      case 'confirm':
        if (selected.value === 'confirm') {
          this.executeExtension();
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
