import { useState } from 'react';
import { Search, RotateCcw, Check } from 'lucide-react';
import { toast } from 'sonner';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Badge } from '@/components/ui/Badge';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/Card';
import { Modal } from '@/components/ui/Modal';
import {
  Table,
  TableHeader,
  TableBody,
  TableRow,
  TableHead,
  TableCell,
} from '@/components/ui/Table';
import { formatDate, isOverdue } from '@/lib/utils';

// Mock data for active rentals
const mockActiveRentals = [
  {
    id: 1,
    locker_number: 'A-001',
    renter_name: 'Max Mustermann',
    renter_email: 'max@example.com',
    end_date: '2025-01-15',
    deposit_paid: true,
    deposit_amount_cents: 1000,
  },
  {
    id: 2,
    locker_number: 'A-002',
    renter_name: 'Erika Musterfrau',
    renter_email: 'erika@example.com',
    end_date: '2024-10-01',
    deposit_paid: true,
    deposit_amount_cents: 1000,
  },
  {
    id: 3,
    locker_number: 'B-005',
    renter_name: 'Peter Müller',
    renter_email: null,
    end_date: '2025-02-01',
    deposit_paid: false,
    deposit_amount_cents: 1000,
  },
];

export function ReturnRental() {
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedRental, setSelectedRental] = useState<typeof mockActiveRentals[0] | null>(null);
  const [returnDeposit, setReturnDeposit] = useState(true);
  const [isReturning, setIsReturning] = useState(false);

  const filteredRentals = mockActiveRentals.filter(
    (rental) =>
      rental.locker_number.toLowerCase().includes(searchTerm.toLowerCase()) ||
      rental.renter_name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const handleReturn = async () => {
    if (!selectedRental) return;

    try {
      setIsReturning(true);
      // TODO: Call Tauri command to return rental
      await new Promise((resolve) => setTimeout(resolve, 1000));
      toast.success('Rückgabe erfolgreich verarbeitet');
      setSelectedRental(null);
    } catch (error) {
      toast.error(`Fehler: ${error}`);
    } finally {
      setIsReturning(false);
    }
  };

  return (
    <div className="space-y-6 animate-fade-in">
      {/* Page Header */}
      <div className="flex items-center gap-4">
        <div className="w-12 h-12 rounded-lg bg-secondary/20 flex items-center justify-center">
          <RotateCcw className="w-6 h-6 text-secondary" />
        </div>
        <div>
          <h1 className="text-3xl font-bold text-text">Rückgabe</h1>
          <p className="text-subtext-0 mt-1">
            Schließfach-Rückgabe durchführen und Pfand erstatten
          </p>
        </div>
      </div>

      {/* Search */}
      <div className="flex items-center gap-4">
        <div className="flex-1 max-w-md">
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
      </div>

      {/* Table */}
      <Card>
        <CardHeader>
          <CardTitle>Aktive Verleihe zur Rückgabe</CardTitle>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Schließfach</TableHead>
                <TableHead>Mieter</TableHead>
                <TableHead>Enddatum</TableHead>
                <TableHead>Status</TableHead>
                <TableHead>Pfand</TableHead>
                <TableHead className="text-right">Aktion</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {filteredRentals.map((rental) => (
                <TableRow key={rental.id}>
                  <TableCell className="font-medium">{rental.locker_number}</TableCell>
                  <TableCell>{rental.renter_name}</TableCell>
                  <TableCell>{formatDate(rental.end_date)}</TableCell>
                  <TableCell>
                    {isOverdue(rental.end_date) ? (
                      <Badge variant="error">Überfällig</Badge>
                    ) : (
                      <Badge variant="success">Aktiv</Badge>
                    )}
                  </TableCell>
                  <TableCell>
                    {rental.deposit_paid ? (
                      <Badge variant="success">Bezahlt</Badge>
                    ) : (
                      <Badge variant="error">Ausstehend</Badge>
                    )}
                  </TableCell>
                  <TableCell className="text-right">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => setSelectedRental(rental)}
                    >
                      <RotateCcw className="w-4 h-4" />
                      Rückgabe
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      {/* Return Modal */}
      <Modal
        open={selectedRental !== null}
        onClose={() => setSelectedRental(null)}
        title="Rückgabe durchführen"
        size="md"
        footer={
          <>
            <Button variant="outline" onClick={() => setSelectedRental(null)}>
              Abbrechen
            </Button>
            <Button onClick={handleReturn} isLoading={isReturning}>
              <Check className="w-4 h-4" />
              Rückgabe bestätigen
            </Button>
          </>
        }
      >
        {selectedRental && (
          <div className="space-y-4">
            <div className="p-4 rounded-lg bg-base border border-surface-1">
              <div className="space-y-3 text-sm">
                <div className="flex items-center justify-between">
                  <span className="text-subtext-0">Schließfach:</span>
                  <span className="text-text font-medium">{selectedRental.locker_number}</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-subtext-0">Mieter:</span>
                  <span className="text-text">{selectedRental.renter_name}</span>
                </div>
                {selectedRental.renter_email && (
                  <div className="flex items-center justify-between">
                    <span className="text-subtext-0">E-Mail:</span>
                    <span className="text-text">{selectedRental.renter_email}</span>
                  </div>
                )}
                <div className="flex items-center justify-between">
                  <span className="text-subtext-0">Enddatum:</span>
                  <span className="text-text">{formatDate(selectedRental.end_date)}</span>
                </div>
              </div>
            </div>

            {/* Deposit Return Checkbox */}
            {selectedRental.deposit_paid && (
              <label className="flex items-center gap-3 cursor-pointer p-4 rounded-lg bg-success/10 border border-success/30">
                <input
                  type="checkbox"
                  checked={returnDeposit}
                  onChange={(e) => setReturnDeposit(e.target.checked)}
                  className="w-5 h-5 rounded border-surface-1 bg-base text-primary focus:ring-primary"
                />
                <div>
                  <span className="text-sm font-medium text-text">Pfand erstatten</span>
                  <p className="text-xs text-subtext-0">
                    {(selectedRental.deposit_amount_cents / 100).toFixed(2)} € werden zurückerstattet
                  </p>
                </div>
              </label>
            )}

            {isOverdue(selectedRental.end_date) && (
              <div className="p-3 rounded-lg bg-warning/10 border border-warning/30">
                <p className="text-sm text-warning">
                  <strong>Hinweis:</strong> Dieser Verleih ist überfällig. Prüfen Sie ggf. Mahngebühren.
                </p>
              </div>
            )}
          </div>
        )}
      </Modal>
    </div>
  );
}
