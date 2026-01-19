import { useState } from 'react';
import { Save, RotateCcw } from 'lucide-react';
import { toast } from 'sonner';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/Card';
import type { Settings } from '@/types';

// Mock settings
const defaultSettings: Settings = {
  deposit_cents: 1000,
  yearly_fee_cents: 1000,
  billing_period: 'yearly',
  currency: 'EUR',
  screensaver_timeout_seconds: 300,
  app_version: '2.1.0-tauri',
};

export function SettingsPage() {
  const [settings, setSettings] = useState<Settings>(defaultSettings);
  const [isSaving, setIsSaving] = useState(false);

  const handleSave = async () => {
    try {
      setIsSaving(true);
      // TODO: Call Tauri command to save settings
      await new Promise((resolve) => setTimeout(resolve, 500));
      toast.success('Einstellungen gespeichert');
    } catch (error) {
      toast.error(`Fehler beim Speichern: ${error}`);
    } finally {
      setIsSaving(false);
    }
  };

  const handleReset = () => {
    setSettings(defaultSettings);
    toast.info('Einstellungen zurückgesetzt');
  };

  return (
    <div className="space-y-6 animate-fade-in max-w-3xl">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold text-text">Einstellungen</h1>
        <p className="text-subtext-0 mt-1">
          Anwendungseinstellungen verwalten
        </p>
      </div>

      {/* Financial Settings */}
      <Card>
        <CardHeader>
          <CardTitle>Finanzielle Einstellungen</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <Input
              label="Pfandbetrag (in Cents)"
              type="number"
              value={settings.deposit_cents}
              onChange={(e) =>
                setSettings((prev) => ({
                  ...prev,
                  deposit_cents: parseInt(e.target.value) || 0,
                }))
              }
              helperText={`= ${(settings.deposit_cents / 100).toFixed(2)} €`}
            />
            <Input
              label="Jahresgebühr (in Cents)"
              type="number"
              value={settings.yearly_fee_cents}
              onChange={(e) =>
                setSettings((prev) => ({
                  ...prev,
                  yearly_fee_cents: parseInt(e.target.value) || 0,
                }))
              }
              helperText={`= ${(settings.yearly_fee_cents / 100).toFixed(2)} €`}
            />
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <Select
              label="Berechnungszeitraum"
              value={settings.billing_period}
              onChange={(e) =>
                setSettings((prev) => ({
                  ...prev,
                  billing_period: e.target.value as 'monthly' | 'yearly',
                }))
              }
              options={[
                { value: 'monthly', label: 'Monatlich' },
                { value: 'yearly', label: 'Jährlich' },
              ]}
            />
            <Select
              label="Währung"
              value={settings.currency}
              onChange={(e) =>
                setSettings((prev) => ({ ...prev, currency: e.target.value }))
              }
              options={[
                { value: 'EUR', label: 'Euro (€)' },
                { value: 'CHF', label: 'Schweizer Franken (CHF)' },
                { value: 'USD', label: 'US-Dollar ($)' },
              ]}
            />
          </div>
        </CardContent>
      </Card>

      {/* Display Settings */}
      <Card>
        <CardHeader>
          <CardTitle>Anzeige-Einstellungen</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <Input
            label="Screensaver-Timeout (Sekunden)"
            type="number"
            value={settings.screensaver_timeout_seconds}
            onChange={(e) =>
              setSettings((prev) => ({
                ...prev,
                screensaver_timeout_seconds: parseInt(e.target.value) || 0,
              }))
            }
            helperText={`= ${Math.floor(settings.screensaver_timeout_seconds / 60)} Minuten`}
          />
        </CardContent>
      </Card>

      {/* App Info */}
      <Card>
        <CardHeader>
          <CardTitle>Anwendungsinformationen</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <span className="text-subtext-0">Version:</span>
              <span className="ml-2 text-text font-mono">{settings.app_version}</span>
            </div>
            <div>
              <span className="text-subtext-0">Framework:</span>
              <span className="ml-2 text-text">Tauri + React</span>
            </div>
            <div>
              <span className="text-subtext-0">Datenbank:</span>
              <span className="ml-2 text-text">SQLite</span>
            </div>
            <div>
              <span className="text-subtext-0">Theme:</span>
              <span className="ml-2 text-text">Catppuccin Mocha</span>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Actions */}
      <div className="flex items-center justify-end gap-4">
        <Button variant="outline" onClick={handleReset}>
          <RotateCcw className="w-4 h-4" />
          Zurücksetzen
        </Button>
        <Button onClick={handleSave} isLoading={isSaving}>
          <Save className="w-4 h-4" />
          Speichern
        </Button>
      </div>
    </div>
  );
}
