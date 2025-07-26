import { useMemo } from 'react';
import { useExerciseData } from '../../../hooks/useExerciseData';
import './AngleIndicator.css';

export default function AngleIndicator({ joint, currentAngle, isError }) {
  const { getExerciseData } = useExerciseData();
  const exerciseData = getExerciseData();
  
  const angleData = useMemo(() => {
    if (!exerciseData?.idealAngles) return null;
    return exerciseData.idealAngles.find(a => a.joint === joint);
  }, [joint, exerciseData]);

  if (!angleData || currentAngle === undefined) return null;

  const percentage = Math.min(100, Math.max(0, 
    ((currentAngle - angleData.min) / (angleData.max - angleData.min)) * 100
  ));

  const isInRange = currentAngle >= angleData.min && currentAngle <= angleData.max;
  const deviation = Math.abs(currentAngle - angleData.perfect);
  const normalizedDeviation = Math.min(100, deviation / 1.8); // Scale for visualization

  return (
    <div className={`angle-indicator ${isError ? 'error' : ''}`}>
      <div className="angle-header">
        <span className="joint-name">{joint.replace('_', ' ')}</span>
        <span className="angle-value">{currentAngle.toFixed(1)}°</span>
      </div>
      
      <div className="angle-range-bar">
        <div 
          className="angle-current-marker"
          style={{
            left: `${percentage}%`,
            backgroundColor: isInRange ? '#4CAF50' : '#F44336'
          }}
        />
        <div className="angle-min">{angleData.min}°</div>
        <div className="angle-perfect" style={{ left: `${(angleData.perfect - angleData.min) / (angleData.max - angleData.min) * 100}%` }}>
          {angleData.perfect}°
        </div>
        <div className="angle-max">{angleData.max}°</div>
      </div>

      <div className="deviation-indicator">
        <div 
          className="deviation-bar" 
          style={{
            width: `${normalizedDeviation}%`,
            backgroundColor: isInRange ? 
              `hsl(${120 - (normalizedDeviation * 1.2)}, 70%, 50%)` : 
              '#F44336'
          }}
        />
        <span>Deviation: {deviation.toFixed(1)}°</span>
      </div>

      {!isInRange && (
        <div className="angle-warning">
          Target: {angleData.min}°-{angleData.max}°
        </div>
      )}
    </div>
  );
}
