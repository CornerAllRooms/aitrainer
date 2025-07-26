import { Vector2 } from '@/utils/math/linearAlgebra';
declare module '@/public/wasm/ai_trainer' {
  export interface Keypoint {
    x: number;
    y: number;
    score: number;
  }

  export interface ValidationResult {
    angles: Record<string, number>;
    errors: string[];
    repDetected: boolean;
    repCount: number;
  }

  export class ExerciseValidator {
    constructor(exerciseId: string);
    validate(keypoints: Record<string, Keypoint>): ValidationResult;
  }

  export default function init(): Promise<void>;
}
const point = new Vector2(wasmData.x, wasmData.y);