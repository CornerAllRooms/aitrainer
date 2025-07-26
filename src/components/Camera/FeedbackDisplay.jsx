import { useEffect, useState } from 'react';
import { createValidator } from '@/libs/wasm/adapter';
import AngleIndicator from './components/AngleIndicator';

export default function FeedbackDisplay({ exerciseId, pose }) {
  const [feedback, setFeedback] = useState(null);
  const [validator, setValidator] = useState(null);

  useEffect(() => {
    const init = async () => {
      const validator = await createValidator(exerciseId);
      setValidator(validator);
    };
    init();
  }, [exerciseId]);

  useEffect(() => {
    if (validator && pose) {
      const result = validator.validatePose(pose);
      setFeedback(result);
    }
  }, [validator, pose]);

  if (!feedback) return <div className="feedback-loading">Loading validator...</div>;

  return (
    <div className="feedback-container">
      <div className="rep-counter">Reps: {feedback.repCount}</div>
      {Object.entries(feedback.angles).map(([joint, angle]) => (
        <AngleIndicator
          key={joint}
          joint={joint}
          currentAngle={angle}
          isError={feedback.errors.some(e => e.joint === joint)}
        />
      ))}
    </div>
  );
}
