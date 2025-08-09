import React, { useEffect, useRef, useState } from 'react';
import { usePoseAnalysis } from '@/context/PoseContext';
import AngleIndicator from '@/components/Camera/components/AngleIndicator';
import ExerciseHUD from '@/components/Skeleton/ExerciseHUD';
import { useAudioFeedback } from '@/feedback/audio/GoogleTTS';
import './CameraSetup.css'; // Reusing your shared styles
import { useWorkout } from '@/state/WorkoutContext';
import { useWasm } from '@/libs/wasm/WasmContext';
import AngleIndicator from '@/components/Camera/components/AngleIndicator';

const RealTimeFeedback = ({ 
  exercise,
  onRepComplete,
  onFormError,
  onExerciseComplete
}) => {
  // Context integration
  const { currentPose, analysis } = usePoseAnalysis();
  const { speak } = useAudioFeedback();
  const feedbackRef = useRef(null);
  
  // State for rep counting and timing
  const [repCount, setRepCount] = useState(0);
  const [activeFeedback, setActiveFeedback] = useState(null);
  const lastRepTime = useRef(0);
  const repTimeout = useRef(null);

  // WASM integration effects
  useEffect(() => {
    if (!analysis) return;

    // Handle rep counting
    if (analysis.repDetected && Date.now() - lastRepTime.current > 1000) {
      lastRepTime.current = Date.now();
      setRepCount(prev => prev + 1);
      onRepComplete?.(repCount + 1);
      
      // Visual feedback for rep completion
      setActiveFeedback({
        type: 'success',
        message: 'Good rep!'
      });
      
      // Audio feedback
      speak('Good repetition');
    }

    // Handle form errors
    if (analysis.formErrors.length > 0) {
      const primaryError = analysis.formErrors[0];
      setActiveFeedback({
        type: 'error',
        message: primaryError
      });
      
      onFormError?.(primaryError);
      speak(primaryError);
    }

    // Clear feedback after delay
    if (activeFeedback) {
      clearTimeout(repTimeout.current);
      repTimeout.current = setTimeout(() => {
        setActiveFeedback(null);
      }, 3000);
    }

    return () => clearTimeout(repTimeout.current);
  }, [analysis]);

  // Exercise completion logic
  useEffect(() => {
    if (exercise?.targetReps && repCount >= exercise.targetReps) {
      onExerciseComplete?.(repCount);
      speak('Exercise complete! Great work!');
    }
  }, [repCount, exercise]);

  // Key joint angles to display
  const criticalJoints = exercise?.formCheckpoints || [
    { joint: ['shoulder', 'elbow', 'wrist'], name: 'Elbow Angle' },
    { joint: ['hip', 'knee', 'ankle'], name: 'Knee Angle' }
  ];

  return (
    <div className="feedback-container">
      {/* Exercise HUD Integration */}
      <ExerciseHUD 
        exercise={exercise} 
        currentStep={0} 
        repCount={repCount}
        onMenuToggle={() => {}}
      />

      {/* Visual Feedback Display */}
      <div 
        ref={feedbackRef}
        className={`feedback-banner ${activeFeedback?.type || ''}`}
      >
        {activeFeedback?.message}
      </div>

      {/* Angle Indicators Grid */}
      <div className="angle-grid">
        {criticalJoints.map((jointData, index) => {
          const angle = analysis?.angles[jointData.name] || 0;
          const isError = analysis?.formErrors.some(e => 
            e.includes(jointData.name)
          );

          return (
            <AngleIndicator
              key={`joint-${index}`}
              joint={jointData.name}
              currentAngle={angle}
              isError={isError}
            />
          );
        })}
      </div>

      {/* WASM Performance Metrics */}
      {analysis && (
        <div className="performance-metrics">
          <span className="metric engagement">
            Engagement: {(analysis.engagement * 100).toFixed(0)}%
          </span>
          <span className="metric fps">
            Analysis: {analysis.processingFPS.toFixed(1)} FPS
          </span>
        </div>
      )}
    </div>
  );
};

RealTimeFeedback.propTypes = {
  exercise: PropTypes.shape({
    id: PropTypes.string,
    name: PropTypes.string,
    targetReps: PropTypes.number,
    formCheckpoints: PropTypes.arrayOf(
      PropTypes.shape({
        joint: PropTypes.arrayOf(PropTypes.string),
        name: PropTypes.string
      })
    )
  }),
  onRepComplete: PropTypes.func,
  onFormError: PropTypes.func,
  onExerciseComplete: PropTypes.func
};

export default RealTimeFeedback;