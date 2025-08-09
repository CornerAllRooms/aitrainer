import React, { useState, useEffect, useMemo } from 'react';
import { muscleData } from './assets/muscleData';
import exercises from '../../data/exercises';
import './assets/mm.css';

// Dynamically import all muscle images
const muscleImages = {
  shoulders: require('../../data/assets/muscles/shoulders.png'),
  chest: require('../../data/assets/muscles/chest.png'),
  abs: require('../../data/assets/muscles/abs.png'),
  back: require('../../data/assets/muscles/back.png'),
  biceps: require('../../data/assets/muscles/biceps.png'),
  triceps: require('../../data/assets/muscles/triceps.png'),
  quads: require('../../data/assets/muscles/quads.png'),
  hamstrings: require('../../data/assets/muscles/hamstrings.png'),
  glutes: require('../../data/assets/muscles/glutes.png'),
  calves: require('../../data/assets/muscles/calves.png')
};

const ourMuscleGroups = Object.keys(muscleImages);

export default function MuscleMap({ currentStep, onExerciseSelect }) {
  const [selectedMuscle, setSelectedMuscle] = useState(null);
  const [hoveredMuscle, setHoveredMuscle] = useState(null);
  const [isLoading, setIsLoading] = useState(false);

  // Combine and memoize all exercises
  const allExercises = useMemo(() => Object.values(exercises).flat(), []);

  // Filter and memoize muscles
  const filteredMuscles = useMemo(() => 
    Object.fromEntries(
      Object.entries(muscleData.muscles).filter(
        ([name]) => ourMuscleGroups.some(group => name.includes(group))
      )
    ),
  []);

  // Memoized exercises for selected muscle
  const muscleExercises = useMemo(() => {
    if (!selectedMuscle) return [];
    
    const muscleGroup = ourMuscleGroups.find(group => 
      selectedMuscle.toLowerCase().includes(group)
    );
    
    return allExercises.filter(exercise => 
      exercise.primaryMuscles?.includes(muscleGroup) ||
      exercise.secondaryMuscles?.includes(muscleGroup)
    ).sort((a, b) => a.difficulty.localeCompare(b.difficulty));
  }, [selectedMuscle, allExercises]);

  // Handle muscle selection with loading state
  const handleMuscleClick = (muscleName) => {
    setIsLoading(true);
    setSelectedMuscle(muscleName);
    setTimeout(() => setIsLoading(false), 300); // Simulate loading
  };

  // Get intensity for muscle highlighting
  const getIntensity = (muscle) => {
    if (selectedMuscle === muscle) return 0.9;
    if (hoveredMuscle === muscle) return 0.6;
    return 0.1;
  };

  // Get image for the selected muscle group
  const getMuscleImage = () => {
    if (!selectedMuscle) return null;
    const group = ourMuscleGroups.find(g => selectedMuscle.includes(g));
    return group ? muscleImages[group] : null;
  };

  return (
    <div className="muscle-map-container">
      <div className="muscle-visualization">
        <svg viewBox="0 0 500 800" className="muscle-svg">
          <path d={muscleData.outline} fill="#f0f0f0" stroke="#ccc" />
          
          {Object.entries(filteredMuscles).map(([name, path]) => (
            <path
              key={name}
              d={path}
              fill={`rgba(255, 100, 100, ${getIntensity(name)})`}
              stroke={`rgba(200, 60, 60, ${getIntensity(name) + 0.2})`}
              onClick={() => handleMuscleClick(name)}
              onMouseEnter={() => setHoveredMuscle(name)}
              onMouseLeave={() => setHoveredMuscle(null)}
              className="muscle-path"
            />
          ))}

          {ourMuscleGroups.map(muscle => (
            muscleData.labels[muscle] && (
              <text
                key={`label-${muscle}`}
                x={muscleData.labels[muscle].x}
                y={muscleData.labels[muscle].y}
                className="muscle-label"
                onClick={() => handleMuscleClick(muscle)}
                onMouseEnter={() => setHoveredMuscle(muscle)}
                onMouseLeave={() => setHoveredMuscle(null)}
              >
                {muscle.replace(/_/g, ' ')}
              </text>
            )
          ))}
        </svg>

        {selectedMuscle && (
          <div className="muscle-image-preview">
            <img 
              src={getMuscleImage()} 
              alt={selectedMuscle} 
              className="muscle-image"
            />
          </div>
        )}
      </div>

      {selectedMuscle && (
        <div className="exercise-panel">
          <div className="panel-header">
            <h3>
              {selectedMuscle.replace(/_/g, ' ')} Exercises
              {isLoading && <span className="loading-dots">...</span>}
            </h3>
          </div>
          
          {muscleExercises.length > 0 ? (
            <div className="exercise-grid">
              {muscleExercises.slice(0, 6).map(exercise => (
                <div key={exercise.id} className="exercise-card">
                  <div className="exercise-header">
                    <h4>{exercise.name}</h4>
                    <span className={`difficulty ${exercise.difficulty.toLowerCase()}`}>
                      {exercise.difficulty}
                    </span>
                  </div>
                  <div className="exercise-meta">
                    <span className="equipment">
                      {exercise.equipment.join(', ')}
                    </span>
                  </div>
                  <button 
                    className="select-button"
                    onClick={() => onExerciseSelect?.(exercise)}
                  >
                    Select
                  </button>
                </div>
              ))}
            </div>
          ) : (
            <div className="no-exercises">
              No exercises found for this muscle group
            </div>
          )}
        </div>
      )}
    </div>
  );
}