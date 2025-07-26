import { useState, useEffect } from 'react';
import StepIndicator from './components/StepIndicator';
import MuscleMap from './MuscleMap';

export default function DemoView({ exercise }) {
  const [currentStep, setCurrentStep] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);

  // Get muscles emphasized in current step
  const getStepMuscles = () => {
    if (!exercise.steps[currentStep].targetMuscles) {
      return exercise.targetMuscles;
    }
    return exercise.steps[currentStep].targetMuscles || exercise.targetMuscles;
  };

  useEffect(() => {
    let interval;
    if (isPlaying && exercise.steps.length > 1) {
      interval = setInterval(() => {
        setCurrentStep(prev => (prev + 1) % exercise.steps.length);
      }, 3000);
    }
    return () => clearInterval(interval);
  }, [isPlaying, exercise.steps.length]);

  return (
    <div className="demo-container">
      <div className="exercise-demo">
        <h3>{exercise.name}</h3>
        <div className="step-display">
          <p>{exercise.steps[currentStep]}</p>
          <StepIndicator 
            totalSteps={exercise.steps.length} 
            currentStep={currentStep}
            onChange={setCurrentStep}
          />
        </div>
        <button onClick={() => setIsPlaying(!isPlaying)}>
          {isPlaying ? 'Pause' : 'Play'}
        </button>
      </div>

      <MuscleMap 
        targetMuscles={getStepMuscles()} 
        currentStep={currentStep}
      />
    </div>
  );
}
