import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { muscleGroups } from '../../data/muscleGroups';
import { createValidator } from '../../libs/wasm/adapter';
import './BodyPartSelector.css';

export default function BodyPartSelector() {
  const [selectedGroup, setSelectedGroup] = useState(null);
  const [exercises, setExercises] = useState([]);
  const [validatorReady, setValidatorReady] = useState(false);
  const navigate = useNavigate();

  useEffect(() => {
    const loadValidator = async () => {
      try {
        // Test WASM validator with first muscle group
        await createValidator('biceps');
        setValidatorReady(true);
      } catch (error) {
        console.error('Validator initialization failed:', error);
        setValidatorReady(true); // Continue with JS fallback
      }
    };
    loadValidator();
  }, []);

  useEffect(() => {
    if (selectedGroup) {
      import(`../../data/exercises/${selectedGroup}.json`)
        .then(module => {
          setExercises(Object.values(module.exercises));
        })
        .catch(err => console.error('Failed to load exercises:', err));
    }
  }, [selectedGroup]);

  const handleExerciseSelect = (exerciseId) => {
    if (validatorReady) {
      navigate(`/exercise/${exerciseId}`);
    }
  };

  return (
    <div className="selector-container">
      <h2>Select Muscle Group</h2>
      <div className="muscle-grid">
        {muscleGroups.map(group => (
          <div 
            key={group.id}
            className={`muscle-card ${selectedGroup === group.id ? 'selected' : ''}`}
            onClick={() => setSelectedGroup(group.id)}
          >
            <img src={group.image} alt={group.name} />
            <p>{group.name}</p>
          </div>
        ))}
      </div>

      {selectedGroup && (
        <div className="exercise-list">
          <h3>Available Exercises</h3>
          <div className="exercise-grid">
            {exercises.map(exercise => (
              <div
                key={exercise.name}
                className="exercise-card"
                onClick={() => handleExerciseSelect(
                  exercise.name.toLowerCase().replace(/\s+/g, '-')
                )}
              >
                <p>{exercise.name}</p>
                <div className="exercise-meta">
                  <span>Equipment: {exercise.equipment.join(', ')}</span>
                  <span>Targets: {exercise.targetMuscles.join(', ')}</span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
