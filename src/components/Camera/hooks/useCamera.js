import { useCallback } from 'react';

export default function useCamera() {
  const startCamera = useCallback(async (videoElement) => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({
        video: {
          width: { ideal: 1280 },
          height: { ideal: 720 },
          facingMode: 'user'
        }
      });
      videoElement.srcObject = stream;
      await new Promise((resolve) => {
        videoElement.onloadedmetadata = resolve;
      });
      videoElement.play();
    } catch (err) {
      console.error("Camera error:", err);
    }
  }, []);

  const stopCamera = useCallback(() => {
    if (videoElement?.srcObject) {
      videoElement.srcObject.getTracks().forEach(track => track.stop());
    }
  }, []);

  return { startCamera, stopCamera };
}
