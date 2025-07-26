export interface ProgressionParams {
  currentWeight: number;
  currentReps: number;
  targetReps: number;
  incrementScheme?: 'linear' | 'percentage';
}

export function calculateNextWeight(params: ProgressionParams): number {
  const { currentWeight, currentReps, targetReps, incrementScheme = 'linear' } = params;
  
  if (currentReps >= targetReps) {
    return incrementScheme === 'percentage' 
      ? currentWeight * 1.025 // 2.5% increase
      : currentWeight + 2.5; // 2.5kg increase
  }
  return currentWeight;
}

export function estimate1RM(weight: number, reps: number): number {
  // Using Epley formula
  return weight * (1 + reps / 30);
}
