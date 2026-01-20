import { X, Keyboard } from 'lucide-react';
import { Modal } from '@/components/ui/Modal';

interface ShortcutItem {
  keys: string[];
  description: string;
}

interface ShortcutCategory {
  name: string;
  shortcuts: ShortcutItem[];
}

const shortcutCategories: ShortcutCategory[] = [
  {
    name: 'Navigation',
    shortcuts: [
      { keys: ['Strg', 'H'], description: 'Dashboard' },
      { keys: ['Strg', 'N'], description: 'Neuer Verleih' },
      { keys: ['Strg', 'R'], description: 'Rückgabe' },
      { keys: ['Strg', ','], description: 'Einstellungen' },
    ],
  },
  {
    name: 'Global',
    shortcuts: [
      { keys: ['Strg', '?'], description: 'Shortcuts anzeigen' },
      { keys: ['Esc'], description: 'Dialog schließen' },
    ],
  },
  {
    name: 'Tabellen',
    shortcuts: [
      { keys: ['↑', '↓'], description: 'Zeile wechseln' },
      { keys: ['Enter'], description: 'Details öffnen' },
      { keys: ['Space'], description: 'Auswählen' },
    ],
  },
  {
    name: 'Formulare',
    shortcuts: [
      { keys: ['Tab'], description: 'Nächstes Feld' },
      { keys: ['Shift', 'Tab'], description: 'Vorheriges Feld' },
      { keys: ['Enter'], description: 'Absenden' },
    ],
  },
];

interface ShortcutsOverlayProps {
  open: boolean;
  onClose: () => void;
}

export function ShortcutsOverlay({ open, onClose }: ShortcutsOverlayProps) {
  return (
    <Modal open={open} onClose={onClose} title="Tastaturkürzel" size="lg">
      <div className="space-y-6">
        {shortcutCategories.map((category) => (
          <div key={category.name}>
            <h3 className="text-sm font-semibold text-subtext-1 uppercase tracking-wide mb-3">
              {category.name}
            </h3>
            <div className="space-y-2">
              {category.shortcuts.map((shortcut, index) => (
                <div
                  key={index}
                  className="flex items-center justify-between py-2 px-3 rounded-md bg-base"
                >
                  <span className="text-text">{shortcut.description}</span>
                  <div className="flex items-center gap-1">
                    {shortcut.keys.map((key, keyIndex) => (
                      <span key={keyIndex}>
                        {keyIndex > 0 && (
                          <span className="text-subtext-0 mx-1">+</span>
                        )}
                        <kbd className="px-2 py-1 text-xs font-mono bg-surface-1 border border-surface-2 rounded text-text">
                          {key}
                        </kbd>
                      </span>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>

      <div className="mt-6 pt-4 border-t border-surface-1 text-center text-sm text-subtext-0">
        <div className="flex items-center justify-center gap-2">
          <Keyboard className="w-4 h-4" />
          <span>Drücken Sie <kbd className="px-1.5 py-0.5 text-xs font-mono bg-surface-1 border border-surface-2 rounded">Esc</kbd> zum Schließen</span>
        </div>
      </div>
    </Modal>
  );
}
