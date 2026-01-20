import '@testing-library/jest-dom';

// Mock Tauri APIs for testing
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));
