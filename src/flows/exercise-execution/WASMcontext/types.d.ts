import { WasmExerciseValidator } from '@/libs/wasm/adapter';

export interface WasmState {
  validator: WasmExerciseValidator | null;
  loading: boolean;
  error: string | null;
  performance: number | null;
  initializeValidator: (exerciseId: string) => Promise<WasmExerciseValidator>;
  validatePose: (keypoints: Record<string, any>) => Promise<{
    angles: Record<string, number>;
    errors: string[];
    repDetected: boolean;
    repCount: number;
    performance: number;
  }>;
}
