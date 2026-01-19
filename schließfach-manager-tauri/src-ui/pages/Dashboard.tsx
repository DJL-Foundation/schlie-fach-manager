import { Archive, DollarSign, AlertTriangle, Clock } from 'lucide-react';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/Card';
import { OccupancyChart } from '@/components/charts/OccupancyChart';
import { formatCurrency, formatPercent } from '@/lib/utils';
import type { DashboardData, OccupancyHistoryPoint } from '@/types';

// Mock data - will be replaced with actual Tauri commands
const mockDashboardData: DashboardData = {
  total_lockers: 150,
  occupied_lockers: 87,
  occupancy_percent: 58.0,
  by_size: {
    S: { total: 50, occupied: 30 },
    M: { total: 60, occupied: 35 },
    L: { total: 30, occupied: 17 },
    XL: { total: 10, occupied: 5 },
  },
  by_location: {
    'Gebäude A': { total: 80, occupied: 48 },
    'Gebäude B': { total: 50, occupied: 29 },
    'Gebäude C': { total: 20, occupied: 10 },
  },
  overdue_returns: 5,
  expiring_soon: 12,
  damaged_lockers: 3,
  pending_payments_cents: 15000,
  revenue_30d_cents: 125000,
  occupancy_history: generateMockHistory(),
};

function generateMockHistory(): OccupancyHistoryPoint[] {
  const data: OccupancyHistoryPoint[] = [];
  const now = new Date();
  for (let i = 29; i >= 0; i--) {
    const date = new Date(now);
    date.setDate(date.getDate() - i);
    data.push({
      date: date.toISOString().split('T')[0],
      percent: 50 + Math.random() * 20,
    });
  }
  return data;
}

export function Dashboard() {
  const data = mockDashboardData;

  return (
    <div className="space-y-6 animate-fade-in">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold text-text">Dashboard</h1>
        <p className="text-subtext-0 mt-1">
          Übersicht über alle Schließfächer und Verleihvorgänge
        </p>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {/* Belegung */}
        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-subtext-0">Belegung</p>
                <p className="text-2xl font-bold text-text mt-1">
                  {formatPercent(data.occupancy_percent)}
                </p>
                <p className="text-xs text-subtext-0 mt-1">
                  {data.occupied_lockers} / {data.total_lockers}
                </p>
              </div>
              <div className="w-12 h-12 rounded-lg bg-primary/20 flex items-center justify-center">
                <Archive className="w-6 h-6 text-primary" />
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Umsatz (30 Tage) */}
        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-subtext-0">Umsatz (30 Tage)</p>
                <p className="text-2xl font-bold text-text mt-1">
                  {formatCurrency(data.revenue_30d_cents)}
                </p>
              </div>
              <div className="w-12 h-12 rounded-lg bg-success/20 flex items-center justify-center">
                <DollarSign className="w-6 h-6 text-success" />
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Überfällig */}
        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-subtext-0">Überfällig</p>
                <p className="text-2xl font-bold text-text mt-1">
                  {data.overdue_returns}
                </p>
              </div>
              <div className="w-12 h-12 rounded-lg bg-error/20 flex items-center justify-center">
                <AlertTriangle className="w-6 h-6 text-error" />
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Auslaufend */}
        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-subtext-0">Auslaufend (7 Tage)</p>
                <p className="text-2xl font-bold text-text mt-1">
                  {data.expiring_soon}
                </p>
              </div>
              <div className="w-12 h-12 rounded-lg bg-warning/20 flex items-center justify-center">
                <Clock className="w-6 h-6 text-warning" />
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Charts Row */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Occupancy Trend */}
        <Card>
          <CardHeader>
            <CardTitle>Belegungstrend (30 Tage)</CardTitle>
          </CardHeader>
          <CardContent>
            <OccupancyChart data={data.occupancy_history} />
          </CardContent>
        </Card>

        {/* By Size */}
        <Card>
          <CardHeader>
            <CardTitle>Belegung nach Größe</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {Object.entries(data.by_size).map(([size, stats]) => {
                const percent = stats.total > 0 ? (stats.occupied / stats.total) * 100 : 0;
                return (
                  <div key={size}>
                    <div className="flex items-center justify-between text-sm mb-2">
                      <span className="text-subtext-1 font-medium">Größe {size}</span>
                      <span className="text-text">
                        {stats.occupied} / {stats.total} ({formatPercent(percent)})
                      </span>
                    </div>
                    <div className="w-full h-2 bg-surface-1 rounded-full overflow-hidden">
                      <div
                        className="h-full bg-primary transition-all duration-500"
                        style={{ width: `${percent}%` }}
                      />
                    </div>
                  </div>
                );
              })}
            </div>
          </CardContent>
        </Card>
      </div>

      {/* By Location */}
      <Card>
        <CardHeader>
          <CardTitle>Belegung nach Standort</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {Object.entries(data.by_location).map(([location, stats]) => {
              const percent = stats.total > 0 ? (stats.occupied / stats.total) * 100 : 0;
              return (
                <div key={location} className="p-4 rounded-lg bg-base border border-surface-1">
                  <h4 className="font-medium text-text mb-3">{location}</h4>
                  <div className="flex items-center justify-between text-sm mb-2">
                    <span className="text-subtext-0">Belegung</span>
                    <span className="text-text font-medium">
                      {stats.occupied} / {stats.total}
                    </span>
                  </div>
                  <div className="w-full h-3 bg-surface-1 rounded-full overflow-hidden">
                    <div
                      className="h-full bg-secondary transition-all duration-500"
                      style={{ width: `${percent}%` }}
                    />
                  </div>
                  <p className="text-center text-sm text-subtext-0 mt-2">
                    {formatPercent(percent)}
                  </p>
                </div>
              );
            })}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
