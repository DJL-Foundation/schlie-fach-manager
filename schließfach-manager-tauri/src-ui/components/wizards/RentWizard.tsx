import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import { User, Lock, CreditCard } from 'lucide-react';
import { WizardLayout } from './WizardLayout';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { Button } from '@/components/ui/Button';
import { cn } from '@/lib/cn';
import type { WizardStep, WizardStepProps, LockerSize } from '@/types';

// Mock available lockers
const mockAvailableLockers = [
  { id: 1, number: 'A-003', location: 'Gebäude A', size: 'S' as LockerSize },
  { id: 2, number: 'A-007', location: 'Gebäude A', size: 'M' as LockerSize },
  { id: 3, number: 'B-002', location: 'Gebäude B', size: 'L' as LockerSize },
  { id: 4, number: 'B-015', location: 'Gebäude B', size: 'S' as LockerSize },
  { id: 5, number: 'C-008', location: 'Gebäude C', size: 'XL' as LockerSize },
  { id: 6, number: 'C-012', location: 'Gebäude C', size: 'M' as LockerSize },
];

// Step 1: Renter Information
function RenterInfoStep({ data, updateData }: WizardStepProps) {
  const [name, setName] = useState((data.renter_name as string) || '');
  const [email, setEmail] = useState((data.renter_email as string) || '');
  const [phone, setPhone] = useState((data.renter_phone as string) || '');
  const [notes, setNotes] = useState((data.notes as string) || '');

  const handleUpdate = () => {
    updateData({ renter_name: name, renter_email: email, renter_phone: phone, notes });
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3 mb-6">
        <div className="w-10 h-10 rounded-lg bg-primary/20 flex items-center justify-center">
          <User className="w-5 h-5 text-primary" />
        </div>
        <div>
          <h2 className="text-xl font-semibold text-text">Mieter-Informationen</h2>
          <p className="text-sm text-subtext-0">Kontaktdaten des Mieters erfassen</p>
        </div>
      </div>

      <Input
        label="Name"
        value={name}
        onChange={(e) => {
          setName(e.target.value);
          handleUpdate();
        }}
        onBlur={handleUpdate}
        required
        placeholder="Max Mustermann"
      />

      <Input
        label="E-Mail"
        type="email"
        value={email}
        onChange={(e) => {
          setEmail(e.target.value);
          handleUpdate();
        }}
        onBlur={handleUpdate}
        placeholder="max@example.com"
      />

      <Input
        label="Telefon"
        type="tel"
        value={phone}
        onChange={(e) => {
          setPhone(e.target.value);
          handleUpdate();
        }}
        onBlur={handleUpdate}
        placeholder="+49 123 456789"
      />

      <div className="space-y-2">
        <label className="block text-sm font-medium text-subtext-1">Notizen</label>
        <textarea
          className="w-full px-3 py-2 rounded-md border border-surface-1 bg-base text-text placeholder:text-overlay-1 focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent min-h-[100px]"
          value={notes}
          onChange={(e) => {
            setNotes(e.target.value);
            handleUpdate();
          }}
          onBlur={handleUpdate}
          placeholder="Zusätzliche Informationen..."
        />
      </div>
    </div>
  );
}

// Step 2: Locker Selection
function LockerSelectStep({ data, updateData }: WizardStepProps) {
  const [selectedLocker, setSelectedLocker] = useState<number | null>(
    (data.locker_id as number) || null
  );
  const [filterSize, setFilterSize] = useState<string>('all');
  const [filterLocation, setFilterLocation] = useState<string>('all');

  const filteredLockers = mockAvailableLockers.filter((locker) => {
    if (filterSize !== 'all' && locker.size !== filterSize) return false;
    if (filterLocation !== 'all' && locker.location !== filterLocation) return false;
    return true;
  });

  const handleSelect = (lockerId: number) => {
    setSelectedLocker(lockerId);
    updateData({ locker_id: lockerId });
  };

  const locations = [...new Set(mockAvailableLockers.map((l) => l.location))];
  const sizes: LockerSize[] = ['S', 'M', 'L', 'XL'];

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3 mb-6">
        <div className="w-10 h-10 rounded-lg bg-secondary/20 flex items-center justify-center">
          <Lock className="w-5 h-5 text-secondary" />
        </div>
        <div>
          <h2 className="text-xl font-semibold text-text">Schließfach auswählen</h2>
          <p className="text-sm text-subtext-0">Wählen Sie ein verfügbares Schließfach</p>
        </div>
      </div>

      {/* Filters */}
      <div className="flex gap-4">
        <Select
          label="Größe"
          value={filterSize}
          onChange={(e) => setFilterSize(e.target.value)}
          options={[
            { value: 'all', label: 'Alle Größen' },
            ...sizes.map((s) => ({ value: s, label: `Größe ${s}` })),
          ]}
        />
        <Select
          label="Standort"
          value={filterLocation}
          onChange={(e) => setFilterLocation(e.target.value)}
          options={[
            { value: 'all', label: 'Alle Standorte' },
            ...locations.map((l) => ({ value: l, label: l })),
          ]}
        />
      </div>

      {/* Locker Grid */}
      <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
        {filteredLockers.map((locker) => (
          <button
            key={locker.id}
            onClick={() => handleSelect(locker.id)}
            className={cn(
              'p-4 rounded-lg border-2 transition-colors text-left',
              selectedLocker === locker.id
                ? 'border-primary bg-primary/10'
                : 'border-surface-1 hover:border-surface-2 bg-base'
            )}
          >
            <div className="font-bold text-text">{locker.number}</div>
            <div className="text-sm text-subtext-0">Größe {locker.size}</div>
            <div className="text-sm text-subtext-0">{locker.location}</div>
          </button>
        ))}
      </div>

      {filteredLockers.length === 0 && (
        <div className="text-center py-8 text-subtext-0">
          Keine Schließfächer mit diesen Filtern verfügbar.
        </div>
      )}
    </div>
  );
}

