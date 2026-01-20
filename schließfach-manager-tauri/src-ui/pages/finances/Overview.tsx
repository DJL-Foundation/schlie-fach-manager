import { BarChart3, TrendingUp, DollarSign, PiggyBank } from 'lucide-react';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/Card';
import { RevenueChart } from '@/components/charts/RevenueChart';
import { formatCurrency, formatPercent } from '@/lib/utils';

// Mock financial data
const mockFinanceData = {
  totalRevenue: 125000,
  monthlyRevenue: 12500,
  pendingPayments: 15000,
  depositHeld: 87000,
  revenueByMonth: [
    { month: 'Jul', revenue: 10000 },
    { month: 'Aug', revenue: 11500 },
    { month: 'Sep', revenue: 9800 },
    { month: 'Okt', revenue: 12500 },
    { month: 'Nov', revenue: 13200 },
    { month: 'Dez', revenue: 12500 },
  ],
  paymentBreakdown: {
    yearly_fee: 85000,
    deposit: 30000,
    extension: 8000,
    other: 2000,
  },
};

export function FinancesOverview() {
  return (
    <div className="space-y-6 animate-fade-in">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold text-text">Finanz-Übersicht</h1>
        <p className="text-subtext-0 mt-1">
          Einnahmen, Zahlungen und Pfandverwaltung
        </p>
      </div>

      {/* Key Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-subtext-0">Jahresumsatz</p>
                <p className="text-2xl font-bold text-text mt-1">
                  {formatCurrency(mockFinanceData.totalRevenue)}
                </p>
              </div>
              <div className="w-12 h-12 rounded-lg bg-success/20 flex items-center justify-center">
                <TrendingUp className="w-6 h-6 text-success" />
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-subtext-0">Monatsumsatz</p>
                <p className="text-2xl font-bold text-text mt-1">
                  {formatCurrency(mockFinanceData.monthlyRevenue)}
                </p>
              </div>
              <div className="w-12 h-12 rounded-lg bg-primary/20 flex items-center justify-center">
                <BarChart3 className="w-6 h-6 text-primary" />
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-subtext-0">Offene Zahlungen</p>
                <p className="text-2xl font-bold text-text mt-1">
                  {formatCurrency(mockFinanceData.pendingPayments)}
                </p>
              </div>
              <div className="w-12 h-12 rounded-lg bg-warning/20 flex items-center justify-center">
                <DollarSign className="w-6 h-6 text-warning" />
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="pt-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-subtext-0">Pfand (gehalten)</p>
                <p className="text-2xl font-bold text-text mt-1">
                  {formatCurrency(mockFinanceData.depositHeld)}
                </p>
              </div>
              <div className="w-12 h-12 rounded-lg bg-secondary/20 flex items-center justify-center">
                <PiggyBank className="w-6 h-6 text-secondary" />
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Revenue Chart */}
      <Card>
        <CardHeader>
          <CardTitle>Umsatz (letzte 6 Monate)</CardTitle>
        </CardHeader>
        <CardContent>
          <RevenueChart data={mockFinanceData.revenueByMonth} />
        </CardContent>
      </Card>

      {/* Payment Breakdown */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card>
          <CardHeader>
            <CardTitle>Einnahmen nach Kategorie</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {Object.entries(mockFinanceData.paymentBreakdown).map(([type, amount]) => {
                const total = Object.values(mockFinanceData.paymentBreakdown).reduce((a, b) => a + b, 0);
                const percent = (amount / total) * 100;
                const labels: Record<string, string> = {
                  yearly_fee: 'Jahresgebühren',
                  deposit: 'Pfandeinnahmen',
                  extension: 'Verlängerungen',
                  other: 'Sonstiges',
                };
                return (
                  <div key={type}>
                    <div className="flex items-center justify-between text-sm mb-2">
                      <span className="text-subtext-1 font-medium">{labels[type] || type}</span>
                      <span className="text-text">
                        {formatCurrency(amount)} ({formatPercent(percent)})
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

        <Card>
          <CardHeader>
            <CardTitle>Schnellübersicht</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div className="p-4 rounded-lg bg-base border border-surface-1">
                <div className="flex items-center justify-between">
                  <span className="text-subtext-0">Durchschnittliche Mietdauer</span>
                  <span className="font-medium text-text">8.5 Monate</span>
                </div>
              </div>
              <div className="p-4 rounded-lg bg-base border border-surface-1">
                <div className="flex items-center justify-between">
                  <span className="text-subtext-0">Durchschnittlicher Umsatz/Mieter</span>
                  <span className="font-medium text-text">{formatCurrency(1437)}</span>
                </div>
              </div>
              <div className="p-4 rounded-lg bg-base border border-surface-1">
                <div className="flex items-center justify-between">
                  <span className="text-subtext-0">Zahlungsquote</span>
                  <span className="font-medium text-success">94.5%</span>
                </div>
              </div>
              <div className="p-4 rounded-lg bg-base border border-surface-1">
                <div className="flex items-center justify-between">
                  <span className="text-subtext-0">Ausstehende Rückerstattungen</span>
                  <span className="font-medium text-text">3 Stück</span>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
