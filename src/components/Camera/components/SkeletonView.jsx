import { Easing, Timer } from '@/utils/math/animation/time';
import { useEffect, useRef } from 'react';
import './SkeletonView.css';

const NEON_COLORS = {
  joints: '#04d9ff', // Neon blue
  bones: '#ff08e8',  // Neon pink
  confidenceHigh: '#00ff00', // Neon green
  confidenceMid: '#ffff00',  // Neon yellow
  confidenceLow: '#ff0000'   // Neon red
};
const timer = new Timer();
const progress = Easing.easeInOutQuad(timer.getElapsedTime() / 1000);

const BONE_CONNECTIONS = [
  ['left_shoulder', 'left_elbow'],
  ['left_elbow', 'left_wrist'],
  ['right_shoulder', 'right_elbow'],
  ['right_elbow', 'right_wrist'],
  ['left_shoulder', 'right_shoulder'],
  ['left_shoulder', 'left_hip'],
  ['right_shoulder', 'right_hip'],
  ['left_hip', 'right_hip'],
  ['left_hip', 'left_knee'],
  ['left_knee', 'left_ankle'],
  ['right_hip', 'right_knee'],
  ['right_knee', 'right_ankle']
];

export default function SkeletonView({ pose, canvasRef }) {
  const animationRef = useRef();

  useEffect(() => {
    const drawSkeleton = () => {
      const canvas = canvasRef.current;
      if (!canvas || !pose) return;

      const ctx = canvas.getContext('2d');
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      // Set canvas dimensions to match video feed
      if (canvas.width !== canvas.offsetWidth || canvas.height !== canvas.offsetHeight) {
        canvas.width = canvas.offsetWidth;
        canvas.height = canvas.offsetHeight;
      }

      // Draw bones first
      BONE_CONNECTIONS.forEach(([jointA, jointB]) => {
        const pointA = pose.keypoints.find(kp => kp.name === jointA);
        const pointB = pose.keypoints.find(kp => kp.name === jointB);

        if (pointA && pointB && pointA.score > 0.3 && pointB.score > 0.3) {
          // Determine bone color based on average confidence
          const avgScore = (pointA.score + pointB.score) / 2;
          let boneColor = NEON_COLORS.bones;
          if (avgScore > 0.8) boneColor = NEON_COLORS.confidenceHigh;
          else if (avgScore > 0.5) boneColor = NEON_COLORS.confidenceMid;

          ctx.beginPath();
          ctx.moveTo(pointA.x, pointA.y);
          ctx.lineTo(pointB.x, pointB.y);
          ctx.lineWidth = 4;
          ctx.strokeStyle = boneColor;
          ctx.shadowBlur = 15;
          ctx.shadowColor = boneColor;
          ctx.stroke();
        }
      });

      // Draw joints on top
      pose.keypoints.forEach(keypoint => {
        if (keypoint.score > 0.3) {
          let jointColor = NEON_COLORS.joints;
          if (keypoint.score > 0.8) jointColor = NEON_COLORS.confidenceHigh;
          else if (keypoint.score > 0.5) jointColor = NEON_COLORS.confidenceMid;

          ctx.beginPath();
          ctx.arc(keypoint.x, keypoint.y, 6, 0, 2 * Math.PI);
          ctx.fillStyle = jointColor;
          ctx.shadowBlur = 20;
          ctx.shadowColor = jointColor;
          ctx.fill();

          // Inner glow
          ctx.beginPath();
          ctx.arc(keypoint.x, keypoint.y, 3, 0, 2 * Math.PI);
          ctx.fillStyle = 'white';
          ctx.fill();
        }
      });

      animationRef.current = requestAnimationFrame(drawSkeleton);
    };

    animationRef.current = requestAnimationFrame(drawSkeleton);
    return () => cancelAnimationFrame(animationRef.current);
  }, [pose, canvasRef]);

  return null;
}