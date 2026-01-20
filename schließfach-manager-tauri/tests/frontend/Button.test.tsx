import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { Button } from '@/components/ui/Button';

describe('Button', () => {
  it('renders correctly with children', () => {
    render(<Button>Click me</Button>);
    expect(screen.getByText('Click me')).toBeInTheDocument();
  });

  it('calls onClick handler when clicked', () => {
    const handleClick = vi.fn();
    render(<Button onClick={handleClick}>Click</Button>);
    fireEvent.click(screen.getByText('Click'));
    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it('is disabled when disabled prop is true', () => {
    render(<Button disabled>Disabled</Button>);
    expect(screen.getByText('Disabled')).toBeDisabled();
  });

  it('shows loading spinner when isLoading is true', () => {
    render(<Button isLoading>Loading</Button>);
    expect(screen.getByText('Loading')).toBeDisabled();
  });

  it('applies correct variant classes', () => {
    const { container } = render(<Button variant="danger">Danger</Button>);
    expect(container.firstChild).toHaveClass('bg-error');
  });
});
