import { createContext, useContext, useState, useEffect } from 'react';
import { createValidator } from '@/libs/wasm/adapter';

// Create context
const WASMContext = createContext({
  validator: null,
  loading: true,
  error: null,
  performance: null
});

export function WASMProvider({ children, exerciseId }) {
  const [state, setState] = useState({
    validator: null,
    loading: true,
    error: null,
    performance: null
  });

  useEffect(() => {
    let mounted = true;
    const initWASM = async () => {
      try {
        const start = performance.now();
        const validator = await createValidator(exerciseId);
        const loadTime = performance.now() - start;

        if (mounted) {
          setState({
            validator,
            loading: false,
            error: null,
            performance: loadTime
          });
        }
      } catch (error) {
        if (mounted) {
          setState({
            validator: null,
            loading: false,
            error: error.message,
            performance: null
          });
        }
      }
    };

    initWASM();

    return () => {
      mounted = false;
    };
  }, [exerciseId]);

  return (
    <WASMContext.Provider value={state}>
      {children}
    </WASMContext.Provider>
  );
}

export function useWASM() {
  const context = useContext(WASMContext);
  if (!context) {
    throw new Error('useWASM must be used within a WASMProvider');
  }
  return context;
}