import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { MainLayout } from '@/components/layout/MainLayout';
import { Dashboard } from '@/pages/Dashboard';
import { RentalsOverview } from '@/pages/rentals/Overview';
import { ActiveRentals } from '@/pages/rentals/Active';
import { OverdueRentals } from '@/pages/rentals/Overdue';
import { ExtendRental } from '@/pages/rentals/Extend';
import { ReturnRental } from '@/pages/rentals/Return';
import { RentalHistory } from '@/pages/rentals/History';
import { RentWizard } from '@/components/wizards/RentWizard';
import { FinancesOverview } from '@/pages/finances/Overview';
import { Payments } from '@/pages/finances/Payments';
import { Reports } from '@/pages/finances/Reports';
import { LockersManagement } from '@/pages/management/Lockers';
import { LocationsManagement } from '@/pages/management/Locations';
import { SettingsPage } from '@/pages/management/Settings';
import { AuditLog } from '@/pages/management/AuditLog';
import { ExportImport } from '@/pages/management/Export';
import { ShortcutsOverlay } from '@/components/ShortcutsOverlay';
import { useGlobalShortcuts, useShortcutsOverlay } from '@/hooks';

function AppContent() {
  const { isOpen, open, close } = useShortcutsOverlay();
  useGlobalShortcuts(open);

  return (
    <>
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
            <Route path="extend" element={<ExtendRental />} />
            <Route path="return" element={<ReturnRental />} />
            <Route path="history" element={<RentalHistory />} />
          </Route>

          {/* Finances */}
          <Route path="finances">
            <Route path="overview" element={<FinancesOverview />} />
            <Route path="payments" element={<Payments />} />
            <Route path="reports" element={<Reports />} />
          </Route>

          {/* Management */}
          <Route path="management">
            <Route path="lockers" element={<LockersManagement />} />
            <Route path="locations" element={<LocationsManagement />} />
            <Route path="settings" element={<SettingsPage />} />
            <Route path="audit" element={<AuditLog />} />
            <Route path="export" element={<ExportImport />} />
          </Route>
        </Route>
      </Routes>
      <ShortcutsOverlay open={isOpen} onClose={close} />
    </>
  );
}

function App() {
  return (
    <BrowserRouter>
      <AppContent />
    </BrowserRouter>
  );
}

export default App;
