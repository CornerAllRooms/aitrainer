import { useState } from 'react';
import './styles/skeleton.css';

export default function ExerciseHUD({ exercise, currentStep, onMenuToggle }) {
  const [isExpanded, setIsExpanded] = useState(false);

  const toggleMenu = () => {
    setIsExpanded(!isExpanded);
    onMenuToggle(!isExpanded);
  };

  return (
    <div className={`hud-container ${isExpanded ? 'expanded' : ''}`}>
      {/* Hamburger Icon */}
      <div className="hamburger" onClick={toggleMenu}>
        <div className="hamburger-line"></div>
        <div className="hamburger-line"></div>
        <div className="hamburger-line"></div>
      </div>

      {/* Dashboard Content */}
      <div className="dashboard">
        <h2 className="exercise-title">{exercise?.name || 'Select Exercise'}</h2>
        
        {isExpanded && (
          <div className="exercise-details">
            <div className="current-step">
              Step {currentStep + 1}: {exercise?.steps[currentStep] || ''}
            </div>
            <div className="muscle-targets">
              {exercise?.targetMuscles?.map(muscle => (
                <span key={muscle} className="muscle-tag">{muscle}</span>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
