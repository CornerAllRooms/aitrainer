const WASM_PATH = '/wasm/ai_trainer_bg.wasm';

let wasm;
let wasmInitialized = false;

async function initWasm() {
    if (wasmInitialized) return wasm;
    
    // Fallback for different import scenarios
    const imports = {
        './ai_trainer_bg.js': {
            __wbg_alert: (ptr, len) => {
                const msg = getStringFromWasm(ptr, len);
                console.warn('WASM Alert:', msg);
            }
        }
    };

    // Instantiate WASM
    if (typeof window === 'object') {
        // Browser environment
        const response = await fetch(WASM_PATH);
        const bytes = await response.arrayBuffer();
        wasm = await WebAssembly.instantiate(bytes, imports);
    } else {
        // Node.js environment
        const fs = await import('fs');
        const path = await import('path');
        const bytes = fs.readFileSync(path.resolve(__dirname, WASM_PATH));
        wasm = await WebAssembly.instantiate(bytes, imports);
    }

    wasmInitialized = true;
    return wasm.instance.exports;
}

// Helper function to handle strings
function getStringFromWasm(ptr, len) {
    const mem = new Uint8Array(wasm.instance.exports.memory.buffer);
    return new TextDecoder().decode(mem.subarray(ptr, ptr + len));
}

export class ExerciseValidator {
    constructor(exerciseId) {
        this.validator = new wasm.instance.exports.ExerciseValidator(exerciseId);
    }

    validatePose(keypoints) {
        const jsonStr = JSON.stringify(keypoints);
        const resultPtr = this.validator.validate_pose(jsonStr);
        const resultJson = getStringFromWasm(resultPtr);
        return JSON.parse(resultJson);
    }
}

export default async function init() {
    if (!wasm) await initWasm();
    return {
        ExerciseValidator,
        ValidationResult: wasm.instance.exports.ValidationResult
    };
}
