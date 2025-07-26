import { useState, useEffect } from 'react';
import { useExerciseStore } from '@/state/useExerciseStore';
import { useWasm } from './WASMcontext/useWasm';
import SetCounter from '@/components/Exercises/SetCounter';
import RestTimer from '@/components/Exercises/RestTimer';

export default function SetTracker({ exerciseId }) {
  const {
    currentSet,
    sets,
    reps,
    restTime,
    completeSet,
    startRestPeriod,
    endRestPeriod,
    updateExerciseStats
  } = useExerciseStore();
  const { validator, validatePose } = useWasm(exerciseId);
  const [activeReps, setActiveReps] = useState(0);
  const [isResting, setIsResting] = useState(false);

  // Handle rep validation
  const handleValidRep = () => {
    const newRepCount = activeReps + 1;
    setActiveReps(newRepCount);
    
    if (newRepCount >= reps) {
      completeSet();
      startRestPeriod();
      setIsResting(true);
    }
  };

  // Connect with WASM validator
  useEffect(() => {
    if (!validator) return;
    
    // In a real implementation, this would come from camera validation
    // For now simulating rep completion every 3 seconds
    const interval = setInterval(() => {
      if (!isResting && activeReps < reps) {
        handleValidRep();
      }
    }, 3000);

    return () => clearInterval(interval);
  }, [validator, activeReps, reps, isResting]);
  
  // Handle rest period completion
  const handleRestEnd = () => {
    endRestPeriod();
    setIsResting(false);
    setActiveReps(0);
  };

  // Update exercise stats when sets are completed
  useEffect(() => {
    if (currentSet > sets) {
      updateExerciseStats({
        exerciseId,
        totalReps: sets * reps,
        dateCompleted: new Date().toISOString()
      });
    }
  }, [currentSet, sets, reps, exerciseId, updateExerciseStats]);

  return (
    <div className="set-tracker">
      <div className="set-display">
        <SetCounter 
          current={currentSet} 
          total={sets} 
          isResting={isResting}
        />
        <div className="rep-display">
          {activeReps}/{reps} reps
        </div>
      </div>

      {isResting && (
        <RestTimer 
          duration={restTime} 
          onComplete={handleRestEnd}
        />
      )}

      <div className="validation-feedback">
        {/* This would display real-time form feedback */}
        {activeReps > 0 && (
          <div className="form-accuracy">
            Last rep form accuracy: 92%
          </div>
        )}
      </div>
    </div>
  );
}