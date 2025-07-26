export function calculateJointAngles(
  keypoints: Array<{ x: number, y: number }>,
  jointIndices: [number, number, number] // [start, middle, end]
): number {
  const [start, middle, end] = jointIndices.map(i => keypoints[i]);
  
  // Vectors from middle joint
  const v1 = { x: start.x - middle.x, y: start.y - middle.y };
  const v2 = { x: end.x - middle.x, y: end.y - middle.y };
  
  // Dot product and magnitudes
  const dot = v1.x * v2.x + v1.y * v2.y;
  const mag1 = Math.sqrt(v1.x * v1.x + v1.y * v1.y);
  const mag2 = Math.sqrt(v2.x * v2.x + v2.y * v2.y);
  
  // Angle in radians then convert to degrees
  return Math.acos(dot / (mag1 * mag2)) * (180 / Math.PI);
}
