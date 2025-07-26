export function calculateVelocity(
  startPos: { x: number, y: number },
  endPos: { x: number, y: number },
  timeDelta: number
): { vx: number, vy: number, speed: number } {
  const vx = (endPos.x - startPos.x) / timeDelta;
  const vy = (endPos.y - startPos.y) / timeDelta;
  const speed = Math.sqrt(vx * vx + vy * vy);
  return { vx, vy, speed };
}

export function calculateAcceleration(
  startVel: { vx: number, vy: number },
  endVel: { vx: number, vy: number },
  timeDelta: number
): { ax: number, ay: number, magnitude: number } {
  const ax = (endVel.vx - startVel.vx) / timeDelta;
  const ay = (endVel.vy - startVel.vy) / timeDelta;
  const magnitude = Math.sqrt(ax * ax + ay * ay);
  return { ax, ay, magnitude };
}
