import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import * as api from '@/lib/tauri';
import type { CreateRentalInput } from '@/types';

export function useRentals() {
  return useQuery({
    queryKey: ['rentals'],
    queryFn: api.getAllRentals,
  });
}

export function useActiveRentals() {
  return useQuery({
    queryKey: ['rentals', 'active'],
    queryFn: api.getActiveRentals,
  });
}

export function useOverdueRentals() {
  return useQuery({
    queryKey: ['rentals', 'overdue'],
    queryFn: api.getOverdueRentals,
  });
}

export function useRental(id: number) {
  return useQuery({
    queryKey: ['rental', id],
    queryFn: () => api.getRental(id),
    enabled: id > 0,
  });
}

export function useCreateRental() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: CreateRentalInput) => api.createRental(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['rentals'] });
      queryClient.invalidateQueries({ queryKey: ['lockers'] });
      queryClient.invalidateQueries({ queryKey: ['dashboard'] });
      toast.success('Verleih erfolgreich erstellt');
    },
    onError: (error: Error) => {
      toast.error(`Fehler beim Erstellen: ${error.message}`);
    },
  });
}

export function useExtendRental() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, months }: { id: number; months: number }) =>
      api.extendRental(id, months),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['rentals'] });
      toast.success('Verleih erfolgreich verlängert');
    },
    onError: (error: Error) => {
      toast.error(`Fehler beim Verlängern: ${error.message}`);
    },
  });
}

export function useReturnRental() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: number) => api.returnRental(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['rentals'] });
      queryClient.invalidateQueries({ queryKey: ['lockers'] });
      queryClient.invalidateQueries({ queryKey: ['dashboard'] });
      toast.success('Rückgabe erfolgreich verarbeitet');
    },
    onError: (error: Error) => {
      toast.error(`Fehler bei Rückgabe: ${error.message}`);
    },
  });
}
