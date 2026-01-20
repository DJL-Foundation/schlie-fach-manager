import { useState } from 'react';
import { FileText, Search, Filter, Download } from 'lucide-react';
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
import { formatDateTime } from '@/lib/utils';

// Mock audit log data
const mockAuditLog = [
  {
    id: 1,
    timestamp: '2024-12-19T14:30:00',
    action: 'create',
    entity_type: 'rental',
    entity_id: 125,
    details: 'Neuer Verleih für Schließfach A-001 an Max Mustermann',
    username: 'admin',
  },
  {
    id: 2,
    timestamp: '2024-12-19T14:25:00',
    action: 'update',
    entity_type: 'locker',
    entity_id: 15,
    details: 'Schließfach B-015 als beschädigt markiert',
    username: 'admin',
  },
  {
    id: 3,
    timestamp: '2024-12-19T10:15:00',
    action: 'return',
    entity_type: 'rental',
    entity_id: 118,
    details: 'Rückgabe verarbeitet, Pfand erstattet: 10.00€',
    username: 'admin',
  },
  {
    id: 4,
    timestamp: '2024-12-18T16:45:00',
    action: 'extend',
    entity_type: 'rental',
    entity_id: 112,
    details: 'Verleih um 3 Monate verlängert bis 15.03.2025',
    username: 'admin',
  },
  {
    id: 5,
    timestamp: '2024-12-18T09:00:00',
    action: 'update',
    entity_type: 'setting',
    entity_id: null,
    details: 'Pfandbetrag geändert: 10.00€ → 15.00€',
    username: 'admin',
  },
  {
    id: 6,
    timestamp: '2024-12-17T15:30:00',
    action: 'delete',
    entity_type: 'locker',
    entity_id: 42,
    details: 'Schließfach C-042 gelöscht',
    username: 'admin',
  },
  {
    id: 7,
    timestamp: '2024-12-17T11:20:00',
    action: 'create',
    entity_type: 'locker',
    entity_id: 86,
    details: 'Neues Schließfach D-086 erstellt (Größe: M)',
    username: 'admin',
  },
];

const actionLabels: Record<string, string> = {
  create: 'Erstellt',
  update: 'Aktualisiert',
  delete: 'Gelöscht',
  return: 'Rückgabe',
  extend: 'Verlängert',
};

const entityLabels: Record<string, string> = {
  rental: 'Verleih',
  locker: 'Schließfach',
  setting: 'Einstellung',
  payment: 'Zahlung',
  location: 'Standort',
};

const actionFilterOptions = [
  { value: 'all', label: 'Alle Aktionen' },
  { value: 'create', label: 'Erstellt' },
  { value: 'update', label: 'Aktualisiert' },
  { value: 'delete', label: 'Gelöscht' },
  { value: 'return', label: 'Rückgabe' },
  { value: 'extend', label: 'Verlängert' },
];

const entityFilterOptions = [
  { value: 'all', label: 'Alle Typen' },
  { value: 'rental', label: 'Verleih' },
  { value: 'locker', label: 'Schließfach' },
  { value: 'setting', label: 'Einstellung' },
  { value: 'payment', label: 'Zahlung' },
];

function getActionBadge(action: string) {
  switch (action) {
    case 'create':
      return <Badge variant="success">Erstellt</Badge>;
    case 'update':
      return <Badge variant="info">Aktualisiert</Badge>;
    case 'delete':
      return <Badge variant="error">Gelöscht</Badge>;
    case 'return':
      return <Badge variant="primary">Rückgabe</Badge>;
    case 'extend':
      return <Badge variant="warning">Verlängert</Badge>;
    default:
      return <Badge>{actionLabels[action] || action}</Badge>;
  }
}

export function AuditLog() {
  const [searchTerm, setSearchTerm] = useState('');
  const [actionFilter, setActionFilter] = useState('all');
  const [entityFilter, setEntityFilter] = useState('all');

  const filteredLogs = mockAuditLog.filter((log) => {
    const matchesSearch = log.details.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesAction = actionFilter === 'all' || log.action === actionFilter;
    const matchesEntity = entityFilter === 'all' || log.entity_type === entityFilter;
    return matchesSearch && matchesAction && matchesEntity;
  });

  return (
    <div className="space-y-6 animate-fade-in">
      {/* Page Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="w-12 h-12 rounded-lg bg-secondary/20 flex items-center justify-center">
            <FileText className="w-6 h-6 text-secondary" />
          </div>
          <div>
            <h1 className="text-3xl font-bold text-text">Audit-Log</h1>
            <p className="text-subtext-0 mt-1">
              Alle Systemaktivitäten und Änderungen
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
              placeholder="Suchen in Details..."
              className="pl-10"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>
        </div>
        <Select
          value={actionFilter}
          onChange={(e) => setActionFilter(e.target.value)}
          options={actionFilterOptions}
        />
        <Select
          value={entityFilter}
          onChange={(e) => setEntityFilter(e.target.value)}
          options={entityFilterOptions}
        />
      </div>

      {/* Log Table */}
      <Card>
        <CardHeader>
          <CardTitle>Aktivitätsverlauf ({filteredLogs.length} Einträge)</CardTitle>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Zeitstempel</TableHead>
                <TableHead>Aktion</TableHead>
                <TableHead>Typ</TableHead>
                <TableHead>Details</TableHead>
                <TableHead>Benutzer</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {filteredLogs.map((log) => (
                <TableRow key={log.id}>
                  <TableCell className="text-subtext-0 whitespace-nowrap">
                    {formatDateTime(log.timestamp)}
                  </TableCell>
                  <TableCell>{getActionBadge(log.action)}</TableCell>
                  <TableCell className="text-subtext-1">
                    {entityLabels[log.entity_type] || log.entity_type}
                    {log.entity_id && (
                      <span className="ml-1 text-subtext-0">#{log.entity_id}</span>
                    )}
                  </TableCell>
                  <TableCell className="max-w-md">
                    <span className="text-text">{log.details}</span>
                  </TableCell>
                  <TableCell>
                    <span className="text-subtext-0">{log.username}</span>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>

          {filteredLogs.length === 0 && (
            <div className="text-center py-8 text-subtext-0">
              Keine Einträge gefunden.
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
