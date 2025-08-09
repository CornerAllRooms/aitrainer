import { createContext, useContext, useState, useEffect } from 'react';
import init from '@/public/wasm/ai_trainer';

const WasmContext = createContext();

export const WasmProvider = ({ children }) => {
  const [wasm, setWasm] = useState(null);

  useEffect(() => {
    const loadWasm = async () => {
      try {
        await init();
        setWasm({
          PoseDetector: new PoseDetector(),
          NeonRenderer: new NeonRenderer(0.1, 0.8, 0.9)
        });
      } catch (err) {
        console.error('WASM initialization failed:', err);
      }
    };
    loadWasm();
  }, []);

  return (
    <WasmContext.Provider value={wasm}>
      {children}
    </WasmContext.Provider>
  );
};

export const useWasm = () => useContext(WasmContext);