import { forwardRef, type SelectHTMLAttributes } from 'react';
import { ChevronDown } from 'lucide-react';
import { cn } from '@/lib/cn';

export interface SelectOption {
  value: string;
  label: string;
}

export interface SelectProps extends SelectHTMLAttributes<HTMLSelectElement> {
  label?: string;
  error?: string;
  options: SelectOption[];
  placeholder?: string;
}

export const Select = forwardRef<HTMLSelectElement, SelectProps>(
  ({ className, label, error, options, placeholder, ...props }, ref) => {
    return (
      <div className="space-y-2">
        {label && (
          <label className="block text-sm font-medium text-subtext-1">
            {label}
            {props.required && <span className="text-error ml-1">*</span>}
          </label>
        )}

        <div className="relative">
          <select
            ref={ref}
            className={cn(
              `w-full px-3 py-2 rounded-md border bg-base text-text
               appearance-none transition-colors cursor-pointer
               focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent`,
              error
                ? 'border-error focus:ring-error'
                : 'border-surface-1 focus:ring-primary',
              className
            )}
            {...props}
          >
            {placeholder && (
              <option value="" disabled>
                {placeholder}
              </option>
            )}
            {options.map((opt) => (
              <option key={opt.value} value={opt.value}>
                {opt.label}
              </option>
            ))}
          </select>

          <ChevronDown className="absolute right-3 top-1/2 -translate-y-1/2 w-4 h-4 text-overlay-1 pointer-events-none" />
        </div>

        {error && <p className="text-sm text-error">{error}</p>}
      </div>
    );
  }
);

Select.displayName = 'Select';
