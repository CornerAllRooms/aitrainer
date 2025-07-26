export function degreesToRadians(degrees: number): number {
  return degrees * (Math.PI / 180);
}

export function radiansToDegrees(radians: number): number {
  return radians * (180 / Math.PI);
}

export function normalizeAngle(angle: number): number {
  return ((angle % 360) + 360) % 360;
}

export function angleBetweenPoints(
  x1: number, y1: number,
  x2: number, y2: number,
  x3: number, y3: number
): number {
  const v1x = x1 - x2;
  const v1y = y1 - y2;
  const v2x = x3 - x2;
  const v2y = y3 - y2;
  
  const dot = v1x * v2x + v1y * v2y;
  const mag1 = Math.sqrt(v1x * v1x + v1y * v1y);
  const mag2 = Math.sqrt(v2x * v2x + v2y * v2y);
  
  return radiansToDegrees(Math.acos(dot / (mag1 * mag2)));
}
