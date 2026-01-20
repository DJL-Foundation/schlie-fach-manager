import { useState } from 'react';
import { Download, Upload, FileText, File, Database } from 'lucide-react';
import { toast } from 'sonner';
import { Button } from '@/components/ui/Button';
import { Select } from '@/components/ui/Select';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/Card';

type ExportFormat = 'toml' | 'json' | 'csv';

export function ExportImport() {
  const [exportFormat, setExportFormat] = useState<ExportFormat>('toml');
  const [isExporting, setIsExporting] = useState(false);
  const [isImporting, setIsImporting] = useState(false);

  const handleExport = async () => {
    try {
      setIsExporting(true);
      // TODO: Call Tauri command to export data
      await new Promise((resolve) => setTimeout(resolve, 1000));
      toast.success(`Daten erfolgreich als ${exportFormat.toUpperCase()} exportiert`);
    } catch (error) {
      toast.error(`Export fehlgeschlagen: ${error}`);
    } finally {
      setIsExporting(false);
    }
  };

  const handleImport = async () => {
    try {
      setIsImporting(true);
      // TODO: Call Tauri dialog to select file and import
      await new Promise((resolve) => setTimeout(resolve, 1000));
      toast.success('Daten erfolgreich importiert');
    } catch (error) {
      toast.error(`Import fehlgeschlagen: ${error}`);
    } finally {
      setIsImporting(false);
    }
  };

  return (
    <div className="space-y-6 animate-fade-in max-w-4xl">
      {/* Page Header */}
      <div>
        <h1 className="text-3xl font-bold text-text">Export / Import</h1>
        <p className="text-subtext-0 mt-1">
          Daten sichern und wiederherstellen
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Export Card */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Download className="w-5 h-5 text-primary" />
              Daten exportieren
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <p className="text-sm text-subtext-0">
              Exportieren Sie alle Daten (Schließfächer, Verleihe, Zahlungen) zur Sicherung oder Übertragung.
            </p>

            <Select
              label="Export-Format"
              value={exportFormat}
              onChange={(e) => setExportFormat(e.target.value as ExportFormat)}
              options={[
                { value: 'toml', label: 'TOML (Standard, empfohlen)' },
                { value: 'json', label: 'JSON (Webkompatibel)' },
                { value: 'csv', label: 'CSV (Excel-kompatibel)' },
              ]}
            />

            {/* Format Info */}
            <div className="p-3 rounded-lg bg-base border border-surface-1">
              <div className="flex items-start gap-3">
                <div className="w-8 h-8 rounded bg-surface-1 flex items-center justify-center flex-shrink-0">
                  {exportFormat === 'toml' && <FileText className="w-4 h-4 text-primary" />}
                  {exportFormat === 'json' && <File className="w-4 h-4 text-secondary" />}
                  {exportFormat === 'csv' && <Database className="w-4 h-4 text-success" />}
                </div>
                <div className="text-sm">
                  {exportFormat === 'toml' && (
                    <>
                      <div className="font-medium text-text">TOML Format</div>
                      <div className="text-subtext-0">
                        Menschenlesbar, strukturiert, ideal für Backups
                      </div>
                    </>
                  )}
                  {exportFormat === 'json' && (
                    <>
                      <div className="font-medium text-text">JSON Format</div>
                      <div className="text-subtext-0">
                        Standard-Datenaustauschformat, für APIs geeignet
                      </div>
                    </>
                  )}
                  {exportFormat === 'csv' && (
                    <>
                      <div className="font-medium text-text">CSV Format</div>
                      <div className="text-subtext-0">
                        Tabellenformat, kann in Excel geöffnet werden
                      </div>
                    </>
                  )}
                </div>
              </div>
            </div>

            <Button 
              onClick={handleExport} 
              isLoading={isExporting}
              className="w-full"
            >
              <Download className="w-4 h-4" />
              Exportieren
            </Button>
          </CardContent>
        </Card>

        {/* Import Card */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Upload className="w-5 h-5 text-secondary" />
              Daten importieren
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <p className="text-sm text-subtext-0">
              Importieren Sie Daten aus einer zuvor exportierten Datei. Unterstützt werden TOML, JSON und CSV.
            </p>

            <div className="p-4 rounded-lg border-2 border-dashed border-surface-1 text-center">
              <Upload className="w-8 h-8 text-subtext-0 mx-auto mb-2" />
              <p className="text-sm text-subtext-0 mb-4">
                Klicken Sie auf den Button, um eine Datei auszuwählen
              </p>
              <Button 
                variant="secondary" 
                onClick={handleImport}
                isLoading={isImporting}
              >
                <Upload className="w-4 h-4" />
                Datei auswählen
              </Button>
            </div>

            {/* Warning */}
            <div className="p-3 rounded-lg bg-warning/10 border border-warning/30">
              <p className="text-sm text-warning">
                <strong>Achtung:</strong> Der Import überschreibt möglicherweise vorhandene Daten. 
                Erstellen Sie vorher eine Sicherung.
              </p>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Backup Info */}
      <Card>
        <CardHeader>
          <CardTitle>Automatische Sicherungen</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-subtext-0">
                Die Datenbank wird automatisch im Benutzerverzeichnis gespeichert.
              </p>
              <p className="text-xs text-overlay-1 mt-1 font-mono">
                ~/.local/share/schließfach-manager/lockers.db
              </p>
            </div>
            <Button variant="outline" size="sm">
              Datenbank-Pfad öffnen
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
