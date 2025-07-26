import { useEffect } from 'react';
import { createValidator } from '@/libs/wasm/adapter';

export default function WasmTest() {
  useEffect(() => {
    const test = async () => {
      try {
        const validator = await createValidator('barbell-curl');
        console.log('WASM validator created:', validator);
      } catch (error) {
        console.error('WASM test failed:', error);
      }
    };
    test();
  }, []);

  return <div>Testing WASM initialization...</div>;
}
