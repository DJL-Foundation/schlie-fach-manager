import { useState } from 'react';
import { Search, ArrowUpRight } from 'lucide-react';
import { toast } from 'sonner';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
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
import { formatDate, isExpiringSoon } from '@/lib/utils';

// Mock data for active rentals that can be extended
const mockActiveRentals = [
  {
    id: 1,
    locker_number: 'A-001',
    renter_name: 'Max Mustermann',
    end_date: '2025-01-15',
    can_extend: true,
  },
  {
    id: 2,
    locker_number: 'A-002',
    renter_name: 'Erika Musterfrau',
    end_date: '2025-06-01',
    can_extend: true,
  },
  {
    id: 3,
    locker_number: 'B-005',
    renter_name: 'Peter Müller',
    end_date: '2025-02-01',
    can_extend: true,
  },
];

const extensionOptions = [
  { value: '1', label: '1 Monat' },
  { value: '3', label: '3 Monate' },
  { value: '6', label: '6 Monate' },
  { value: '12', label: '12 Monate' },
];

export function ExtendRental() {
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedRental, setSelectedRental] = useState<typeof mockActiveRentals[0] | null>(null);
  const [extensionMonths, setExtensionMonths] = useState('3');
  const [isExtending, setIsExtending] = useState(false);

  const filteredRentals = mockActiveRentals.filter(
    (rental) =>
      rental.locker_number.toLowerCase().includes(searchTerm.toLowerCase()) ||
      rental.renter_name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const handleExtend = async () => {
    if (!selectedRental) return;
    
    try {
      setIsExtending(true);
      // TODO: Call Tauri command to extend rental
      await new Promise((resolve) => setTimeout(resolve, 1000));
      toast.success(`Verleih um ${extensionMonths} Monate verlängert`);
      setSelectedRental(null);
    } catch (error) {
      toast.error(`Fehler: ${error}`);
    } finally {
      setIsExtending(false);
    }
  };

  return (
    <div className="space-y-6 animate-fade-in">
      {/* Page Header */}
      <div className="flex items-center gap-4">
        <div className="w-12 h-12 rounded-lg bg-primary/20 flex items-center justify-center">
          <ArrowUpRight className="w-6 h-6 text-primary" />
        </div>
        <div>
          <h1 className="text-3xl font-bold text-text">Verleih verlängern</h1>
          <p className="text-subtext-0 mt-1">
            Wählen Sie einen aktiven Verleih zur Verlängerung
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
          <CardTitle>Aktive Verleihe</CardTitle>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Schließfach</TableHead>
                <TableHead>Mieter</TableHead>
                <TableHead>Aktuelles Ende</TableHead>
                <TableHead>Status</TableHead>
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
                    {isExpiringSoon(rental.end_date) ? (
                      <Badge variant="warning">Läuft bald ab</Badge>
                    ) : (
                      <Badge variant="success">Aktiv</Badge>
                    )}
                  </TableCell>
                  <TableCell className="text-right">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => setSelectedRental(rental)}
                    >
                      <ArrowUpRight className="w-4 h-4" />
                      Verlängern
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      {/* Extension Modal */}
      <Modal
        open={selectedRental !== null}
        onClose={() => setSelectedRental(null)}
        title="Verleih verlängern"
        size="md"
        footer={
          <>
            <Button variant="outline" onClick={() => setSelectedRental(null)}>
              Abbrechen
            </Button>
            <Button onClick={handleExtend} isLoading={isExtending}>
              <ArrowUpRight className="w-4 h-4" />
              Verlängern
            </Button>
          </>
        }
      >
        {selectedRental && (
          <div className="space-y-4">
            <div className="p-4 rounded-lg bg-base border border-surface-1">
              <div className="grid grid-cols-2 gap-4 text-sm">
                <div>
                  <span className="text-subtext-0">Schließfach:</span>
                  <span className="ml-2 text-text font-medium">{selectedRental.locker_number}</span>
                </div>
                <div>
                  <span className="text-subtext-0">Mieter:</span>
                  <span className="ml-2 text-text">{selectedRental.renter_name}</span>
                </div>
                <div className="col-span-2">
                  <span className="text-subtext-0">Aktuelles Ende:</span>
                  <span className="ml-2 text-text">{formatDate(selectedRental.end_date)}</span>
                </div>
              </div>
            </div>

            <Select
              label="Verlängerung um"
              value={extensionMonths}
              onChange={(e) => setExtensionMonths(e.target.value)}
              options={extensionOptions}
            />

            <div className="p-4 rounded-lg bg-success/10 border border-success/30">
              <p className="text-sm text-success">
                Nach der Verlängerung endet der Verleih am{' '}
                <strong>
                  {/* Calculate new end date (simplified) */}
                  {formatDate(
                    new Date(
                      new Date(selectedRental.end_date).getTime() +
                        parseInt(extensionMonths) * 30 * 24 * 60 * 60 * 1000
                    ).toISOString()
                  )}
                </strong>
              </p>
            </div>
          </div>
        )}
      </Modal>
    </div>
  );
}
