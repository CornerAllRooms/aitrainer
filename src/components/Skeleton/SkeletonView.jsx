import { useEffect, useRef, useMemo } from 'react';
import './styles/skeleton.css';

const BONE_CONNECTIONS = [
  ['left_wrist', 'left_elbow'],
  ['left_elbow', 'left_shoulder'],
  ['right_wrist', 'right_elbow'], 
  ['right_elbow', 'right_shoulder'],
  ['left_shoulder', 'right_shoulder'],
  ['left_shoulder', 'left_hip'],
  ['right_shoulder', 'right_hip'],
  ['left_hip', 'right_hip'],
  ['left_hip', 'left_knee'],
  ['right_hip', 'right_knee'],
  ['left_knee', 'left_ankle'],
  ['right_knee', 'right_ankle']
];

export default function SkeletonView({ keypoints, engagement }) {
  const canvasRef = useRef(null);
  const animationRef = useRef(null);
  const lastRenderTime = useRef(0);

  const connections = useMemo(() => {
    return BONE_CONNECTIONS.map(([a, b]) => {
      const pointA = keypoints?.find(kp => kp.name === a);
      const pointB = keypoints?.find(kp => kp.name === b);
      return pointA && pointB ? { pointA, pointB } : null;
    }).filter(Boolean);
  }, [keypoints]);

  const renderFrame = useCallback((timestamp) => {
    if (timestamp - lastRenderTime.current < 16) { // ~60fps
      animationRef.current = requestAnimationFrame(renderFrame);
      return;
    }
    lastRenderTime.current = timestamp;

    const ctx = canvasRef.current?.getContext('2d');
    if (!ctx) return;

    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);

    // Animated neon pulse
    const pulseIntensity = 0.5 + 0.3 * Math.sin(timestamp / 300);
    const glowColor = `rgba(0, 240, 255, ${0.6 + engagement * 0.4})`;

    // Draw bones with pulse animation
    ctx.lineWidth = 3;
    ctx.shadowBlur = 10 + 5 * pulseIntensity;
    ctx.shadowColor = glowColor;
    ctx.strokeStyle = glowColor;

    connections.forEach(({ pointA, pointB }) => {
      ctx.beginPath();
      ctx.moveTo(pointA.x, pointA.y);
      ctx.lineTo(pointB.x, pointB.y);
      ctx.stroke();
    });

    // Draw joints
    keypoints?.forEach(kp => {
      const size = 4 + 2 * kp.score * pulseIntensity;
      ctx.beginPath();
      ctx.arc(kp.x, kp.y, size, 0, Math.PI * 2);
      ctx.fillStyle = glowColor;
      ctx.fill();
    });

    animationRef.current = requestAnimationFrame(renderFrame);
  }, [connections, engagement, keypoints]);

  useEffect(() => {
    animationRef.current = requestAnimationFrame(renderFrame);
    return () => cancelAnimationFrame(animationRef.current);
  }, [renderFrame]);

  return <canvas ref={canvasRef} className="skeleton-canvas" />;
}