// Step 3: Payment & Duration
function PaymentStep({ data, updateData }: WizardStepProps) {
  const [startDate, setStartDate] = useState(
    (data.start_date as string) || new Date().toISOString().split('T')[0]
  );
  const [duration, setDuration] = useState((data.duration as string) || '12');
  const [depositPaid, setDepositPaid] = useState((data.deposit_paid as boolean) ?? true);

  const handleUpdate = () => {
    updateData({ start_date: startDate, duration: parseInt(duration), deposit_paid: depositPaid });
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3 mb-6">
        <div className="w-10 h-10 rounded-lg bg-success/20 flex items-center justify-center">
          <CreditCard className="w-5 h-5 text-success" />
        </div>
        <div>
          <h2 className="text-xl font-semibold text-text">Zeitraum & Zahlung</h2>
          <p className="text-sm text-subtext-0">Mietdauer und Pfand festlegen</p>
        </div>
      </div>

      <Input
        label="Startdatum"
        type="date"
        value={startDate}
        onChange={(e) => {
          setStartDate(e.target.value);
          handleUpdate();
        }}
        onBlur={handleUpdate}
        required
      />

      <Select
        label="Mietdauer"
        value={duration}
        onChange={(e) => {
          setDuration(e.target.value);
          handleUpdate();
        }}
        options={[
          { value: '1', label: '1 Monat' },
          { value: '3', label: '3 Monate' },
          { value: '6', label: '6 Monate' },
          { value: '12', label: '12 Monate' },
          { value: '24', label: '24 Monate' },
        ]}
      />

      {/* Summary */}
      <div className="p-4 rounded-lg bg-base border border-surface-1 space-y-3">
        <h3 className="font-medium text-text">Zusammenfassung</h3>
        <div className="flex items-center justify-between text-sm">
          <span className="text-subtext-0">Jahresgebühr</span>
          <span className="text-text">10,00 €</span>
        </div>
        <div className="flex items-center justify-between text-sm">
          <span className="text-subtext-0">Pfand</span>
          <span className="text-text">10,00 €</span>
        </div>
        <div className="h-px bg-surface-1" />
        <div className="flex items-center justify-between font-medium">
          <span className="text-text">Gesamt</span>
          <span className="text-text">20,00 €</span>
        </div>
      </div>

      <label className="flex items-center gap-3 cursor-pointer">
        <input
          type="checkbox"
          checked={depositPaid}
          onChange={(e) => {
            setDepositPaid(e.target.checked);
            handleUpdate();
          }}
          className="w-5 h-5 rounded border-surface-1 bg-base text-primary focus:ring-primary"
        />
        <span className="text-sm text-subtext-1">Pfand wurde bezahlt</span>
      </label>
    </div>
  );
}

// Main Wizard Component
export function RentWizard() {
  const navigate = useNavigate();

  const steps: WizardStep[] = [
    {
      id: 'renter',
      title: 'Mieter',
      description: 'Kontaktdaten',
      component: RenterInfoStep,
    },
    {
      id: 'locker',
      title: 'Schließfach',
      description: 'Auswahl',
      component: LockerSelectStep,
    },
    {
      id: 'payment',
      title: 'Zahlung',
      description: 'Zeitraum & Pfand',
      component: PaymentStep,
    },
  ];

  const handleComplete = async (data: Record<string, unknown>) => {
    try {
      // TODO: Call Tauri command to create rental
      console.log('Creating rental:', data);
      
      // Simulate API call
      await new Promise((resolve) => setTimeout(resolve, 1000));
      
      toast.success('Verleih erfolgreich erstellt!');
      navigate('/rentals/active');
    } catch (error) {
      toast.error(`Fehler: ${error}`);
      throw error;
    }
  };

  const handleCancel = () => {
    navigate('/rentals/overview');
  };

  return (
    <WizardLayout
      title="Neuen Verleih erstellen"
      steps={steps}
      onComplete={handleComplete}
      onCancel={handleCancel}
    />
  );
}
