// C:\Users\Molap\Documents\work\AI-Trainer\src\flows\exercise-execution\WASMcontext\useWasm.js
import { useState, useEffect, useCallback } from 'react';
import { createValidator } from '@/libs/wasm/adapter';
import { useExerciseStore } from '@/state/useExerciseStore';

export function useWasm(exerciseId) {
  const [state, setState] = useState({
    validator: null,
    loading: true,
    error: null,
    performance: null
  });
  const { setWasmPerformance } = useExerciseStore();

  const initializeValidator = useCallback(async (id) => {
    setState(prev => ({ ...prev, loading: true, error: null }));
    
    try {
      const startTime = performance.now();
      const validator = await createValidator(id);
      const loadTime = performance.now() - startTime;

      setState({
        validator,
        loading: false,
        error: null,
        performance: loadTime
      });
      setWasmPerformance(loadTime);
      
      return validator;
    } catch (error) {
      setState({
        validator: null,
        loading: false,
        error: error.message,
        performance: null
      });
      throw error;
    }
  }, [setWasmPerformance]);

  // Reinitialize when exerciseId changes
  useEffect(() => {
    let isMounted = true;
    
    const init = async () => {
      try {
        await initializeValidator(exerciseId);
      } catch (error) {
        if (isMounted) {
          console.error('WASM initialization failed:', error);
        }
      }
    };

    init();

    return () => {
      isMounted = false;
    };
  }, [exerciseId, initializeValidator]);

  const validatePose = useCallback(async (keypoints) => {
    if (!state.validator) {
      throw new Error('WASM validator not initialized');
    }
    
    try {
      const start = performance.now();
      const result = await state.validator.validatePose(keypoints);
      const validationTime = performance.now() - start;
      
      return {
        ...result,
        performance: validationTime
      };
    } catch (error) {
      console.error('Pose validation failed:', error);
      throw error;
    }
  }, [state.validator]);

  return {
    ...state,
    initializeValidator,
    validatePose
  };
}