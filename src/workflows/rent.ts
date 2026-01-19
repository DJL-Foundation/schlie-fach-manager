/**
 * Rental Workflow
 * @module workflows/rent
 */

import { WizardRenderer } from '../ui/widgets/wizard';
import { getAvailableLockers } from '../db/lockers';
import { createRental } from '../db/rentals';
import { createPayment } from '../db/payments';
import { logAction } from '../db/audit';
import { DatabaseConnection } from '../db/connection';
import {
  createOptions,
  createYesNoOptions,
  createPaymentTypeOptions,
  formatCurrency,
  getCurrentDate,
  calculateEndDate,
} from './common';
import type { WizardOption, WizardAction } from '../types/ui';
import type { Locker, PaymentType } from '../types/database';

/**
 * Rental workflow state
 */
export interface RentWorkflowState {
  step: 'location' | 'size' | 'locker' | 'name' | 'duration' | 'deposit' | 'payment' | 'confirm' | 'complete';
  selectedLocation: string | null;
  selectedSize: string | null;
  selectedLocker: Locker | null;
  renterName: string | null;
  duration: number; // months
  depositPaid: boolean;
  paymentType: PaymentType | null;
}

/**
 * Create initial rental workflow state
 */
export function createRentWorkflowState(): RentWorkflowState {
  return {
    step: 'location',
    selectedLocation: null,
    selectedSize: null,
    selectedLocker: null,
    renterName: null,
    duration: 12,
    depositPaid: false,
    paymentType: null,
  };
}

/**
 * Rental Workflow class
 */
export class RentWorkflow {
  private wizard: WizardRenderer;
  private state: RentWorkflowState;
  private availableLockers: Locker[] = [];

  constructor() {
    this.wizard = new WizardRenderer();
    this.state = createRentWorkflowState();
    this.availableLockers = getAvailableLockers();
    this.startWorkflow();
  }

  /**
   * Start the workflow
   */
  private startWorkflow(): void {
    this.wizard.addInfo('Willkommen zum Verleih-Wizard!');
    this.showLocationStep();
  }

  /**
   * Show location selection step
   */
  private showLocationStep(): void {
    const locations = [...new Set(this.availableLockers.map((l) => l.location))];

    if (locations.length === 0) {
      this.wizard.addInfo('Keine verfügbaren Schließfächer vorhanden.');
      this.state.step = 'complete';
      return;
    }

    const options = createOptions(
      locations.map((loc) => {
        const count = this.availableLockers.filter((l) => l.location === loc).length;
        return {
          label: `${loc} (${count} verfügbar)`,
          value: loc,
        };
      })
    );

    this.wizard.addQuestion('Standort auswählen:', options);
    this.state.step = 'location';
  }

  /**
   * Show size selection step
   */
  private showSizeStep(): void {
    const sizes = ['S', 'M', 'L', 'XL'];
    const locationLockers = this.availableLockers.filter(
      (l) => l.location === this.state.selectedLocation
    );

    const options = createOptions(
      sizes
        .filter((size) => locationLockers.some((l) => l.size === size))
        .map((size) => {
          const count = locationLockers.filter((l) => l.size === size).length;
          const sizeLabels: Record<string, string> = {
            S: 'Klein',
            M: 'Mittel',
            L: 'Groß',
            XL: 'Extra Groß',
          };
          return {
            label: `${size} - ${sizeLabels[size]} (${count} verfügbar)`,
            value: size,
          };
        })
    );

    this.wizard.addQuestion('Größe auswählen:', options);
    this.state.step = 'size';
  }

  /**
   * Show locker selection step
   */
  private showLockerStep(): void {
    const matchingLockers = this.availableLockers.filter(
      (l) => l.location === this.state.selectedLocation && l.size === this.state.selectedSize
    );

    const options = createOptions(
      matchingLockers.map((locker) => ({
        label: `Schließfach ${locker.number}`,
        value: locker.id.toString(),
        metadata: { locker },
      }))
    );

    this.wizard.addQuestion('Schließfach auswählen:', options);
    this.state.step = 'locker';
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

    this.wizard.addQuestion('Mietdauer auswählen:', options);
    this.state.step = 'duration';
  }

