export default function StepIndicator({ totalSteps, currentStep, onChange }) {
  return (
    <div className="step-indicator">
      {Array.from({ length: totalSteps }).map((_, index) => (
        <button
          key={index}
          onClick={() => onChange(index)}
          className={`step-dot ${index === currentStep ? 'active' : ''}`}
          aria-label={`Go to step ${index + 1}`}
        />
      ))}
    </div>
  );
}
