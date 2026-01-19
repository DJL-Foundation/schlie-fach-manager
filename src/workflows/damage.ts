/**
 * Damage Reporting Workflow
 * @module workflows/damage
 */

import { WizardRenderer } from '../ui/widgets/wizard';
import { getAllLockers, updateLockerDamageStatus } from '../db/lockers';
import { createOptions, createYesNoOptions } from './common';
import type { WizardAction } from '../types/ui';
import type { Locker } from '../types/database';

/**
 * Damage workflow state
 */
export interface DamageWorkflowState {
  step: 'select' | 'action' | 'notes' | 'confirm' | 'complete';
  selectedLocker: Locker | null;
  action: 'report' | 'repair' | null;
  notes: string | null;
}

/**
 * Create initial damage workflow state
 */
export function createDamageWorkflowState(): DamageWorkflowState {
  return {
    step: 'select',
    selectedLocker: null,
    action: null,
    notes: null,
  };
}

/**
 * Damage Reporting Workflow class
 */
export class DamageWorkflow {
  private wizard: WizardRenderer;
  private state: DamageWorkflowState;
  private allLockers: Locker[] = [];

  constructor() {
    this.wizard = new WizardRenderer();
    this.state = createDamageWorkflowState();
    this.allLockers = getAllLockers();
    this.startWorkflow();
  }

  /**
   * Start the workflow
   */
  private startWorkflow(): void {
    this.wizard.addInfo('Schadensmeldung / Reparatur');
    this.showSelectStep();
  }

  /**
   * Show locker selection step
   */
  private showSelectStep(): void {
    if (this.allLockers.length === 0) {
      this.wizard.addInfo('Keine Schließfächer vorhanden.');
      this.state.step = 'complete';
      return;
    }

    const options = createOptions(
      this.allLockers.map((locker) => ({
        label: `${locker.number} (${locker.location}) ${locker.is_damaged ? '⚠️ beschädigt' : '✓ intakt'}`,
        value: locker.id.toString(),
        metadata: { locker },
      }))
    );

    this.wizard.addQuestion('Schließfach auswählen:', options);
    this.state.step = 'select';
  }

  /**
   * Show action selection step
   */
  private showActionStep(): void {
    const locker = this.state.selectedLocker!;

    const options = locker.is_damaged
      ? createOptions([
          { label: 'Als repariert markieren', value: 'repair' },
          { label: 'Abbrechen', value: 'cancel' },
        ])
      : createOptions([
          { label: 'Schaden melden', value: 'report' },
          { label: 'Abbrechen', value: 'cancel' },
        ]);

    this.wizard.addQuestion(
      locker.is_damaged ? 'Schließfach repariert?' : 'Aktion auswählen:',
      options
    );
    this.state.step = 'action';
  }

  /**
   * Show confirmation step
   */
  private showConfirmStep(): void {
    const locker = this.state.selectedLocker!;

    this.wizard.addInfo(`--- ${this.state.action === 'report' ? 'Schadensmeldung' : 'Reparatur'} ---`);
    this.wizard.addInfo(`Schließfach: ${locker.number}`);
    this.wizard.addInfo(`Standort: ${locker.location}`);
    this.wizard.addInfo(`Aktion: ${this.state.action === 'report' ? 'Als beschädigt markieren' : 'Als repariert markieren'}`);

    this.wizard.addQuestion('Bestätigen?', [
      { label: 'Bestätigen', value: 'confirm' },
      { label: 'Abbrechen', value: 'cancel' },
    ]);
    this.state.step = 'confirm';
  }

  /**
   * Execute the damage action
   */
  private executeDamageAction(): void {
    const locker = this.state.selectedLocker!;
    const isDamaged = this.state.action === 'report';

    try {
      updateLockerDamageStatus(locker.id, isDamaged, this.state.notes || undefined);

      if (isDamaged) {
        this.wizard.addInfo('✓ Schaden wurde gemeldet!', 'system');
        this.wizard.addInfo('Schließfach ist jetzt als beschädigt markiert.');
      } else {
        this.wizard.addInfo('✓ Reparatur wurde vermerkt!', 'system');
        this.wizard.addInfo('Schließfach ist wieder verfügbar.');
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
        const lockerId = parseInt(selected.value, 10);
        this.state.selectedLocker = this.allLockers.find((l) => l.id === lockerId) || null;
        this.showActionStep();
        break;

      case 'action':
        if (selected.value === 'cancel') {
          return true;
        }
        this.state.action = selected.value as 'report' | 'repair';
        this.showConfirmStep();
        break;

      case 'confirm':
        if (selected.value === 'confirm') {
          this.executeDamageAction();
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
