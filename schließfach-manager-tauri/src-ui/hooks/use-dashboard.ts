import { useQuery } from '@tanstack/react-query';
import * as api from '@/lib/tauri';

export function useDashboard() {
  return useQuery({
    queryKey: ['dashboard'],
    queryFn: api.getDashboardData,
    staleTime: 30000, // 30 seconds
    refetchInterval: 60000, // Refetch every minute
  });
}

export function useStatusBar() {
  return useQuery({
    queryKey: ['statusBar'],
    queryFn: api.getStatusBarData,
    staleTime: 30000,
    refetchInterval: 30000, // Refetch every 30 seconds
  });
}
