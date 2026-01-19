import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { MainLayout } from '@/components/layout/MainLayout';
import { Dashboard } from '@/pages/Dashboard';
import { RentalsOverview } from '@/pages/rentals/Overview';
import { ActiveRentals } from '@/pages/rentals/Active';
import { OverdueRentals } from '@/pages/rentals/Overdue';
import { RentWizard } from '@/components/wizards/RentWizard';
import { LockersManagement } from '@/pages/management/Lockers';
import { SettingsPage } from '@/pages/management/Settings';
import { ExportImport } from '@/pages/management/Export';

// Placeholder components for routes not yet implemented
function PlaceholderPage({ title }: { title: string }) {
  return (
    <div className="flex items-center justify-center h-full">
      <div className="text-center">
        <h2 className="text-2xl font-bold text-text mb-2">{title}</h2>
        <p className="text-subtext-0">Diese Seite wird noch implementiert.</p>
      </div>
    </div>
  );
}

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<MainLayout />}>
          {/* Dashboard */}
          <Route index element={<Dashboard />} />

          {/* Rentals */}
          <Route path="rentals">
            <Route path="overview" element={<RentalsOverview />} />
            <Route path="new" element={<RentWizard />} />
            <Route path="active" element={<ActiveRentals />} />
            <Route path="overdue" element={<OverdueRentals />} />
            <Route path="extend" element={<PlaceholderPage title="Verleih verlängern" />} />
            <Route path="return" element={<PlaceholderPage title="Rückgabe" />} />
            <Route path="history" element={<PlaceholderPage title="Verleih-Verlauf" />} />
          </Route>

          {/* Finances */}
          <Route path="finances">
            <Route path="overview" element={<PlaceholderPage title="Finanz-Übersicht" />} />
            <Route path="payments" element={<PlaceholderPage title="Zahlungen" />} />
            <Route path="reports" element={<PlaceholderPage title="Berichte" />} />
          </Route>

          {/* Management */}
          <Route path="management">
            <Route path="lockers" element={<LockersManagement />} />
            <Route path="locations" element={<PlaceholderPage title="Standorte" />} />
            <Route path="settings" element={<SettingsPage />} />
            <Route path="audit" element={<PlaceholderPage title="Audit-Log" />} />
            <Route path="export" element={<ExportImport />} />
          </Route>
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
