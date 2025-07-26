// Top of file
import { calculateJointAngles, Vector2 } from '@/utils/math';
import { calculateVelocity } from '@/utils/math/physics/kinematics';
import { useRef, useEffect } from 'react';
import useCamera from './hooks/useCamera';
import PoseRenderer from './PoseRenderer';
import SkeletonView from './components/SkeletonView';

function CameraProcessor({ pose }) {
  const canvasRef = useRef();

  return (
    <div className="camera-container">
      <video autoPlay playsInline />
      <canvas ref={canvasRef} />
      <SkeletonView pose={pose} canvasRef={canvasRef} />
    </div>
  );
}
export default function CameraFeed({ exerciseId, onAnalysis }) {
  const videoRef = useRef(null);
  const { startCamera, stopCamera } = useCamera();
  const { analyzeFrame } = usePoseDetection(exerciseId);
// Usage example in component:
const elbowAngle = calculateJointAngles(keypoints, [5, 7, 9]);
const velocity = calculateVelocity(prevPosition, currentPosition, deltaTime);
  useEffect(() => {
    const initCamera = async () => {
      await startCamera(videoRef.current);
      
      const processFrame = async () => {
        if (videoRef.current && videoRef.current.readyState >= 2) {
          const analysis = await analyzeFrame(videoRef.current);
          onAnalysis(analysis);
        }
        requestAnimationFrame(processFrame);
      };
      processFrame();
    };

    initCamera();
    return () => stopCamera();
  }, [exerciseId]);

  return (
    <div className="relative">
      <video 
        ref={videoRef} 
        autoPlay 
        playsInline 
        muted
        className="w-full h-auto mirror-mode"
      />
      <PoseRenderer exerciseId={exerciseId} />
    </div>
  );
}
