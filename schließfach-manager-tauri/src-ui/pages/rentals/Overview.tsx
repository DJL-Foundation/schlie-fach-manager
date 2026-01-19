import { Link } from 'react-router-dom';
import { Plus, Search, Filter } from 'lucide-react';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Badge } from '@/components/ui/Badge';
import {
  Table,
  TableHeader,
  TableBody,
  TableRow,
  TableHead,
  TableCell,
} from '@/components/ui/Table';
import { formatDate, isOverdue, isExpiringSoon } from '@/lib/utils';
import type { Rental } from '@/types';

// Mock data
const mockRentals: Rental[] = [
  {
    id: 1,
    locker_id: 1,
    locker_number: 'A-001',
    renter_name: 'Max Mustermann',
    renter_email: 'max@example.com',
    renter_phone: '+49 123 456789',
    start_date: '2024-01-15',
    end_date: '2025-01-15',
    deposit_paid: true,
    deposit_returned: false,
    notes: null,
    created_at: '2024-01-15T10:00:00Z',
  },
  {
    id: 2,
    locker_id: 2,
    locker_number: 'A-002',
    renter_name: 'Erika Musterfrau',
    renter_email: 'erika@example.com',
    renter_phone: null,
    start_date: '2024-06-01',
    end_date: '2024-12-01',
    deposit_paid: true,
    deposit_returned: false,
    notes: null,
    created_at: '2024-06-01T09:30:00Z',
  },
  {
    id: 3,
    locker_id: 5,
    locker_number: 'B-003',
    renter_name: 'Hans Schmidt',
    renter_email: null,
    renter_phone: '+49 987 654321',
    start_date: '2024-03-01',
    end_date: '2025-03-01',
    deposit_paid: false,
    deposit_returned: false,
    notes: 'Pfand ausstehend',
    created_at: '2024-03-01T14:00:00Z',
  },
];

function getStatusBadge(rental: Rental) {
  if (isOverdue(rental.end_date)) {
    return <Badge variant="error">Überfällig</Badge>;
  }
  if (isExpiringSoon(rental.end_date)) {
    return <Badge variant="warning">Läuft bald ab</Badge>;
  }
  return <Badge variant="success">Aktiv</Badge>;
}

export function RentalsOverview() {
  return (
    <div className="space-y-6 animate-fade-in">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-text">Verleih-Übersicht</h1>
          <p className="text-subtext-0 mt-1">
            Alle aktiven und vergangenen Verleihe
          </p>
        </div>
        <Link to="/rentals/new">
          <Button>
            <Plus className="w-4 h-4" />
            Neu verleihen
          </Button>
        </Link>
      </div>

      {/* Filters */}
      <div className="flex items-center gap-4">
        <div className="flex-1 max-w-md">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-subtext-0" />
            <Input
              placeholder="Suchen nach Name, Schließfach..."
              className="pl-10"
            />
          </div>
        </div>
        <Button variant="outline">
          <Filter className="w-4 h-4" />
          Filter
        </Button>
      </div>

      {/* Table */}
      <div className="rounded-lg border border-surface-1 overflow-hidden">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Schließfach</TableHead>
              <TableHead>Mieter</TableHead>
              <TableHead>Kontakt</TableHead>
              <TableHead>Zeitraum</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Pfand</TableHead>
              <TableHead className="text-right">Aktionen</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {mockRentals.map((rental) => (
              <TableRow key={rental.id}>
                <TableCell className="font-medium">
                  {rental.locker_number}
                </TableCell>
                <TableCell>{rental.renter_name}</TableCell>
                <TableCell>
                  <div className="text-sm">
                    {rental.renter_email && (
                      <div className="text-subtext-0">{rental.renter_email}</div>
                    )}
                    {rental.renter_phone && (
                      <div className="text-subtext-0">{rental.renter_phone}</div>
                    )}
                  </div>
                </TableCell>
                <TableCell>
                  <div className="text-sm">
                    <div>{formatDate(rental.start_date)}</div>
                    <div className="text-subtext-0">
                      bis {formatDate(rental.end_date)}
                    </div>
                  </div>
                </TableCell>
                <TableCell>{getStatusBadge(rental)}</TableCell>
                <TableCell>
                  {rental.deposit_paid ? (
                    <Badge variant="success">Bezahlt</Badge>
                  ) : (
                    <Badge variant="error">Ausstehend</Badge>
                  )}
                </TableCell>
                <TableCell className="text-right">
                  <div className="flex items-center justify-end gap-2">
                    <Button variant="ghost" size="sm">
                      Details
                    </Button>
                    <Button variant="outline" size="sm">
                      Verlängern
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
