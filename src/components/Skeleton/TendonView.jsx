import { useEffect, useRef, useMemo } from 'react';
import './styles/skeleton.css';

const TENDON_CONNECTIONS = [
  ['left_elbow', 'left_shoulder', 'left_hip'],
  ['right_elbow', 'right_shoulder', 'right_hip'],
  ['left_knee', 'left_ankle'],
  ['right_knee', 'right_ankle']
];

export default function TendonView({ keypoints, intensity }) {
  const canvasRef = useRef(null);
  const pointsCache = useRef([]);
  const animationRef = useRef(null);

  const tendonPaths = useMemo(() => {
    return TENDON_CONNECTIONS.map(path => {
      return path.map(joint => keypoints?.find(kp => kp.name === joint)).filter(Boolean);
    }).filter(path => path.length > 1);
  }, [keypoints]);

  useEffect(() => {
    if (!tendonPaths.length) return;

    const ctx = canvasRef.current?.getContext('2d');
    if (!ctx) return;

    const render = () => {
      ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);

      // Smooth point transitions
      tendonPaths.forEach((points, i) => {
        if (!pointsCache.current[i]) {
          pointsCache.current[i] = points.map(p => ({ ...p }));
        }

        // Ease towards actual points (dampened movement)
        points.forEach((point, j) => {
          const cached = pointsCache.current[i][j];
          cached.x += (point.x - cached.x) * 0.3;
          cached.y += (point.y - cached.y) * 0.3;
        });

        // Draw tension lines
        ctx.beginPath();
        pointsCache.current[i].forEach((point, idx) => {
          if (idx === 0) ctx.moveTo(point.x, point.y);
          else ctx.lineTo(point.x, point.y);
        });

        const tensionColor = `rgba(50, 255, 100, ${0.2 + intensity * 0.8})`;
        ctx.strokeStyle = tensionColor;
        ctx.lineWidth = 1.5;
        ctx.lineCap = 'round';
        ctx.shadowBlur = 8 * intensity;
        ctx.shadowColor = '#32FF64';
        ctx.stroke();
      });

      animationRef.current = requestAnimationFrame(render);
    };

    animationRef.current = requestAnimationFrame(render);
    return () => cancelAnimationFrame(animationRef.current);
  }, [tendonPaths, intensity]);

  return <canvas ref={canvasRef} className="tendon-canvas" />;
}
