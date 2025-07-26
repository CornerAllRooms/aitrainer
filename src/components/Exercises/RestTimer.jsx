import { useState, useEffect } from 'react';
import './ExerciseComponents.css';

export default function RestTimer({ duration, onComplete }) {
  const [secondsLeft, setSecondsLeft] = useState(duration);

  useEffect(() => {
    const timer = setInterval(() => {
      setSecondsLeft(prev => {
        if (prev <= 1) {
          clearInterval(timer);
          onComplete();
          return 0;
        }
        return prev - 1;
      });
    }, 1000);

    return () => clearInterval(timer);
  }, [duration, onComplete]);

  const formatTime = (seconds) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs < 10 ? '0' : ''}${secs}`;
  };

  return (
    <div className="rest-timer">
      <div className="timer-display">
        {formatTime(secondsLeft)}
      </div>
      <div className="timer-label">Rest Period</div>
    </div>
  );
}
