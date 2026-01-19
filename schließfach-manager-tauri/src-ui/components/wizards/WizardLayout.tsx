import { useState, type ReactNode } from 'react';
import { Check, ChevronLeft, ChevronRight } from 'lucide-react';
import { Button } from '@/components/ui/Button';
import { cn } from '@/lib/cn';
import type { WizardStep, WizardStepProps } from '@/types';

interface WizardLayoutProps {
  title: string;
  steps: WizardStep[];
  onComplete: (data: Record<string, unknown>) => Promise<void>;
  onCancel: () => void;
}

export function WizardLayout({
  title,
  steps,
  onComplete,
  onCancel,
}: WizardLayoutProps) {
  const [currentStep, setCurrentStep] = useState(0);
  const [data, setData] = useState<Record<string, unknown>>({});
  const [isSubmitting, setIsSubmitting] = useState(false);

  const isFirstStep = currentStep === 0;
  const isLastStep = currentStep === steps.length - 1;

  const goNext = () => {
    if (!isLastStep) {
      setCurrentStep((prev) => prev + 1);
    }
  };

  const goPrev = () => {
    if (!isFirstStep) {
      setCurrentStep((prev) => prev - 1);
    }
  };

  const updateData = (newData: Record<string, unknown>) => {
    setData((prev) => ({ ...prev, ...newData }));
  };

  const handleComplete = async () => {
    try {
      setIsSubmitting(true);
      await onComplete(data);
    } finally {
      setIsSubmitting(false);
    }
  };

  const StepComponent = steps[currentStep].component;

  return (
    <div className="max-w-3xl mx-auto space-y-6 animate-fade-in">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold text-text">{title}</h1>
      </div>

      {/* Step Indicator */}
      <div className="flex items-center gap-2">
        {steps.map((step, index) => {
          const isCompleted = index < currentStep;
          const isCurrent = index === currentStep;

          return (
            <div key={step.id} className="flex items-center flex-1">
              {/* Step Circle */}
              <div
                className={cn(
                  'w-10 h-10 rounded-full flex items-center justify-center flex-shrink-0 transition-colors',
                  isCompleted
                    ? 'bg-success text-base'
                    : isCurrent
                    ? 'bg-primary text-base'
                    : 'bg-surface-1 text-subtext-0'
                )}
              >
                {isCompleted ? (
                  <Check className="w-5 h-5" />
                ) : (
                  <span className="font-medium">{index + 1}</span>
                )}
              </div>

              {/* Step Info */}
              <div className="ml-3 flex-1">
                <div
                  className={cn(
                    'text-sm font-medium',
                    isCurrent ? 'text-text' : 'text-subtext-0'
                  )}
                >
                  {step.title}
                </div>
                <div className="text-xs text-subtext-0">{step.description}</div>
              </div>

              {/* Connector Line */}
              {index < steps.length - 1 && (
                <div
                  className={cn(
                    'h-0.5 flex-shrink-0 w-8 mx-2',
                    isCompleted ? 'bg-success' : 'bg-surface-1'
                  )}
                />
              )}
            </div>
          );
        })}
      </div>

      {/* Step Content */}
      <div className="bg-surface-0 rounded-lg border border-surface-1 p-6 min-h-[400px]">
        <StepComponent
          data={data}
          updateData={updateData}
          goNext={goNext}
          goPrev={goPrev}
        />
      </div>

      {/* Navigation */}
      <div className="flex items-center justify-between">
        <Button variant="ghost" onClick={onCancel}>
          Abbrechen
        </Button>

        <div className="flex gap-2">
          {!isFirstStep && (
            <Button variant="outline" onClick={goPrev}>
              <ChevronLeft className="w-4 h-4" />
              Zurück
            </Button>
          )}

          {isLastStep ? (
            <Button onClick={handleComplete} isLoading={isSubmitting}>
              <Check className="w-4 h-4" />
              Abschließen
            </Button>
          ) : (
            <Button onClick={goNext}>
              Weiter
              <ChevronRight className="w-4 h-4" />
            </Button>
          )}
        </div>
      </div>
    </div>
  );
}
