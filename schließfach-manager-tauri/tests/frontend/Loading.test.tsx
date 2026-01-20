import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { Loading, LoadingPage, LoadingOverlay } from '@/components/ui/Loading';

describe('Loading', () => {
  it('renders loading spinner', () => {
    const { container } = render(<Loading />);
    expect(container.firstChild).toHaveClass('animate-spin');
  });

  it('applies small size class', () => {
    const { container } = render(<Loading size="sm" />);
    expect(container.firstChild).toHaveClass('w-4', 'h-4');
  });

  it('applies medium size class', () => {
    const { container } = render(<Loading size="md" />);
    expect(container.firstChild).toHaveClass('w-8', 'h-8');
  });

  it('applies large size class', () => {
    const { container } = render(<Loading size="lg" />);
    expect(container.firstChild).toHaveClass('w-12', 'h-12');
  });

  it('applies custom className', () => {
    const { container } = render(<Loading className="custom" />);
    expect(container.firstChild).toHaveClass('custom');
  });
});

describe('LoadingPage', () => {
  it('renders loading page with text', () => {
    render(<LoadingPage />);
    expect(screen.getByText('Laden...')).toBeInTheDocument();
  });
});

describe('LoadingOverlay', () => {
  it('renders loading overlay with text', () => {
    render(<LoadingOverlay />);
    expect(screen.getByText('Bitte warten...')).toBeInTheDocument();
  });
});
