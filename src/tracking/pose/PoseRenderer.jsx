import { useEffect, useRef } from 'react';

export default function PoseRenderer({ canvasRef, pose }) {
  const animationRef = useRef();

  useEffect(() => {
    const drawPose = () => {
      const canvas = canvasRef.current;
      if (!canvas || !pose) return;
      
      const ctx = canvas.getContext('2d');
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      
      // Draw skeleton
      pose.keypoints.forEach(kp => {
        if (kp.score > 0.5) {
          ctx.beginPath();
          ctx.arc(kp.x, kp.y, 5, 0, 2 * Math.PI);
          ctx.fillStyle = 'red';
          ctx.fill();
        }
      });
      
      animationRef.current = requestAnimationFrame(drawPose);
    };
    
    animationRef.current = requestAnimationFrame(drawPose);
    return () => cancelAnimationFrame(animationRef.current);
  }, [pose, canvasRef]);

  return null;
}
