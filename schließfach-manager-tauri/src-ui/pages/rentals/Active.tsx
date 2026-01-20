import { AlertTriangle } from 'lucide-react';
import { Button } from '@/components/ui/Button';
import { Badge } from '@/components/ui/Badge';
import {
  Table,
  TableHeader,
  TableBody,
  TableRow,
  TableHead,
  TableCell,
} from '@/components/ui/Table';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/Card';
import { formatDate } from '@/lib/utils';

// Mock data for active rentals
const mockActiveRentals = [
  {
    id: 1,
    locker_number: 'A-001',
    renter_name: 'Max Mustermann',
    renter_email: 'max@example.com',
    start_date: '2024-01-15',
    end_date: '2025-01-15',
    deposit_paid: true,
    days_remaining: 180,
  },
  {
    id: 2,
    locker_number: 'A-002',
    renter_name: 'Erika Musterfrau',
    renter_email: 'erika@example.com',
    start_date: '2024-06-01',
    end_date: '2025-06-01',
    deposit_paid: true,
    days_remaining: 320,
  },
  {
    id: 3,
    locker_number: 'B-005',
    renter_name: 'Peter Müller',
    renter_email: 'peter@example.com',
    start_date: '2024-10-01',
    end_date: '2025-02-01',
    deposit_paid: true,
    days_remaining: 5,
  },
];

export function ActiveRentals() {
  return (
    <div className="space-y-6 animate-fade-in">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold text-text">Aktive Verleihe</h1>
        <p className="text-subtext-0 mt-1">
          Alle derzeit aktiven Schließfach-Verleihungen
        </p>
      </div>

      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-text">
              {mockActiveRentals.length}
            </div>
            <p className="text-sm text-subtext-0">Aktive Verleihe gesamt</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-warning">
              {mockActiveRentals.filter((r) => r.days_remaining <= 7).length}
            </div>
            <p className="text-sm text-subtext-0">Laufen in 7 Tagen ab</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-success">
              {mockActiveRentals.filter((r) => r.deposit_paid).length}
            </div>
            <p className="text-sm text-subtext-0">Pfand bezahlt</p>
          </CardContent>
        </Card>
      </div>

      {/* Table */}
      <div className="rounded-lg border border-surface-1 overflow-hidden">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Schließfach</TableHead>
              <TableHead>Mieter</TableHead>
              <TableHead>E-Mail</TableHead>
              <TableHead>Start</TableHead>
              <TableHead>Ende</TableHead>
              <TableHead>Status</TableHead>
              <TableHead className="text-right">Aktionen</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {mockActiveRentals.map((rental) => (
              <TableRow key={rental.id}>
                <TableCell className="font-medium">
                  {rental.locker_number}
                </TableCell>
                <TableCell>{rental.renter_name}</TableCell>
                <TableCell className="text-subtext-0">
                  {rental.renter_email}
                </TableCell>
                <TableCell>{formatDate(rental.start_date)}</TableCell>
                <TableCell>
                  <div className="flex items-center gap-2">
                    {formatDate(rental.end_date)}
                    {rental.days_remaining <= 7 && (
                      <AlertTriangle className="w-4 h-4 text-warning" />
                    )}
                  </div>
                </TableCell>
                <TableCell>
                  {rental.days_remaining <= 7 ? (
                    <Badge variant="warning">
                      {rental.days_remaining} Tage übrig
                    </Badge>
                  ) : (
                    <Badge variant="success">Aktiv</Badge>
                  )}
                </TableCell>
                <TableCell className="text-right">
                  <div className="flex items-center justify-end gap-2">
                    <Button variant="outline" size="sm">
                      Verlängern
                    </Button>
                    <Button variant="ghost" size="sm">
                      Rückgabe
                    </Button>
                  </div>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
    </div>
  );
}
