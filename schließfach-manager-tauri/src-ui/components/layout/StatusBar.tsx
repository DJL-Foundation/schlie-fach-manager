import { useState, useEffect } from 'react';
import { Database, Clock, TrendingUp } from 'lucide-react';
import { format } from 'date-fns';
import { de } from 'date-fns/locale';

interface StatusBarData {
  db_status: 'connected' | 'error';
  total_lockers: number;
  occupied_lockers: number;
  occupancy_percent: number;
}

// Mock data - will be replaced with actual Tauri commands
const mockStatusData: StatusBarData = {
  db_status: 'connected',
  total_lockers: 150,
  occupied_lockers: 87,
  occupancy_percent: 58.0,
};

export function StatusBar() {
  const [time, setTime] = useState(new Date());
  const [data] = useState<StatusBarData>(mockStatusData);

  useEffect(() => {
    const interval = setInterval(() => setTime(new Date()), 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <footer className="h-8 bg-crust border-t border-surface-1 flex items-center px-4 text-xs text-subtext-0 flex-shrink-0">
      {/* DB Status */}
      <div className="flex items-center gap-2">
        <Database className="w-3.5 h-3.5" />
        <span>
          DB:{' '}
          {data.db_status === 'connected' ? (
            <span className="text-success">Verbunden</span>
          ) : (
            <span className="text-error">Fehler</span>
          )}
        </span>
      </div>

      <div className="mx-3 h-4 w-px bg-surface-1" />

      {/* Occupancy */}
      <div className="flex items-center gap-2">
        <TrendingUp className="w-3.5 h-3.5" />
        <span>
          Belegung: {data.occupied_lockers} / {data.total_lockers}
          {' '}
          ({data.occupancy_percent.toFixed(1)}%)
        </span>
      </div>

      <div className="mx-3 h-4 w-px bg-surface-1" />

      {/* Clock */}
      <div className="flex items-center gap-2">
        <Clock className="w-3.5 h-3.5" />
        <span>{format(time, 'dd.MM.yyyy HH:mm:ss', { locale: de })}</span>
      </div>

      {/* Spacer */}
      <div className="flex-1" />

      {/* Version */}
      <span className="text-overlay-1">v2.1-Tauri</span>
    </footer>
  );
}
