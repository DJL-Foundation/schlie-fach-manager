import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import * as api from '@/lib/tauri';
import type { CreateLockerInput, UpdateLockerInput } from '@/types';

export function useLockers() {
  return useQuery({
    queryKey: ['lockers'],
    queryFn: api.getAllLockers,
  });
}

export function useAvailableLockers() {
  return useQuery({
    queryKey: ['lockers', 'available'],
    queryFn: api.getAvailableLockers,
  });
}

export function useLocker(id: number) {
  return useQuery({
    queryKey: ['locker', id],
    queryFn: () => api.getLocker(id),
    enabled: id > 0,
  });
}

export function useCreateLocker() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: CreateLockerInput) => api.createLocker(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['lockers'] });
      toast.success('Schließfach erfolgreich erstellt');
    },
    onError: (error: Error) => {
      toast.error(`Fehler beim Erstellen: ${error.message}`);
    },
  });
}

export function useUpdateLocker() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: UpdateLockerInput) => api.updateLocker(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['lockers'] });
      toast.success('Schließfach erfolgreich aktualisiert');
    },
    onError: (error: Error) => {
      toast.error(`Fehler beim Aktualisieren: ${error.message}`);
    },
  });
}

export function useDeleteLocker() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: number) => api.deleteLocker(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['lockers'] });
      toast.success('Schließfach erfolgreich gelöscht');
    },
    onError: (error: Error) => {
      toast.error(`Fehler beim Löschen: ${error.message}`);
    },
  });
}
