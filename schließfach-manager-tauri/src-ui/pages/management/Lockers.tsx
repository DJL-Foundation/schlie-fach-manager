import { useState } from 'react';
import { Plus, Search, Edit2, Trash2, AlertTriangle } from 'lucide-react';
import { toast } from 'sonner';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { Badge } from '@/components/ui/Badge';
import { Modal } from '@/components/ui/Modal';
import {
  Table,
  TableHeader,
  TableBody,
  TableRow,
  TableHead,
  TableCell,
} from '@/components/ui/Table';
import { cn } from '@/lib/cn';
import type { Locker, LockerSize } from '@/types';

// Mock data
const mockLockers: Locker[] = [
  { id: 1, number: 'A-001', location: 'Gebäude A', size: 'S', is_damaged: false, notes: null, created_at: '2024-01-01' },
  { id: 2, number: 'A-002', location: 'Gebäude A', size: 'M', is_damaged: false, notes: null, created_at: '2024-01-01' },
  { id: 3, number: 'A-003', location: 'Gebäude A', size: 'S', is_damaged: true, notes: 'Schloss defekt', created_at: '2024-01-01' },
  { id: 4, number: 'B-001', location: 'Gebäude B', size: 'L', is_damaged: false, notes: null, created_at: '2024-01-01' },
  { id: 5, number: 'B-002', location: 'Gebäude B', size: 'XL', is_damaged: false, notes: 'VIP Schließfach', created_at: '2024-01-01' },
  { id: 6, number: 'C-001', location: 'Gebäude C', size: 'M', is_damaged: false, notes: null, created_at: '2024-01-01' },
];

const sizeOptions = [
  { value: 'S', label: 'S - Klein' },
  { value: 'M', label: 'M - Mittel' },
  { value: 'L', label: 'L - Groß' },
  { value: 'XL', label: 'XL - Extra Groß' },
];

