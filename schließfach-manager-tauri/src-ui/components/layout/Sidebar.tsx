import { useState } from 'react';
import {
  Home,
  Archive,
  DollarSign,
  Settings,
  ChevronRight,
  ChevronLeft,
  Plus,
  List,
  Clock,
  Check,
  History,
  AlertTriangle,
  ArrowUpRight,
  RotateCcw,
  BarChart3,
  FileText,
  Lock,
  MapPin,
  Cog,
  ScrollText,
  Download,
} from 'lucide-react';
import { Link, useLocation } from 'react-router-dom';
import { cn } from '@/lib/cn';
import type { NavItem } from '@/types';

const NAV_ITEMS: NavItem[] = [
  {
    label: 'Dashboard',
    icon: Home,
    path: '/',
  },
  {
    label: 'Verleih',
    icon: Archive,
    children: [
      { label: 'Übersicht', icon: List, path: '/rentals/overview' },
      { label: 'Neu verleihen', icon: Plus, path: '/rentals/new' },
      { label: 'Aktive Verleihe', icon: Check, path: '/rentals/active' },
      { label: 'Überfällig', icon: AlertTriangle, path: '/rentals/overdue' },
      { label: 'Verlängern', icon: ArrowUpRight, path: '/rentals/extend' },
      { label: 'Rückgabe', icon: RotateCcw, path: '/rentals/return' },
      { label: 'Verlauf', icon: History, path: '/rentals/history' },
    ],
  },
  {
    label: 'Finanzen',
    icon: DollarSign,
    children: [
      { label: 'Übersicht', icon: BarChart3, path: '/finances/overview' },
      { label: 'Zahlungen', icon: DollarSign, path: '/finances/payments' },
      { label: 'Berichte', icon: FileText, path: '/finances/reports' },
    ],
  },
  {
    label: 'Verwaltung',
    icon: Settings,
    children: [
      { label: 'Schließfächer', icon: Lock, path: '/management/lockers' },
      { label: 'Standorte', icon: MapPin, path: '/management/locations' },
      { label: 'Einstellungen', icon: Cog, path: '/management/settings' },
      { label: 'Audit-Log', icon: ScrollText, path: '/management/audit' },
      { label: 'Export/Import', icon: Download, path: '/management/export' },
    ],
  },
];

export function Sidebar() {
  const location = useLocation();
  const [collapsed, setCollapsed] = useState(false);
  const [expandedCategories, setExpandedCategories] = useState<Set<string>>(
    new Set(['Verleih', 'Finanzen', 'Verwaltung'])
  );

  const toggleCategory = (label: string) => {
    setExpandedCategories((prev) => {
      const next = new Set(prev);
      if (next.has(label)) {
        next.delete(label);
      } else {
        next.add(label);
      }
      return next;
    });
  };

  const isActive = (path: string) => {
    if (path === '/') {
      return location.pathname === '/';
    }
    return location.pathname.startsWith(path);
  };

  return (
    <aside
      className={cn(
        'bg-mantle border-r border-surface-1 flex flex-col flex-shrink-0 transition-all duration-base',
        collapsed ? 'w-16' : 'w-60'
      )}
    >
      {/* Collapse Toggle */}
      <button
        onClick={() => setCollapsed(!collapsed)}
        className="h-12 flex items-center justify-center border-b border-surface-1 text-subtext-0 hover:text-text hover:bg-surface-0 transition-colors"
      >
        {collapsed ? (
          <ChevronRight className="w-5 h-5" />
        ) : (
          <ChevronLeft className="w-5 h-5" />
        )}
      </button>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto py-4">
        {NAV_ITEMS.map((item) => (
          <div key={item.label} className="mb-1">
            {item.path ? (
              // Direct link (e.g., Dashboard)
              <Link
                to={item.path}
                className={cn(
                  'flex items-center gap-3 px-4 py-2.5 mx-2 rounded-md transition-colors',
                  isActive(item.path)
                    ? 'bg-surface-0 text-primary'
                    : 'text-subtext-1 hover:bg-surface-0 hover:text-text'
                )}
              >
                <item.icon className="w-5 h-5 flex-shrink-0" />
                {!collapsed && (
                  <span className="text-sm font-medium">{item.label}</span>
                )}
              </Link>
            ) : (
              // Category with children
              <>
                <button
                  onClick={() => toggleCategory(item.label)}
                  className={cn(
                    'w-full flex items-center gap-3 px-4 py-2.5 mx-2 rounded-md transition-colors',
                    'text-subtext-1 hover:bg-surface-0 hover:text-text',
                    collapsed ? 'justify-center' : ''
                  )}
                  style={{ width: collapsed ? 'auto' : 'calc(100% - 16px)' }}
                >
                  <item.icon className="w-5 h-5 flex-shrink-0" />
                  {!collapsed && (
                    <>
                      <span className="text-sm font-medium flex-1 text-left">
                        {item.label}
                      </span>
                      <ChevronRight
                        className={cn(
                          'w-4 h-4 transition-transform',
                          expandedCategories.has(item.label) ? 'rotate-90' : ''
                        )}
                      />
                    </>
                  )}
                </button>

                {/* Children */}
                {!collapsed && expandedCategories.has(item.label) && (
                  <div className="mt-1 ml-4 space-y-1">
                    {item.children?.map((child) => (
                      <Link
                        key={child.path}
                        to={child.path!}
                        className={cn(
                          'flex items-center gap-3 px-4 py-2 mx-2 rounded-md text-sm transition-colors',
                          isActive(child.path!)
                            ? 'bg-surface-0 text-primary font-medium'
                            : 'text-subtext-0 hover:bg-surface-0 hover:text-text'
                        )}
                      >
                        {child.icon && <child.icon className="w-4 h-4 flex-shrink-0" />}
                        {child.label}
                      </Link>
                    ))}
                  </div>
                )}
              </>
            )}
          </div>
        ))}
      </nav>
    </aside>
  );
}
