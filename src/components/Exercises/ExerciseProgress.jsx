import './ExerciseComponents.css';
import { calculateNextWeight } from '@/utils/workoutMath/progression';
function WorkoutScreen() {
  return (
    <ExerciseExecution exerciseId={currentExerciseId} />
  );
}

export default function ExerciseProgress({ exercise }) {
  return (
    <div className="exercise-progress">
      <h3>{exercise.name}</h3>
      <div className="muscle-targets">
        {exercise.targetMuscles.join(', ')}
      </div>
      <div className="current-stats">
        <span>{exercise.sets} sets</span>
        <span>{exercise.reps} reps</span>
        <span>{exercise.restTime}s rest</span>
      </div>
    </div>
  );
}
const nextWorkoutWeight = calculateNextWeight({
  currentWeight: 100,
  currentReps: 8,
  targetReps: 12
});