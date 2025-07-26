export class FrameRateController {
  private lastTime: number = 0;
  private fpsInterval: number;
  private frameCount: number = 0;
  private currentFps: number = 0;

  constructor(private targetFps: number) {
    this.fpsInterval = 1000 / targetFps;
  }

  start(): void {
    this.lastTime = performance.now();
  }

  shouldRender(): boolean {
    const now = performance.now();
    const elapsed = now - this.lastTime;

    if (elapsed > this.fpsInterval) {
      this.lastTime = now - (elapsed % this.fpsInterval);
      this.calculateFPS(now);
      return true;
    }
    return false;
  }

  private calculateFPS(now: number): void {
    this.frameCount++;
    if (now > this.lastTime + 1000) {
      this.currentFps = this.frameCount * 1000 / (now - this.lastTime);
      this.frameCount = 0;
      this.lastTime = now;
    }
  }

  getFPS(): number {
    return this.currentFps;
  }
}
