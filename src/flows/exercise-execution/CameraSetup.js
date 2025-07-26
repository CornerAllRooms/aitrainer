import { useEffect, useRef, useState } from 'react';
import { useWasm } from './WASMcontext/useWasm';
import { useExerciseStore } from '@/state/useExerciseStore';
import FeedbackDisplay from '@/components/Camera/FeedbackDisplay';
import PoseRenderer from '@/tracking/pose/PoseRenderer';

export default function CameraSetup({ exerciseId }) {
  const videoRef = useRef(null);
  const canvasRef = useRef(null);
  const [pose, setPose] = useState(null);
  const { validator, loading, error, validatePose } = useWasm(exerciseId);
  const { startSession, updateRepCount, addFormError } = useExerciseStore();

  // Initialize camera and pose detection
  useEffect(() => {
    let poseDetector;
    let animationFrameId;

    const initCamera = async () => {
      try {
        const stream = await navigator.mediaDevices.getUserMedia({
          video: { width: 1280, height: 720, facingMode: 'user' }
        });
        videoRef.current.srcObject = stream;
        
        // Initialize your pose detector (e.g., MoveNet, MediaPipe)
        poseDetector = await initializePoseDetector();
        startSession(exerciseId);
        
        const detectPose = async () => {
          if (videoRef.current && canvasRef.current) {
            const detectedPose = await poseDetector.estimatePose(videoRef.current);
            setPose(detectedPose);
            
            if (validator && detectedPose) {
              const validation = await validatePose(detectedPose.keypoints);
              if (validation.repDetected) updateRepCount();
              if (validation.errors.length) addFormError(validation.errors);
            }
          }
          animationFrameId = requestAnimationFrame(detectPose);
        };
        
        detectPose();
      } catch (err) {
        console.error('Camera setup failed:', err);
      }
    };

    initCamera();

    return () => {
      if (animationFrameId) cancelAnimationFrame(animationFrameId);
      if (videoRef.current?.srcObject) {
        videoRef.current.srcObject.getTracks().forEach(track => track.stop());
      }
    };
  }, [exerciseId, validator, validatePose, startSession, updateRepCount, addFormError]);

  if (loading) return <div className="camera-loading">Initializing WASM validator...</div>;
  if (error) return <div className="camera-error">Error: {error}</div>;

  return (
    <div className="camera-setup">
      <div className="video-container">
        <video ref={videoRef} playsInline autoPlay muted className="input-video" />
        <canvas ref={canvasRef} className="output-canvas" />
        <PoseRenderer canvasRef={canvasRef} pose={pose} />
      </div>
      
      <div className="feedback-panel">
        <FeedbackDisplay 
          exerciseId={exerciseId} 
          pose={pose} 
          validator={validator}
        />
      </div>
    </div>
  );
}

// Mock pose detector initialization - replace with your actual implementation
async function initializePoseDetector() {
  return {
    estimatePose: async (video) => {
      // This would be replaced with actual pose detection logic
      return {
        keypoints: [
          // Sample keypoints data structure
          { name: 'left_wrist', x: 100, y: 200, score: 0.9 },
          { name: 'right_elbow', x: 300, y: 400, score: 0.8 },
          // ... other keypoints
        ]
      };
    }
  };
}