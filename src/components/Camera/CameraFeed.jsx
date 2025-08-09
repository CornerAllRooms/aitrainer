import React, { useRef, useEffect, useState } from 'react';
import init, { ExerciseAnalyzer } from '@/public/wasm/ai_trainer';
import { Vector2 } from '@/utils/math/linearAlgebra';
import './camera-feed.css';
import { usePoseDetection } from './hooks/usePoseDetection';
import { useWasm } from '@/libs/wasm/WasmContext';
import PoseRenderer from './PoseRenderer';

const CameraFeed = ({ exerciseId, onAnalysis, onError }) => {
  const videoRef = useRef(null);
  const canvasRef = useRef(null);
  const analyzerRef = useRef(null);
  const animationIdRef = useRef(null);
  const [isWasmReady, setWasmReady] = useState(false);
  const [cameraState, setCameraState] = useState('loading');

  // Initialize WASM and camera
  useEffect(() => {
    const initialize = async () => {
      try {
        // 1. Load WASM module
        await init();
        setWasmReady(true);
        
        // 2. Initialize exercise analyzer
        analyzerRef.current = new ExerciseAnalyzer(exerciseId);
        
        // 3. Start camera stream
        const stream = await navigator.mediaDevices.getUserMedia({
          video: {
            width: { ideal: 640 },
            height: { ideal: 480 },
            facingMode: 'user',
            frameRate: { ideal: 30 }
          },
          audio: false
        });
        
        if (videoRef.current) {
          videoRef.current.srcObject = stream;
          videoRef.current.onloadedmetadata = () => {
            setCameraState('ready');
            startProcessing();
          };
        }
      } catch (err) {
        console.error('Initialization error:', err);
        onError(`Failed to initialize: ${err.message}`);
        setCameraState('error');
      }
    };

    initialize();

    return () => {
      // Cleanup
      if (animationIdRef.current) {
        cancelAnimationFrame(animationIdRef.current);
      }
      if (videoRef.current?.srcObject) {
        videoRef.current.srcObject.getTracks().forEach(track => track.stop());
      }
    };
  }, [exerciseId]);

  // Frame processing loop
  const startProcessing = () => {
    const processFrame = async (timestamp) => {
      if (!videoRef.current || !canvasRef.current || !analyzerRef.current) {
        animationIdRef.current = requestAnimationFrame(processFrame);
        return;
      }

      try {
        // 1. Prepare canvas
        const canvas = canvasRef.current;
        const ctx = canvas.getContext('2d');
        canvas.width = videoRef.current.videoWidth;
        canvas.height = videoRef.current.videoHeight;
        
        // 2. Draw video frame
        ctx.drawImage(videoRef.current, 0, 0, canvas.width, canvas.height);
        
        // 3. Get keypoints data (simplified - in reality would use pose detection)
        const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        const keypoints = await detectPose(imageData); // Your pose detection logic
        
        // 4. Process with WASM
        const result = await analyzerRef.current.process_frame(
          keypointsToFloat32Array(keypoints),
          timestamp
        );
        
        // 5. Render feedback
        renderFeedback(ctx, result);
        
        // 6. Pass analysis to parent
        onAnalysis({
          ...result,
          keypoints: keypoints.map(kp => ({
            ...kp,
            position: new Vector2(kp.x, kp.y)
          }))
        });
      } catch (err) {
        console.error('Frame processing error:', err);
      }
      
      animationIdRef.current = requestAnimationFrame(processFrame);
    };
    
    animationIdRef.current = requestAnimationFrame(processFrame);
  };

  // Helper: Convert keypoints to WASM-compatible format
  const keypointsToFloat32Array = (keypoints) => {
    const array = new Float32Array(keypoints.length * 3);
    keypoints.forEach((kp, i) => {
      array[i * 3] = kp.x;
      array[i * 3 + 1] = kp.y;
      array[i * 3 + 2] = kp.confidence;
    });
    return array;
  };

  // Helper: Render WASM analysis results
  const renderFeedback = (ctx, analysis) => {
    // 1. Clear previous frame
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    
    // 2. Draw video frame
    ctx.drawImage(videoRef.current, 0, 0);
    
    // 3. Draw neon overlay (from WASM)
    if (analysis.overlay_data) {
      const imgData = new ImageData(
        new Uint8ClampedArray(analysis.overlay_data),
        ctx.canvas.width,
        ctx.canvas.height
      );
      ctx.putImageData(imgData, 0, 0);
    }
    
    // 4. Draw form feedback
    ctx.fillStyle = '#ffffff';
    ctx.font = '16px Arial';
    analysis.form_errors.forEach((error, i) => {
      ctx.fillText(error, 20, 30 + i * 20);
    });
    
    // 5. Draw rep counter
    ctx.fillStyle = '#3498db';
    ctx.font = 'bold 24px Arial';
    ctx.fillText(`Reps: ${analysis.rep_count}`, 20, ctx.canvas.height - 30);
  };

  // Mock pose detection - replace with your actual implementation
  const detectPose = async (imageData) => {
    return [
      { x: 100, y: 150, confidence: 0.9, name: 'left_shoulder' },
      { x: 120, y: 160, confidence: 0.8, name: 'left_elbow' },
      // ... other keypoints
    ];
  };

  return (
    <div className="camera-container">
      {/* Video Feed */}
      <video
        ref={videoRef}
        autoPlay
        playsInline
        muted
        className={`camera-feed ${cameraState === 'ready' ? 'visible' : 'hidden'}`}
      />
      
      {/* Processing Canvas */}
      <canvas
        ref={canvasRef}
        className="pose-canvas"
        style={{
          position: 'absolute',
          top: 0,
          left: 0,
          pointerEvents: 'none'
        }}
      />
      
      {/* Status Overlay */}
      {cameraState !== 'ready' && (
        <div className="camera-status">
          {cameraState === 'loading' && <p>Initializing camera...</p>}
          {cameraState === 'error' && <p className="error">Camera unavailable</p>}
          {!isWasmReady && <p>Loading AI analyzer...</p>}
        </div>
      )}
    </div>
  );
};

export default CameraFeed;