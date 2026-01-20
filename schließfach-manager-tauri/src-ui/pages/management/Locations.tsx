import { useState } from 'react';
import { MapPin, Search, Plus, Edit2, Trash2 } from 'lucide-react';
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

// Mock locations data
const mockLocations = [
  {
    id: 1,
    name: 'Gebäude A',
    description: 'Hauptgebäude, Erdgeschoss',
    total_lockers: 50,
    occupied_lockers: 32,
    damaged_lockers: 2,
  },
  {
    id: 2,
    name: 'Gebäude B',
    description: 'Nebengebäude, 1. Etage',
    total_lockers: 30,
    occupied_lockers: 18,
    damaged_lockers: 0,
  },
  {
    id: 3,
    name: 'Gebäude C',
    description: 'Außenbereich, überdacht',
    total_lockers: 20,
    occupied_lockers: 8,
    damaged_lockers: 1,
  },
];

export function LocationsManagement() {
  const [searchTerm, setSearchTerm] = useState('');
  const [isAddModalOpen, setIsAddModalOpen] = useState(false);
  const [editingLocation, setEditingLocation] = useState<typeof mockLocations[0] | null>(null);
  const [deleteLocation, setDeleteLocation] = useState<typeof mockLocations[0] | null>(null);

  const filteredLocations = mockLocations.filter(
    (loc) =>
      loc.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      loc.description.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const totalLockers = mockLocations.reduce((sum, loc) => sum + loc.total_lockers, 0);
  const totalOccupied = mockLocations.reduce((sum, loc) => sum + loc.occupied_lockers, 0);
  const totalDamaged = mockLocations.reduce((sum, loc) => sum + loc.damaged_lockers, 0);

  return (
    <div className="space-y-6 animate-fade-in">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-lg bg-accent/20 flex items-center justify-center">
            <MapPin className="w-6 h-6 text-accent" />
          </div>
          <div>
            <h1 className="text-3xl font-bold text-text">Standorte</h1>
            <p className="text-subtext-0 mt-1">
              Standorte und Gebäude verwalten
            </p>
          </div>
        </div>
        <Button onClick={() => setIsAddModalOpen(true)}>
          <Plus className="w-4 h-4" />
          Neuer Standort
        </Button>
      </div>

      {/* Search */}
      <div className="flex items-center gap-4">
        <div className="flex-1 max-w-md">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-subtext-0" />
            <Input
              placeholder="Standort suchen..."
              className="pl-10"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>
        </div>
      </div>

      {/* Summary */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-text">{mockLocations.length}</div>
            <p className="text-sm text-subtext-0">Standorte</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-text">{totalLockers}</div>
            <p className="text-sm text-subtext-0">Schließfächer gesamt</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-success">{totalOccupied}</div>
            <p className="text-sm text-subtext-0">Belegt</p>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="pt-6">
            <div className="text-2xl font-bold text-error">{totalDamaged}</div>
            <p className="text-sm text-subtext-0">Beschädigt</p>
          </CardContent>
        </Card>
      </div>

      {/* Table */}
      <Card>
        <CardHeader>
          <CardTitle>Alle Standorte</CardTitle>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead>Beschreibung</TableHead>
                <TableHead>Schließfächer</TableHead>
                <TableHead>Belegung</TableHead>
                <TableHead>Status</TableHead>
                <TableHead className="text-right">Aktionen</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {filteredLocations.map((location) => {
                const occupancyPercent = Math.round(
                  (location.occupied_lockers / location.total_lockers) * 100
                );
                return (
                  <TableRow key={location.id}>
                    <TableCell className="font-medium">{location.name}</TableCell>
                    <TableCell className="text-subtext-0">{location.description}</TableCell>
                    <TableCell>{location.total_lockers}</TableCell>
                    <TableCell>
                      <div className="flex items-center gap-2">
                        <div className="w-24 h-2 bg-surface-1 rounded-full overflow-hidden">
                          <div
                            className="h-full bg-primary transition-all"
                            style={{ width: `${occupancyPercent}%` }}
                          />
                        </div>
                        <span className="text-sm text-subtext-0">{occupancyPercent}%</span>
                      </div>
                    </TableCell>
                    <TableCell>
                      {location.damaged_lockers > 0 ? (
                        <Badge variant="warning">
                          {location.damaged_lockers} beschädigt
                        </Badge>
                      ) : (
                        <Badge variant="success">OK</Badge>
                      )}
                    </TableCell>
                    <TableCell className="text-right">
                      <div className="flex items-center justify-end gap-2">
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => setEditingLocation(location)}
                        >
                          <Edit2 className="w-4 h-4" />
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => setDeleteLocation(location)}
                        >
                          <Trash2 className="w-4 h-4" />
                        </Button>
                      </div>
                    </TableCell>
                  </TableRow>
                );
              })}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      {/* Add Location Modal */}
      <Modal
        open={isAddModalOpen}
        onClose={() => setIsAddModalOpen(false)}
        title="Neuer Standort"
        size="md"
        footer={
          <>
            <Button variant="outline" onClick={() => setIsAddModalOpen(false)}>
              Abbrechen
            </Button>
            <Button
              onClick={() => {
                toast.success('Standort erstellt');
                setIsAddModalOpen(false);
              }}
            >
              Erstellen
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input label="Name" placeholder="z.B. Gebäude A" required />
          <Input label="Beschreibung" placeholder="Optionale Beschreibung..." />
        </div>
      </Modal>

      {/* Edit Location Modal */}
      <Modal
        open={editingLocation !== null}
        onClose={() => setEditingLocation(null)}
        title="Standort bearbeiten"
        size="md"
        footer={
          <>
            <Button variant="outline" onClick={() => setEditingLocation(null)}>
              Abbrechen
            </Button>
            <Button
              onClick={() => {
                toast.success('Standort aktualisiert');
                setEditingLocation(null);
              }}
            >
              Speichern
            </Button>
          </>
        }
      >
        {editingLocation && (
          <div className="space-y-4">
            <Input label="Name" defaultValue={editingLocation.name} required />
            <Input label="Beschreibung" defaultValue={editingLocation.description} />
          </div>
        )}
      </Modal>

      {/* Delete Confirmation Modal */}
      <Modal
        open={deleteLocation !== null}
        onClose={() => setDeleteLocation(null)}
        title="Standort löschen"
        size="sm"
        footer={
          <>
            <Button variant="outline" onClick={() => setDeleteLocation(null)}>
              Abbrechen
            </Button>
            <Button
              variant="danger"
              onClick={() => {
                toast.success('Standort gelöscht');
                setDeleteLocation(null);
              }}
            >
              Löschen
            </Button>
          </>
        }
      >
        {deleteLocation && (
          <div className="space-y-4">
            <p className="text-text">
              Möchten Sie den Standort <strong>{deleteLocation.name}</strong> wirklich löschen?
            </p>
            {deleteLocation.total_lockers > 0 && (
              <div className="p-3 rounded-lg bg-error/10 border border-error/30">
                <p className="text-sm text-error">
                  <strong>Warnung:</strong> Dieser Standort enthält noch {deleteLocation.total_lockers} Schließfächer.
                  Diese müssen zuerst einem anderen Standort zugewiesen werden.
                </p>
              </div>
            )}
          </div>
        )}
      </Modal>
    </div>
  );
}
