export class Timer {
  private startTime: number;
  private pausedTime: number = 0;
  private isRunning: boolean = false;

  constructor() {
    this.startTime = performance.now();
  }

  start(): void {
    if (!this.isRunning) {
      this.startTime = performance.now() - this.pausedTime;
      this.isRunning = true;
    }
  }

  pause(): void {
    if (this.isRunning) {
      this.pausedTime = performance.now() - this.startTime;
      this.isRunning = false;
    }
  }

  reset(): void {
    this.startTime = performance.now();
    this.pausedTime = 0;
  }

  getElapsedTime(): number {
    return this.isRunning 
      ? performance.now() - this.startTime 
      : this.pausedTime;
  }
}
