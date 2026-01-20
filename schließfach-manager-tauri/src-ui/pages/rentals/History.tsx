import { useState } from 'react';
import { History, Search, Filter, Download } from 'lucide-react';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { Badge } from '@/components/ui/Badge';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/Card';
import {
  Table,
  TableHeader,
  TableBody,
  TableRow,
  TableHead,
  TableCell,
} from '@/components/ui/Table';
import { formatDate } from '@/lib/utils';

// Mock historical rental data
const mockRentalHistory = [
  {
    id: 1,
    locker_number: 'A-001',
    renter_name: 'Max Mustermann',
    start_date: '2023-01-15',
    end_date: '2024-01-15',
    status: 'returned',
    deposit_returned: true,
  },
  {
    id: 2,
    locker_number: 'B-010',
    renter_name: 'Anna Schmidt',
    start_date: '2023-03-01',
    end_date: '2024-03-01',
    status: 'returned',
    deposit_returned: true,
  },
  {
    id: 3,
    locker_number: 'C-005',
    renter_name: 'Klaus Weber',
    start_date: '2023-06-15',
    end_date: '2024-06-15',
    status: 'cancelled',
    deposit_returned: true,
  },
  {
    id: 4,
    locker_number: 'A-015',
    renter_name: 'Lisa Braun',
    start_date: '2023-09-01',
    end_date: '2024-09-01',
    status: 'returned',
    deposit_returned: false,
  },
  {
    id: 5,
    locker_number: 'B-020',
    renter_name: 'Thomas Richter',
    start_date: '2022-05-01',
    end_date: '2023-05-01',
    status: 'returned',
    deposit_returned: true,
  },
];

const statusOptions = [
  { value: 'all', label: 'Alle Status' },
  { value: 'returned', label: 'Zurückgegeben' },
  { value: 'cancelled', label: 'Storniert' },
];

const yearOptions = [
  { value: 'all', label: 'Alle Jahre' },
  { value: '2024', label: '2024' },
  { value: '2023', label: '2023' },
  { value: '2022', label: '2022' },
];

function getStatusBadge(status: string) {
  switch (status) {
    case 'returned':
      return <Badge variant="success">Zurückgegeben</Badge>;
    case 'cancelled':
      return <Badge variant="warning">Storniert</Badge>;
    default:
      return <Badge>{status}</Badge>;
  }
}

export function RentalHistory() {
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [yearFilter, setYearFilter] = useState('all');

  const filteredHistory = mockRentalHistory.filter((rental) => {
    const matchesSearch =
      rental.locker_number.toLowerCase().includes(searchTerm.toLowerCase()) ||
      rental.renter_name.toLowerCase().includes(searchTerm.toLowerCase());

    const matchesStatus =
      statusFilter === 'all' || rental.status === statusFilter;

    const matchesYear =
      yearFilter === 'all' ||
      rental.end_date.startsWith(yearFilter);

    return matchesSearch && matchesStatus && matchesYear;
  });

  return (
    <div className="space-y-6 animate-fade-in">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-lg bg-accent/20 flex items-center justify-center">
            <History className="w-6 h-6 text-accent" />
          </div>
          <div>
            <h1 className="text-3xl font-bold text-text">Verleih-Verlauf</h1>
            <p className="text-subtext-0 mt-1">
              Historische Übersicht aller abgeschlossenen Verleihe
            </p>
          </div>
        </div>
        <Button variant="outline">
          <Download className="w-4 h-4" />
          Exportieren
        </Button>
      </div>

      {/* Filters */}
      <div className="flex flex-wrap items-center gap-4">
        <div className="flex-1 min-w-[200px] max-w-md">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-subtext-0" />
            <Input
              placeholder="Suchen nach Schließfach oder Name..."
              className="pl-10"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>
        </div>
        <Select
          value={statusFilter}
          onChange={(e) => setStatusFilter(e.target.value)}
          options={statusOptions}
        />
        <Select
          value={yearFilter}
          onChange={(e) => setYearFilter(e.target.value)}
          options={yearOptions}
        />
      </div>

      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-text">
              {mockRentalHistory.length}
            </div>
            <p className="text-sm text-subtext-0">Verleihe gesamt</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-success">
              {mockRentalHistory.filter((r) => r.status === 'returned').length}
            </div>
            <p className="text-sm text-subtext-0">Zurückgegeben</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-warning">
              {mockRentalHistory.filter((r) => r.status === 'cancelled').length}
            </div>
            <p className="text-sm text-subtext-0">Storniert</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-text">
              {mockRentalHistory.filter((r) => r.deposit_returned).length}
            </div>
            <p className="text-sm text-subtext-0">Pfand erstattet</p>
          </CardContent>
        </Card>
      </div>

      {/* Table */}
      <Card>
        <CardHeader>
          <CardTitle>Verlauf ({filteredHistory.length} Einträge)</CardTitle>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Schließfach</TableHead>
                <TableHead>Mieter</TableHead>
                <TableHead>Zeitraum</TableHead>
                <TableHead>Status</TableHead>
                <TableHead>Pfand</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {filteredHistory.map((rental) => (
                <TableRow key={rental.id}>
                  <TableCell className="font-medium">{rental.locker_number}</TableCell>
                  <TableCell>{rental.renter_name}</TableCell>
                  <TableCell>
                    <div className="text-sm">
                      <div>{formatDate(rental.start_date)}</div>
                      <div className="text-subtext-0">bis {formatDate(rental.end_date)}</div>
                    </div>
                  </TableCell>
                  <TableCell>{getStatusBadge(rental.status)}</TableCell>
                  <TableCell>
                    {rental.deposit_returned ? (
                      <Badge variant="success">Erstattet</Badge>
                    ) : (
                      <Badge variant="error">Offen</Badge>
                    )}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>

          {filteredHistory.length === 0 && (
            <div className="text-center py-8 text-subtext-0">
              Keine Einträge gefunden.
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
