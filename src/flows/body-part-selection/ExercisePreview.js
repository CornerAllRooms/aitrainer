import React from 'react';
import PropTypes from 'prop-types';
import './DifficultyTabs.css'; // Reusing the same CSS file

const ExercisePreview = ({ 
  exercise, 
  onSelect, 
  onClose,
  isActive = false
}) => {
  if (!exercise) return null;

  return (
    <div className={`exercise-preview ${isActive ? 'active' : ''}`}>
      <div className="preview-header">
        <h3 className="preview-title">{exercise.name}</h3>
        <button 
          className="close-button" 
          onClick={onClose}
          aria-label="Close preview"
        >
          &times;
        </button>
      </div>

      <div className="preview-content">
        <div className="preview-meta">
          <span className={`difficulty ${exercise.difficulty.toLowerCase()}`}>
            {exercise.difficulty}
          </span>
          <span className="equipment">
            {exercise.equipment.join(', ')}
          </span>
        </div>

        <div className="preview-description">
          <p>{exercise.description || 'No description available.'}</p>
        </div>

        <div className="preview-muscles">
          <h4>Target Muscles:</h4>
          <div className="muscle-tags">
            {exercise.primaryMuscles.map(muscle => (
              <span key={`primary-${muscle}`} className="muscle-tag primary">
                {muscle}
              </span>
            ))}
            {exercise.secondaryMuscles.map(muscle => (
              <span key={`secondary-${muscle}`} className="muscle-tag secondary">
                {muscle}
              </span>
            ))}
          </div>
        </div>
      </div>

      <div className="preview-actions">
        <button 
          className="select-button"
          onClick={() => onSelect(exercise)}
        >
          Select Exercise
        </button>
      </div>
    </div>
  );
};

ExercisePreview.propTypes = {
  exercise: PropTypes.shape({
    id: PropTypes.string.isRequired,
    name: PropTypes.string.isRequired,
    difficulty: PropTypes.oneOf(['Beginner', 'Intermediate', 'Advanced']),
    equipment: PropTypes.arrayOf(PropTypes.string),
    description: PropTypes.string,
    primaryMuscles: PropTypes.arrayOf(PropTypes.string),
    secondaryMuscles: PropTypes.arrayOf(PropTypes.string)
  }),
  onSelect: PropTypes.func.isRequired,
  onClose: PropTypes.func.isRequired,
  isActive: PropTypes.bool
};

export default ExercisePreview;