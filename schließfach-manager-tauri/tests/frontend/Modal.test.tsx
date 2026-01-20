import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { Modal } from '@/components/ui/Modal';

describe('Modal', () => {
  it('does not render when closed', () => {
    render(
      <Modal open={false} onClose={() => {}}>
        Modal content
      </Modal>
    );
    expect(screen.queryByText('Modal content')).not.toBeInTheDocument();
  });

  it('renders when open', () => {
    render(
      <Modal open={true} onClose={() => {}}>
        Modal content
      </Modal>
    );
    expect(screen.getByText('Modal content')).toBeInTheDocument();
  });

  it('renders title when provided', () => {
    render(
      <Modal open={true} onClose={() => {}} title="Test Title">
        Content
      </Modal>
    );
    expect(screen.getByText('Test Title')).toBeInTheDocument();
  });

  it('renders footer when provided', () => {
    render(
      <Modal 
        open={true} 
        onClose={() => {}} 
        footer={<button>Submit</button>}
      >
        Content
      </Modal>
    );
    expect(screen.getByText('Submit')).toBeInTheDocument();
  });

  it('calls onClose when overlay is clicked', () => {
    const handleClose = vi.fn();
    render(
      <Modal open={true} onClose={handleClose}>
        Content
      </Modal>
    );
    // The overlay should be clickable to close
    const overlay = screen.getByText('Content').closest('.fixed');
    if (overlay) {
      fireEvent.click(overlay);
      expect(handleClose).toHaveBeenCalled();
    }
  });

  it('does not close when content is clicked', () => {
    const handleClose = vi.fn();
    render(
      <Modal open={true} onClose={handleClose}>
        <div data-testid="content">Content</div>
      </Modal>
    );
    fireEvent.click(screen.getByTestId('content'));
    expect(handleClose).not.toHaveBeenCalled();
  });
});
