import { Vector2 } from '@/utils/math/linearAlgebra';
declare module '@/public/wasm/ai_trainer' {
    export interface ValidationResult {
        angles: Record<string, number>;
        errors: string[];
        repDetected: boolean;
    }

    export class ExerciseValidator {
        constructor(exerciseId: string);
        validatePose(keypoints: Record<string, {x: number, y: number}>): ValidationResult;
    }

    export default function init(): Promise<{
        ExerciseValidator: typeof ExerciseValidator,
        ValidationResult: ValidationResult
    }>;
}
const point = new Vector2(wasmData.x, wasmData.y);