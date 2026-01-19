import { forwardRef, type ButtonHTMLAttributes } from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '@/lib/cn';

const buttonVariants = cva(
  `inline-flex items-center justify-center gap-2 rounded-md font-medium
   transition-colors focus:outline-none focus:ring-2 focus:ring-primary
   focus:ring-offset-2 focus:ring-offset-base disabled:opacity-50
   disabled:pointer-events-none`,
  {
    variants: {
      variant: {
        primary: 'bg-primary text-base hover:bg-primary/90',
        secondary: 'bg-secondary text-base hover:bg-secondary/90',
        outline: 'border border-surface-1 bg-transparent text-text hover:bg-surface-0',
        ghost: 'bg-transparent text-text hover:bg-surface-0',
        danger: 'bg-error text-base hover:bg-error/90',
      },
      size: {
        sm: 'h-8 px-3 text-sm',
        md: 'h-10 px-4 text-base',
        lg: 'h-12 px-6 text-lg',
        icon: 'h-10 w-10',
      },
    },
    defaultVariants: {
      variant: 'primary',
      size: 'md',
    },
  }
);

export interface ButtonProps
  extends ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  isLoading?: boolean;
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, isLoading, children, disabled, ...props }, ref) => {
    return (
      <button
        ref={ref}
        className={cn(buttonVariants({ variant, size }), className)}
        disabled={isLoading || disabled}
        {...props}
      >
        {isLoading && (
          <div className="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin" />
        )}
        {children}
      </button>
    );
  }
);

Button.displayName = 'Button';
