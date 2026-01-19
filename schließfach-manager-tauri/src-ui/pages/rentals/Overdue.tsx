import { AlertTriangle, Mail, Phone } from 'lucide-react';
import { Button } from '@/components/ui/Button';
import { Badge } from '@/components/ui/Badge';
import { Card, CardContent } from '@/components/ui/Card';
import { formatDate } from '@/lib/utils';

// Mock data for overdue rentals
const mockOverdueRentals = [
  {
    id: 1,
    locker_number: 'C-010',
    renter_name: 'Klaus Weber',
    renter_email: 'klaus@example.com',
    renter_phone: '+49 111 222333',
    end_date: '2024-10-15',
    days_overdue: 15,
    deposit_paid: true,
  },
  {
    id: 2,
    locker_number: 'A-025',
    renter_name: 'Anna Bauer',
    renter_email: 'anna@example.com',
    renter_phone: null,
    end_date: '2024-10-20',
    days_overdue: 10,
    deposit_paid: true,
  },
  {
    id: 3,
    locker_number: 'B-008',
    renter_name: 'Thomas Richter',
    renter_email: null,
    renter_phone: '+49 333 444555',
    end_date: '2024-10-28',
    days_overdue: 2,
    deposit_paid: false,
  },
];

export function OverdueRentals() {
  return (
    <div className="space-y-6 animate-fade-in">
      {/* Page Header */}
      <div className="flex items-center gap-4">
        <div className="w-12 h-12 rounded-lg bg-error/20 flex items-center justify-center">
          <AlertTriangle className="w-6 h-6 text-error" />
        </div>
        <div>
          <h1 className="text-3xl font-bold text-text">Überfällige Verleihe</h1>
          <p className="text-subtext-0 mt-1">
            {mockOverdueRentals.length} Verleih(e) erfordern Aufmerksamkeit
          </p>
        </div>
      </div>

      {/* Alert Banner */}
      <div className="p-4 rounded-lg bg-error/10 border border-error/30">
        <div className="flex items-start gap-3">
          <AlertTriangle className="w-5 h-5 text-error flex-shrink-0 mt-0.5" />
          <div>
            <h3 className="font-medium text-error">Handlungsbedarf</h3>
            <p className="text-sm text-subtext-0 mt-1">
              Die folgenden Verleihe haben ihr Enddatum überschritten. 
              Bitte kontaktieren Sie die Mieter oder veranlassen Sie die Rückgabe.
            </p>
          </div>
        </div>
      </div>

      {/* Overdue Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {mockOverdueRentals.map((rental) => (
          <Card key={rental.id} className="border-error/30">
            <CardContent className="pt-6">
              <div className="flex items-start justify-between mb-4">
                <div>
                  <h3 className="font-bold text-lg text-text">
                    {rental.locker_number}
                  </h3>
                  <p className="text-subtext-0">{rental.renter_name}</p>
                </div>
                <Badge variant="error">{rental.days_overdue} Tage überfällig</Badge>
              </div>

              <div className="space-y-2 text-sm">
                <div className="flex items-center justify-between">
                  <span className="text-subtext-0">Enddatum:</span>
                  <span className="text-text">{formatDate(rental.end_date)}</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-subtext-0">Pfand:</span>
                  {rental.deposit_paid ? (
                    <Badge variant="success">Bezahlt</Badge>
                  ) : (
                    <Badge variant="error">Ausstehend</Badge>
                  )}
                </div>
              </div>

              {/* Contact Info */}
              <div className="mt-4 pt-4 border-t border-surface-1 space-y-2">
                {rental.renter_email && (
                  <div className="flex items-center gap-2 text-sm text-subtext-0">
                    <Mail className="w-4 h-4" />
                    <span>{rental.renter_email}</span>
                  </div>
                )}
                {rental.renter_phone && (
                  <div className="flex items-center gap-2 text-sm text-subtext-0">
                    <Phone className="w-4 h-4" />
                    <span>{rental.renter_phone}</span>
                  </div>
                )}
              </div>

              {/* Actions */}
              <div className="mt-4 flex gap-2">
                <Button variant="outline" size="sm" className="flex-1">
                  Kontaktieren
                </Button>
                <Button variant="danger" size="sm" className="flex-1">
                  Rückgabe
                </Button>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      {mockOverdueRentals.length === 0 && (
        <div className="text-center py-12">
          <div className="w-16 h-16 rounded-full bg-success/20 flex items-center justify-center mx-auto mb-4">
            <AlertTriangle className="w-8 h-8 text-success" />
          </div>
          <h3 className="text-lg font-medium text-text">Keine überfälligen Verleihe</h3>
          <p className="text-subtext-0 mt-1">
            Alle Schließfächer wurden rechtzeitig zurückgegeben.
          </p>
        </div>
      )}
    </div>
  );
}
