let wasmModule;

export async function initWasm() {
  if (!wasmModule) {
    wasmModule = await import('@/public/wasm/ai_trainer');
    await wasmModule.default();
  }
  return wasmModule;
}

export async function createWasmValidator(exerciseId) {
  try {
    await initWasm();
    return new wasmModule.ExerciseValidator(exerciseId);
  } catch (error) {
    console.error('WASM validator creation failed:', error);
    throw error;
  }
}
