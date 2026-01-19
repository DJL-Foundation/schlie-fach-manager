import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import * as api from '@/lib/tauri';

export function useSettings() {
  return useQuery({
    queryKey: ['settings'],
    queryFn: api.getSettings,
  });
}

export function useUpdateSetting() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ key, value }: { key: string; value: string }) =>
      api.updateSetting(key, value),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['settings'] });
      toast.success('Einstellung gespeichert');
    },
    onError: (error: Error) => {
      toast.error(`Fehler beim Speichern: ${error.message}`);
    },
  });
}

export function useAuditLog(limit?: number) {
  return useQuery({
    queryKey: ['auditLog', limit],
    queryFn: () => api.getAuditLog(limit),
  });
}
