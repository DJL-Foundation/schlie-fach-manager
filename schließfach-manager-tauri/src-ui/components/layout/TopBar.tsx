import { Home, ChevronRight } from 'lucide-react';
import { useLocation, Link } from 'react-router-dom';
import { useMemo } from 'react';

interface BreadcrumbItem {
  label: string;
  path: string;
}

const pathLabels: Record<string, string> = {
  '': 'Dashboard',
  rentals: 'Verleih',
  overview: 'Übersicht',
  new: 'Neu verleihen',
  active: 'Aktive Verleihe',
  overdue: 'Überfällig',
  extend: 'Verlängern',
  return: 'Rückgabe',
  history: 'Verlauf',
  finances: 'Finanzen',
  payments: 'Zahlungen',
  reports: 'Berichte',
  management: 'Verwaltung',
  lockers: 'Schließfächer',
  locations: 'Standorte',
  settings: 'Einstellungen',
  audit: 'Audit-Log',
  export: 'Export/Import',
};

export function TopBar() {
  const location = useLocation();

  const breadcrumb = useMemo<BreadcrumbItem[]>(() => {
    const parts = location.pathname.split('/').filter(Boolean);
    const items: BreadcrumbItem[] = [{ label: 'Dashboard', path: '/' }];

    let currentPath = '';
    for (const part of parts) {
      currentPath += `/${part}`;
      const label = pathLabels[part] || part;
      items.push({ label, path: currentPath });
    }

    return items;
  }, [location.pathname]);

  return (
    <header className="h-16 bg-crust border-b border-surface-1 flex items-center px-6 flex-shrink-0">
      {/* Logo */}
      <div className="flex items-center gap-3">
        <div className="w-8 h-8 bg-primary rounded-md flex items-center justify-center">
          <Home className="w-5 h-5 text-base" />
        </div>
        <span className="font-semibold text-lg text-text">
          Schließfach-Manager
        </span>
        <span className="text-xs text-subtext-0 bg-surface-0 px-2 py-0.5 rounded">
          v2.1-Tauri
        </span>
      </div>

      {/* Breadcrumb */}
      <nav className="ml-8 flex items-center gap-2 text-sm">
        {breadcrumb.map((item, index) => (
          <div key={item.path} className="flex items-center gap-2">
            {index > 0 && (
              <ChevronRight className="w-4 h-4 text-overlay-1" />
            )}
            {index === breadcrumb.length - 1 ? (
              <span className="text-text font-medium">{item.label}</span>
            ) : (
              <Link
                to={item.path}
                className="text-subtext-0 hover:text-text transition-colors"
              >
                {item.label}
              </Link>
            )}
          </div>
        ))}
      </nav>

      {/* Right Section */}
      <div className="ml-auto flex items-center gap-4">
        {/* Keyboard Shortcuts Hint */}
        <div className="text-xs text-subtext-0 bg-surface-0 px-2 py-1 rounded border border-surface-1">
          <kbd className="font-mono">Strg</kbd> + <kbd className="font-mono">?</kbd>
        </div>
      </div>
    </header>
  );
}
