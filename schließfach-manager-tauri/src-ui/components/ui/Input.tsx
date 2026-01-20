import { forwardRef, type InputHTMLAttributes } from 'react';
import { cn } from '@/lib/cn';

export interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
  helperText?: string;
}

export const Input = forwardRef<HTMLInputElement, InputProps>(
  ({ className, label, error, helperText, type = 'text', ...props }, ref) => {
    return (
      <div className="space-y-2">
        {label && (
          <label className="block text-sm font-medium text-subtext-1">
            {label}
            {props.required && <span className="text-error ml-1">*</span>}
          </label>
        )}

        <input
          ref={ref}
          type={type}
          className={cn(
            `w-full px-3 py-2 rounded-md border bg-base text-text
             placeholder:text-overlay-1 transition-colors
             focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent`,
            error
              ? 'border-error focus:ring-error'
              : 'border-surface-1 focus:ring-primary',
            className
          )}
          {...props}
        />

        {error && <p className="text-sm text-error">{error}</p>}

        {!error && helperText && (
          <p className="text-sm text-subtext-0">{helperText}</p>
        )}
      </div>
    );
  }
);

Input.displayName = 'Input';
