/**
 * Return Locker Workflow
 * @module workflows/return-locker
 */

import { WizardRenderer } from '../ui/widgets/wizard';
import { getActiveRentals, endRental } from '../db/rentals';
import { DatabaseConnection } from '../db/connection';
import { createOptions, createYesNoOptions, formatCurrency, getCurrentDate } from './common';
import type { WizardAction } from '../types/ui';
import type { RentalWithLocker } from '../types/database';

/**
 * Return workflow state
 */
export interface ReturnWorkflowState {
  step: 'select' | 'deposit' | 'damage' | 'confirm' | 'complete';
  selectedRental: RentalWithLocker | null;
  returnDeposit: boolean;
  hasDamage: boolean;
}

/**
 * Create initial return workflow state
 */
export function createReturnWorkflowState(): ReturnWorkflowState {
  return {
    step: 'select',
    selectedRental: null,
    returnDeposit: false,
    hasDamage: false,
  };
}

/**
 * Return Locker Workflow class
 */
export class ReturnWorkflow {
  private wizard: WizardRenderer;
  private state: ReturnWorkflowState;
  private activeRentals: RentalWithLocker[] = [];

  constructor() {
    this.wizard = new WizardRenderer();
    this.state = createReturnWorkflowState();
    this.activeRentals = getActiveRentals();
    this.startWorkflow();
  }

  /**
   * Start the workflow
   */
  private startWorkflow(): void {
    this.wizard.addInfo('Rückgabe-Wizard');
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
        label: `${rental.locker_number} - ${rental.renter_name}`,
        value: rental.id.toString(),
        metadata: { rental },
      }))
    );

    this.wizard.addQuestion('Verleih auswählen:', options);
    this.state.step = 'select';
  }

  /**
   * Show deposit return step
   */
  private showDepositStep(): void {
    const rental = this.state.selectedRental!;
    
    if (!rental.deposit_paid) {
      this.wizard.addInfo('Kein Pfand wurde bezahlt.');
      this.showDamageStep();
      return;
    }

    const db = DatabaseConnection.getConnection();
    const depositSetting = db
      .prepare("SELECT value FROM settings WHERE key = 'deposit_cents'")
      .get() as { value: string } | undefined;
    const depositCents = depositSetting ? parseInt(depositSetting.value, 10) : 1000;

    this.wizard.addInfo(`Pfand: ${formatCurrency(depositCents)}`);
    this.wizard.addQuestion('Pfand zurückgeben?', createYesNoOptions());
    this.state.step = 'deposit';
  }

  /**
   * Show damage check step
   */
  private showDamageStep(): void {
    this.wizard.addQuestion('Gibt es Schäden am Schließfach?', createYesNoOptions());
    this.state.step = 'damage';
  }

  /**
   * Show confirmation step
   */
  private showConfirmStep(): void {
    const rental = this.state.selectedRental!;

    this.wizard.addInfo(`--- Rückgabe ---`);
    this.wizard.addInfo(`Schließfach: ${rental.locker_number}`);
    this.wizard.addInfo(`Mieter: ${rental.renter_name}`);
    this.wizard.addInfo(`Pfand zurückgeben: ${this.state.returnDeposit ? 'Ja' : 'Nein'}`);
    this.wizard.addInfo(`Schäden: ${this.state.hasDamage ? 'Ja' : 'Nein'}`);

    this.wizard.addQuestion('Rückgabe bestätigen?', [
      { label: 'Bestätigen', value: 'confirm' },
      { label: 'Abbrechen', value: 'cancel' },
    ]);
    this.state.step = 'confirm';
  }

  /**
   * Execute the return
   */
  private executeReturn(): void {
    const rental = this.state.selectedRental!;

    try {
      // End the rental
      endRental(rental.id, this.state.returnDeposit);

      // If damage, mark locker as damaged
      if (this.state.hasDamage) {
        const db = DatabaseConnection.getConnection();
        db.prepare('UPDATE lockers SET is_damaged = 1 WHERE id = ?').run(rental.locker_id);
      }

      this.wizard.addInfo('✓ Rückgabe erfolgreich!', 'system');
      
      if (this.state.returnDeposit) {
        this.wizard.addInfo('Pfand wurde zurückgegeben.');
      }
      
      if (this.state.hasDamage) {
        this.wizard.addInfo('Schließfach wurde als beschädigt markiert.');
      }
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
        this.showDepositStep();
        break;

      case 'deposit':
        this.state.returnDeposit = selected.value === 'yes';
        this.showDamageStep();
        break;

      case 'damage':
        this.state.hasDamage = selected.value === 'yes';
        this.showConfirmStep();
        break;

      case 'confirm':
        if (selected.value === 'confirm') {
          this.executeReturn();
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
