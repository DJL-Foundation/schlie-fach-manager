import { useState } from 'react';
import { DollarSign, Search, Plus, Download } from 'lucide-react';
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
import { formatDate, formatCurrency } from '@/lib/utils';

// Mock payments data
const mockPayments = [
  {
    id: 1,
    rental_id: 1,
    locker_number: 'A-001',
    renter_name: 'Max Mustermann',
    amount_cents: 1000,
    payment_type: 'deposit',
    payment_date: '2024-01-15',
    notes: null,
  },
  {
    id: 2,
    rental_id: 1,
    locker_number: 'A-001',
    renter_name: 'Max Mustermann',
    amount_cents: 1000,
    payment_type: 'yearly_fee',
    payment_date: '2024-01-15',
    notes: null,
  },
  {
    id: 3,
    rental_id: 2,
    locker_number: 'B-005',
    renter_name: 'Erika Musterfrau',
    amount_cents: 1000,
    payment_type: 'deposit',
    payment_date: '2024-06-01',
    notes: null,
  },
  {
    id: 4,
    rental_id: 3,
    locker_number: 'C-010',
    renter_name: 'Peter Müller',
    amount_cents: 500,
    payment_type: 'extension',
    payment_date: '2024-09-15',
    notes: 'Verlängerung um 6 Monate',
  },
];

const paymentTypeLabels: Record<string, string> = {
  deposit: 'Pfand',
  yearly_fee: 'Jahresgebühr',
  extension: 'Verlängerung',
  refund: 'Rückerstattung',
  other: 'Sonstiges',
};

const paymentTypeOptions = [
  { value: 'all', label: 'Alle Typen' },
  { value: 'deposit', label: 'Pfand' },
  { value: 'yearly_fee', label: 'Jahresgebühr' },
  { value: 'extension', label: 'Verlängerung' },
  { value: 'refund', label: 'Rückerstattung' },
  { value: 'other', label: 'Sonstiges' },
];

function getPaymentTypeBadge(type: string) {
  switch (type) {
    case 'deposit':
      return <Badge variant="primary">Pfand</Badge>;
    case 'yearly_fee':
      return <Badge variant="success">Jahresgebühr</Badge>;
    case 'extension':
      return <Badge variant="info">Verlängerung</Badge>;
    case 'refund':
      return <Badge variant="warning">Rückerstattung</Badge>;
    default:
      return <Badge>{paymentTypeLabels[type] || type}</Badge>;
  }
}

export function Payments() {
  const [searchTerm, setSearchTerm] = useState('');
  const [typeFilter, setTypeFilter] = useState('all');
  const [isAddModalOpen, setIsAddModalOpen] = useState(false);

  const filteredPayments = mockPayments.filter((payment) => {
    const matchesSearch =
      payment.locker_number.toLowerCase().includes(searchTerm.toLowerCase()) ||
      payment.renter_name.toLowerCase().includes(searchTerm.toLowerCase());

    const matchesType =
      typeFilter === 'all' || payment.payment_type === typeFilter;

    return matchesSearch && matchesType;
  });

  const totalAmount = filteredPayments.reduce((sum, p) => sum + p.amount_cents, 0);

  return (
    <div className="space-y-6 animate-fade-in">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-lg bg-success/20 flex items-center justify-center">
            <DollarSign className="w-6 h-6 text-success" />
          </div>
          <div>
            <h1 className="text-3xl font-bold text-text">Zahlungen</h1>
            <p className="text-subtext-0 mt-1">
              Alle Zahlungsvorgänge verwalten
            </p>
          </div>
        </div>
        <div className="flex gap-2">
          <Button variant="outline">
            <Download className="w-4 h-4" />
            Exportieren
          </Button>
          <Button onClick={() => setIsAddModalOpen(true)}>
            <Plus className="w-4 h-4" />
            Zahlung erfassen
          </Button>
        </div>
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
          value={typeFilter}
          onChange={(e) => setTypeFilter(e.target.value)}
          options={paymentTypeOptions}
        />
      </div>

      {/* Summary */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-text">{filteredPayments.length}</div>
            <p className="text-sm text-subtext-0">Zahlungen</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-success">{formatCurrency(totalAmount)}</div>
            <p className="text-sm text-subtext-0">Gesamtsumme</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-text">
              {formatCurrency(Math.round(totalAmount / Math.max(filteredPayments.length, 1)))}
            </div>
            <p className="text-sm text-subtext-0">Durchschnitt</p>
          </CardContent>
        </Card>
      </div>

      {/* Table */}
      <Card>
        <CardHeader>
          <CardTitle>Zahlungsverlauf</CardTitle>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Datum</TableHead>
                <TableHead>Schließfach</TableHead>
                <TableHead>Mieter</TableHead>
                <TableHead>Typ</TableHead>
                <TableHead className="text-right">Betrag</TableHead>
                <TableHead>Notizen</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {filteredPayments.map((payment) => (
                <TableRow key={payment.id}>
                  <TableCell>{formatDate(payment.payment_date)}</TableCell>
                  <TableCell className="font-medium">{payment.locker_number}</TableCell>
                  <TableCell>{payment.renter_name}</TableCell>
                  <TableCell>{getPaymentTypeBadge(payment.payment_type)}</TableCell>
                  <TableCell className="text-right font-medium">
                    {formatCurrency(payment.amount_cents)}
                  </TableCell>
                  <TableCell className="text-subtext-0 max-w-[200px] truncate">
                    {payment.notes || '-'}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      {/* Add Payment Modal */}
      <Modal
        open={isAddModalOpen}
        onClose={() => setIsAddModalOpen(false)}
        title="Zahlung erfassen"
        size="md"
        footer={
          <>
            <Button variant="outline" onClick={() => setIsAddModalOpen(false)}>
              Abbrechen
            </Button>
            <Button onClick={() => {
              toast.success('Zahlung erfasst');
              setIsAddModalOpen(false);
            }}>
              Speichern
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input label="Verleih-ID oder Schließfach" placeholder="z.B. A-001" required />
          <Select
            label="Zahlungstyp"
            options={paymentTypeOptions.slice(1)}
            required
          />
          <Input label="Betrag (€)" type="number" placeholder="10.00" required />
          <Input label="Datum" type="date" />
          <Input label="Notizen" placeholder="Optional..." />
        </div>
      </Modal>
    </div>
  );
}
