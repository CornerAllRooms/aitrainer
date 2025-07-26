import { muscleData } from './assets/muscleData';

const ourMuscleGroups = [
  'shoulders', 'chest', 'abs', 'back', 
  'biceps', 'triceps', 'quads', 
  'hamstrings', 'glutes', 'calves'
];

export default function MuscleMap({ targetMuscles, currentStep }) {
  // Filter to only show muscles we've defined
  const filteredMuscles = Object.fromEntries(
    Object.entries(muscleData.muscles).filter(
      ([name]) => ourMuscleGroups.some(group => name.includes(group))
    )
  );

  const getIntensity = (muscle) => {
    const isPrimary = targetMuscles.some(target => muscle.includes(target));
    return isPrimary ? 0.8 : 0.1;
  };

  return (
    <svg viewBox="0 0 500 800" className="muscle-map">
      <path d={muscleData.outline} fill="#f0f0f0" stroke="#ccc" />
      
      {Object.entries(filteredMuscles).map(([name, path]) => (
        <path
          key={name}
          d={path}
          fill={`rgba(255, 100, 100, ${getIntensity(name)})`}
          stroke={`rgba(200, 60, 60, ${getIntensity(name) + 0.2})`}
        />
      ))}

      {targetMuscles.map(muscle => (
        muscleData.labels[muscle] && (
          <text
            key={`label-${muscle}`}
            x={muscleData.labels[muscle].x}
            y={muscleData.labels[muscle].y}
            className="muscle-label"
          >
            {muscle.replace(/_/g, ' ')}
          </text>
        )
      ))}
    </svg>
  );
}
