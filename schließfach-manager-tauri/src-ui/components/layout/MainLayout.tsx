import { Outlet } from 'react-router-dom';
import { TopBar } from './TopBar';
import { Sidebar } from './Sidebar';
import { StatusBar } from './StatusBar';
import { Toaster } from 'sonner';

export function MainLayout() {
  return (
    <div className="h-screen flex flex-col bg-base text-text">
      <TopBar />

      <div className="flex-1 flex overflow-hidden">
        <Sidebar />

        <main className="flex-1 overflow-y-auto p-6">
          <Outlet />
        </main>
      </div>

      <StatusBar />

      {/* Toast Notifications */}
      <Toaster
        position="top-right"
        toastOptions={{
          style: {
            background: 'var(--color-surface-0)',
            color: 'var(--color-text)',
            border: '1px solid var(--color-surface-1)',
          },
        }}
      />
    </div>
  );
}
