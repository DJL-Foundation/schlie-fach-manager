import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./tests/frontend/setup.ts'],
    include: ['tests/frontend/**/*.test.{ts,tsx}'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      include: ['src-ui/**/*.{ts,tsx}'],
      exclude: ['src-ui/**/*.d.ts', 'src-ui/types/**'],
    },
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src-ui'),
    },
  },
});