  /**
   * Show deposit step
   */
  private showDepositStep(): void {
    const db = DatabaseConnection.getConnection();
    const depositSetting = db
      .prepare("SELECT value FROM settings WHERE key = 'deposit_cents'")
      .get() as { value: string } | undefined;
    const depositCents = depositSetting ? parseInt(depositSetting.value, 10) : 1000;

    this.wizard.addInfo(`Pfand: ${formatCurrency(depositCents)}`);
    this.wizard.addQuestion('Pfand bezahlt?', createYesNoOptions());
    this.state.step = 'deposit';
  }

  /**
   * Show payment type step
   */
  private showPaymentStep(): void {
    this.wizard.addQuestion('Zahlungsart:', createPaymentTypeOptions());
    this.state.step = 'payment';
  }

  /**
   * Show confirmation step
   */
  private showConfirmStep(): void {
    const locker = this.state.selectedLocker!;
    const startDate = getCurrentDate();
    const endDate = calculateEndDate(startDate, this.state.duration);

    this.wizard.addInfo(`--- Zusammenfassung ---`);
    this.wizard.addInfo(`Schließfach: ${locker.number} (${locker.size})`);
    this.wizard.addInfo(`Standort: ${locker.location}`);
    this.wizard.addInfo(`Mieter: ${this.state.renterName}`);
    this.wizard.addInfo(`Zeitraum: ${startDate} bis ${endDate}`);
    this.wizard.addInfo(`Pfand bezahlt: ${this.state.depositPaid ? 'Ja' : 'Nein'}`);

    this.wizard.addQuestion('Buchung bestätigen?', [
      { label: 'Bestätigen', value: 'confirm' },
      { label: 'Abbrechen', value: 'cancel' },
    ]);
    this.state.step = 'confirm';
  }

  /**
   * Execute the rental
   */
  private executeRental(): void {
    const locker = this.state.selectedLocker!;
    const startDate = getCurrentDate();
    const endDate = calculateEndDate(startDate, this.state.duration);

    try {
      const rental = createRental(
        locker.id,
        this.state.renterName!,
        startDate,
        endDate,
        undefined,
        undefined,
        this.state.depositPaid
      );

      if (this.state.depositPaid && this.state.paymentType) {
        const db = DatabaseConnection.getConnection();
        const depositSetting = db
          .prepare("SELECT value FROM settings WHERE key = 'deposit_cents'")
          .get() as { value: string } | undefined;
        const depositCents = depositSetting ? parseInt(depositSetting.value, 10) : 1000;

        createPayment(rental.id, depositCents, startDate, this.state.paymentType);
      }

      this.wizard.addInfo('✓ Buchung erfolgreich!', 'system');
      this.wizard.addInfo(`Buchungsnummer: ${rental.id}`);
    } catch (error) {
      this.wizard.addInfo(`✗ Fehler bei der Buchung: ${error}`, 'system');
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
        this.state.selectedLocation = selected.value;
        this.showSizeStep();
        break;

      case 'size':
        this.state.selectedSize = selected.value;
        this.showLockerStep();
        break;

      case 'locker':
        const lockerId = parseInt(selected.value, 10);
        this.state.selectedLocker = this.availableLockers.find((l) => l.id === lockerId) || null;
        // For simplicity, use a default name (in real app, would have text input)
        this.state.renterName = 'Neuer Mieter';
        this.showDurationStep();
        break;

      case 'duration':
        this.state.duration = parseInt(selected.value, 10);
        this.showDepositStep();
        break;

      case 'deposit':
        this.state.depositPaid = selected.value === 'yes';
        if (this.state.depositPaid) {
          this.showPaymentStep();
        } else {
          this.showConfirmStep();
        }
        break;

      case 'payment':
        this.state.paymentType = selected.value as PaymentType;
        this.showConfirmStep();
        break;

      case 'confirm':
        if (selected.value === 'confirm') {
          this.executeRental();
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
