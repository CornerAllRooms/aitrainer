import './ExerciseComponents.css';

export default function SetCounter({ current, total, isResting }) {
  return (
    <div className={`set-counter ${isResting ? 'resting' : ''}`}>
      <span className="current-set">{current}</span>
      <span className="set-divider">/</span>
      <span className="total-sets">{total}</span>
      <span className="set-label">Sets</span>
      {isResting && <span className="rest-indicator">REST</span>}
    </div>
  );
}
