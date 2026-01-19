import { useEffect, useCallback, useState } from 'react';
import { useNavigate } from 'react-router-dom';

interface Shortcut {
  key: string;
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  action: () => void;
  description: string;
  category: string;
}

/**
 * Custom hook for global keyboard shortcuts
 */
export function useGlobalShortcuts(onShowShortcuts: () => void) {
  const navigate = useNavigate();

  const shortcuts: Shortcut[] = [
    // Navigation
    { key: 'h', ctrl: true, action: () => navigate('/'), description: 'Dashboard', category: 'Navigation' },
    { key: 'n', ctrl: true, action: () => navigate('/rentals/new'), description: 'Neuer Verleih', category: 'Navigation' },
    { key: 'r', ctrl: true, action: () => navigate('/rentals/return'), description: 'Rückgabe', category: 'Navigation' },
    { key: ',', ctrl: true, action: () => navigate('/management/settings'), description: 'Einstellungen', category: 'Navigation' },
    
    // Global
    { key: '?', ctrl: true, action: onShowShortcuts, description: 'Shortcuts anzeigen', category: 'Global' },
    { key: '/', ctrl: true, action: onShowShortcuts, description: 'Shortcuts anzeigen', category: 'Global' },
  ];

  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      // Don't trigger shortcuts when typing in inputs
      const target = event.target as HTMLElement;
      if (
        target.tagName === 'INPUT' ||
        target.tagName === 'TEXTAREA' ||
        target.tagName === 'SELECT' ||
        target.isContentEditable
      ) {
        return;
      }

      const shortcut = shortcuts.find(
        (s) =>
          s.key.toLowerCase() === event.key.toLowerCase() &&
          !!s.ctrl === event.ctrlKey &&
          !!s.shift === event.shiftKey &&
          !!s.alt === event.altKey
      );

      if (shortcut) {
        event.preventDefault();
        shortcut.action();
      }
    },
    [shortcuts]
  );

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);

  return shortcuts;
}

/**
 * Hook to manage shortcuts overlay state
 */
export function useShortcutsOverlay() {
  const [isOpen, setIsOpen] = useState(false);
  
  const open = useCallback(() => setIsOpen(true), []);
  const close = useCallback(() => setIsOpen(false), []);
  const toggle = useCallback(() => setIsOpen((prev) => !prev), []);

  // Close on Escape
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isOpen) {
        close();
      }
    };

    window.addEventListener('keydown', handleEscape);
    return () => window.removeEventListener('keydown', handleEscape);
  }, [isOpen, close]);

  return { isOpen, open, close, toggle };
}
