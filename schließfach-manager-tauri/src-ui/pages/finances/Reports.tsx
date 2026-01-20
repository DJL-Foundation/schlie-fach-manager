import { useState } from 'react';
import { FileText, Download } from 'lucide-react';
import { Button } from '@/components/ui/Button';
import { Select } from '@/components/ui/Select';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/Card';
import { formatCurrency } from '@/lib/utils';

// Mock report data
const mockReportData = {
  summary: {
    totalIncome: 125000,
    totalExpenses: 0,
    netProfit: 125000,
    occupancyRate: 58.0,
    averageRentalDuration: 8.5,
  },
  monthly: [
    { month: 'Januar', income: 10500, newRentals: 5, returns: 2 },
    { month: 'Februar', income: 9800, newRentals: 3, returns: 4 },
    { month: 'März', income: 11200, newRentals: 6, returns: 3 },
    { month: 'April', income: 10000, newRentals: 4, returns: 2 },
    { month: 'Mai', income: 10800, newRentals: 5, returns: 3 },
    { month: 'Juni', income: 12000, newRentals: 7, returns: 2 },
    { month: 'Juli', income: 11500, newRentals: 5, returns: 4 },
    { month: 'August', income: 9500, newRentals: 3, returns: 5 },
    { month: 'September', income: 10200, newRentals: 4, returns: 3 },
    { month: 'Oktober', income: 12500, newRentals: 6, returns: 2 },
    { month: 'November', income: 13200, newRentals: 8, returns: 1 },
    { month: 'Dezember', income: 3800, newRentals: 2, returns: 1 },
  ],
};

const yearOptions = [
  { value: '2024', label: '2024' },
  { value: '2023', label: '2023' },
  { value: '2022', label: '2022' },
];

const reportTypeOptions = [
  { value: 'annual', label: 'Jahresbericht' },
  { value: 'quarterly', label: 'Quartalsbericht' },
  { value: 'monthly', label: 'Monatsbericht' },
];

export function Reports() {
  const [selectedYear, setSelectedYear] = useState('2024');
  const [reportType, setReportType] = useState('annual');

  const handleExport = (format: string) => {
    // TODO: Implement actual export
    console.log(`Exporting ${reportType} report for ${selectedYear} as ${format}`);
  };

  return (
    <div className="space-y-6 animate-fade-in">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-lg bg-primary/20 flex items-center justify-center">
            <FileText className="w-6 h-6 text-primary" />
          </div>
          <div>
            <h1 className="text-3xl font-bold text-text">Berichte</h1>
            <p className="text-subtext-0 mt-1">
              Finanzberichte und Statistiken generieren
            </p>
          </div>
        </div>
      </div>

      {/* Filters */}
      <Card>
        <CardContent className="pt-6">
          <div className="flex flex-wrap items-end gap-4">
            <Select
              label="Jahr"
              value={selectedYear}
              onChange={(e) => setSelectedYear(e.target.value)}
              options={yearOptions}
            />
            <Select
              label="Berichtstyp"
              value={reportType}
              onChange={(e) => setReportType(e.target.value)}
              options={reportTypeOptions}
            />
            <div className="flex gap-2">
              <Button variant="outline" onClick={() => handleExport('pdf')}>
                <Download className="w-4 h-4" />
                Als PDF
              </Button>
              <Button variant="outline" onClick={() => handleExport('csv')}>
                <Download className="w-4 h-4" />
                Als CSV
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
        <Card>
          <CardContent className="pt-6">
            <p className="text-sm text-subtext-0">Gesamteinnahmen</p>
            <p className="text-2xl font-bold text-success mt-1">
              {formatCurrency(mockReportData.summary.totalIncome)}
            </p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <p className="text-sm text-subtext-0">Ausgaben</p>
            <p className="text-2xl font-bold text-text mt-1">
              {formatCurrency(mockReportData.summary.totalExpenses)}
            </p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <p className="text-sm text-subtext-0">Nettogewinn</p>
            <p className="text-2xl font-bold text-success mt-1">
              {formatCurrency(mockReportData.summary.netProfit)}
            </p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <p className="text-sm text-subtext-0">Belegungsrate</p>
            <p className="text-2xl font-bold text-text mt-1">
              {mockReportData.summary.occupancyRate}%
            </p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <p className="text-sm text-subtext-0">Ø Mietdauer</p>
            <p className="text-2xl font-bold text-text mt-1">
              {mockReportData.summary.averageRentalDuration} Mon.
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Monthly Breakdown */}
      <Card>
        <CardHeader>
          <CardTitle>Monatliche Aufschlüsselung - {selectedYear}</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-surface-1">
                  <th className="text-left py-3 px-4 text-sm font-medium text-subtext-1">Monat</th>
                  <th className="text-right py-3 px-4 text-sm font-medium text-subtext-1">Einnahmen</th>
                  <th className="text-right py-3 px-4 text-sm font-medium text-subtext-1">Neue Verleihe</th>
                  <th className="text-right py-3 px-4 text-sm font-medium text-subtext-1">Rückgaben</th>
                  <th className="text-right py-3 px-4 text-sm font-medium text-subtext-1">Netto</th>
                </tr>
              </thead>
              <tbody>
                {mockReportData.monthly.map((month, index) => (
                  <tr
                    key={month.month}
                    className={index < mockReportData.monthly.length - 1 ? 'border-b border-surface-1' : ''}
                  >
                    <td className="py-3 px-4 text-text">{month.month}</td>
                    <td className="py-3 px-4 text-right text-success font-medium">
                      {formatCurrency(month.income)}
                    </td>
                    <td className="py-3 px-4 text-right text-text">{month.newRentals}</td>
                    <td className="py-3 px-4 text-right text-subtext-0">{month.returns}</td>
                    <td className="py-3 px-4 text-right text-text font-medium">
                      +{month.newRentals - month.returns}
                    </td>
                  </tr>
                ))}
              </tbody>
              <tfoot>
                <tr className="border-t-2 border-surface-1 bg-surface-0/50">
                  <td className="py-3 px-4 font-bold text-text">Gesamt</td>
                  <td className="py-3 px-4 text-right font-bold text-success">
                    {formatCurrency(mockReportData.monthly.reduce((sum, m) => sum + m.income, 0))}
                  </td>
                  <td className="py-3 px-4 text-right font-bold text-text">
                    {mockReportData.monthly.reduce((sum, m) => sum + m.newRentals, 0)}
                  </td>
                  <td className="py-3 px-4 text-right font-bold text-text">
                    {mockReportData.monthly.reduce((sum, m) => sum + m.returns, 0)}
                  </td>
                  <td className="py-3 px-4 text-right font-bold text-text">
                    +{mockReportData.monthly.reduce((sum, m) => sum + (m.newRentals - m.returns), 0)}
                  </td>
                </tr>
              </tfoot>
            </table>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
