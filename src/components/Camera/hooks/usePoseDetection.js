import { useEffect, useState } from 'react';
import * as poseDetection from '@tensorflow-models/pose-detection';
import * as tf from '@tensorflow/tfjs';
import wasmAdapter from '../../lib/wasm/adapter';

export default function usePoseDetection(exerciseId) {
  const [detector, setDetector] = useState(null);
  const [wasm, setWasm] = useState(null);

  // Initialize detector and WASM
  useEffect(() => {
    const init = async () => {
      await tf.setBackend('wasm');
      await tf.ready();
      
      const model = poseDetection.SupportedModels.MoveNet;
      const detector = await poseDetection.createDetector(model, {
        modelType: poseDetection.movenet.modelType.SINGLEPOSE_THUNDER,
      });
      
      const wasm = await wasmAdapter.init(exerciseId);
      setDetector(detector);
      setWasm(wasm);
    };

    init();
    return () => {
      detector?.dispose();
    };
  }, [exerciseId]);

  const analyzeFrame = useCallback(async (videoElement) => {
    if (!detector || !wasm) return null;
    
    const poses = await detector.estimatePoses(videoElement);
    if (poses.length === 0) return null;
    
    const keypoints = poses[0].keypoints;
    const analysis = wasm.analyzePose(keypoints);
    
    return {
      keypoints,
      angles: analysis.angles,
      formErrors: analysis.formErrors,
      repCount: analysis.repCount
    };
  }, [detector, wasm]);

  return { analyzeFrame };
}
