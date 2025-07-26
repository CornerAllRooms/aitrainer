import { createWasmValidator } from './loader';
import JsValidator from '@/services/exercise/ExerciseValidator';

export async function createValidator(exerciseId, fallback = true) {
  try {
    const validator = await createWasmValidator(exerciseId);
    return {
      validatePose: (keypoints) => validator.validatePose(keypoints),
      type: 'wasm'
    };
  } catch (error) {
    console.error('WASM validator failed:', error);
    if (fallback) {
      console.log('Falling back to JavaScript validator');
      const jsValidator = new JsValidator(exerciseId);
      return {
        validatePose: (keypoints) => jsValidator.validatePose(keypoints),
        type: 'js'
      };
    }
    throw error;
  }
}

export function isWasmSupported() {
  try {
    return typeof WebAssembly === 'object' && 
           typeof WebAssembly.instantiate === 'function';
  } catch (e) {
    return false;
  }
}
