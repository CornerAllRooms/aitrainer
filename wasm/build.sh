#!/bin/bash
set -e

# Build with wasm-pack
wasm-pack build --target web --out-name ai_trainer --out-dir ./out

# Move files to public directory
mkdir -p ../public/wasm
mv ./out/ai_trainer_bg.wasm ../public/wasm/
mv ./out/ai_trainer.js ../public/wasm/ai_trainer_bg.js

# Cleanup
rm -rf ./out

echo "WASM build complete!"