export function LockersManagement() {
  const [lockers, setLockers] = useState(mockLockers);
  const [searchTerm, setSearchTerm] = useState('');
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingLocker, setEditingLocker] = useState<Locker | null>(null);
  const [deleteConfirm, setDeleteConfirm] = useState<number | null>(null);

  // Form state
  const [formNumber, setFormNumber] = useState('');
  const [formLocation, setFormLocation] = useState('');
  const [formSize, setFormSize] = useState<LockerSize>('M');
  const [formNotes, setFormNotes] = useState('');
  const [formIsDamaged, setFormIsDamaged] = useState(false);

  const filteredLockers = lockers.filter(
    (locker) =>
      locker.number.toLowerCase().includes(searchTerm.toLowerCase()) ||
      locker.location.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const openCreateModal = () => {
    setEditingLocker(null);
    setFormNumber('');
    setFormLocation('');
    setFormSize('M');
    setFormNotes('');
    setFormIsDamaged(false);
    setIsModalOpen(true);
  };

  const openEditModal = (locker: Locker) => {
    setEditingLocker(locker);
    setFormNumber(locker.number);
    setFormLocation(locker.location);
    setFormSize(locker.size);
    setFormNotes(locker.notes || '');
    setFormIsDamaged(locker.is_damaged);
    setIsModalOpen(true);
  };

  const handleSave = () => {
    if (!formNumber || !formLocation) {
      toast.error('Bitte füllen Sie alle Pflichtfelder aus');
      return;
    }

    if (editingLocker) {
      // Update
      setLockers((prev) =>
        prev.map((l) =>
          l.id === editingLocker.id
            ? { ...l, number: formNumber, location: formLocation, size: formSize, notes: formNotes || null, is_damaged: formIsDamaged }
            : l
        )
      );
      toast.success('Schließfach aktualisiert');
    } else {
      // Create
      const newLocker: Locker = {
        id: Math.max(...lockers.map((l) => l.id)) + 1,
        number: formNumber,
        location: formLocation,
        size: formSize,
        notes: formNotes || null,
        is_damaged: formIsDamaged,
        created_at: new Date().toISOString(),
      };
      setLockers((prev) => [...prev, newLocker]);
      toast.success('Schließfach erstellt');
    }

    setIsModalOpen(false);
  };

  const handleDelete = (id: number) => {
    setLockers((prev) => prev.filter((l) => l.id !== id));
    setDeleteConfirm(null);
    toast.success('Schließfach gelöscht');
  };

  return (
    <div className="space-y-6 animate-fade-in">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-text">Schließfächer</h1>
          <p className="text-subtext-0 mt-1">
            Verwaltung aller Schließfächer
          </p>
        </div>
        <Button onClick={openCreateModal}>
          <Plus className="w-4 h-4" />
          Neues Schließfach
        </Button>
      </div>

      {/* Search */}
      <div className="flex items-center gap-4">
        <div className="flex-1 max-w-md">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-subtext-0" />
            <Input
              placeholder="Suchen nach Nummer oder Standort..."
              className="pl-10"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>
        </div>
      </div>

      {/* Table */}
      <div className="rounded-lg border border-surface-1 overflow-hidden">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Nummer</TableHead>
              <TableHead>Standort</TableHead>
              <TableHead>Größe</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Notizen</TableHead>
              <TableHead className="text-right">Aktionen</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {filteredLockers.map((locker) => (
              <TableRow key={locker.id}>
                <TableCell className="font-medium">{locker.number}</TableCell>
                <TableCell>{locker.location}</TableCell>
                <TableCell>
                  <Badge variant="default">{locker.size}</Badge>
                </TableCell>
                <TableCell>
                  {locker.is_damaged ? (
                    <Badge variant="error">
                      <AlertTriangle className="w-3 h-3 mr-1" />
                      Beschädigt
                    </Badge>
                  ) : (
                    <Badge variant="success">Verfügbar</Badge>
                  )}
                </TableCell>
                <TableCell className="text-subtext-0 max-w-[200px] truncate">
                  {locker.notes || '-'}
                </TableCell>
                <TableCell className="text-right">
                  <div className="flex items-center justify-end gap-2">
                    <Button
                      variant="ghost"
                      size="icon"
                      onClick={() => openEditModal(locker)}
                    >
                      <Edit2 className="w-4 h-4" />
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon"
                      onClick={() => setDeleteConfirm(locker.id)}
                    >
                      <Trash2 className="w-4 h-4 text-error" />
                    </Button>
                  </div>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>

      {/* Create/Edit Modal */}
      <Modal
        open={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        title={editingLocker ? 'Schließfach bearbeiten' : 'Neues Schließfach'}
        size="md"
        footer={
          <>
            <Button variant="outline" onClick={() => setIsModalOpen(false)}>
              Abbrechen
            </Button>
            <Button onClick={handleSave}>
              {editingLocker ? 'Speichern' : 'Erstellen'}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input
            label="Schließfach-Nummer"
            value={formNumber}
            onChange={(e) => setFormNumber(e.target.value)}
            placeholder="z.B. A-001"
            required
          />
          <Input
            label="Standort"
            value={formLocation}
            onChange={(e) => setFormLocation(e.target.value)}
            placeholder="z.B. Gebäude A"
            required
          />
          <Select
            label="Größe"
            value={formSize}
            onChange={(e) => setFormSize(e.target.value as LockerSize)}
            options={sizeOptions}
          />
          <Input
            label="Notizen"
            value={formNotes}
            onChange={(e) => setFormNotes(e.target.value)}
            placeholder="Optional..."
          />
          <label className="flex items-center gap-3 cursor-pointer">
            <input
              type="checkbox"
              checked={formIsDamaged}
              onChange={(e) => setFormIsDamaged(e.target.checked)}
              className="w-5 h-5 rounded border-surface-1 bg-base text-primary focus:ring-primary"
            />
            <span className="text-sm text-subtext-1">Als beschädigt markieren</span>
          </label>
        </div>
      </Modal>

      {/* Delete Confirmation Modal */}
      <Modal
        open={deleteConfirm !== null}
        onClose={() => setDeleteConfirm(null)}
        title="Schließfach löschen"
        size="sm"
        footer={
          <>
            <Button variant="outline" onClick={() => setDeleteConfirm(null)}>
              Abbrechen
            </Button>
            <Button variant="danger" onClick={() => deleteConfirm && handleDelete(deleteConfirm)}>
              Löschen
            </Button>
          </>
        }
      >
        <p className="text-subtext-0">
          Sind Sie sicher, dass Sie dieses Schließfach löschen möchten? Diese Aktion kann nicht rückgängig gemacht werden.
        </p>
      </Modal>
    </div>
  );
}
