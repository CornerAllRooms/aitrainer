#!/bin/bash
set -e

# Build with optimizations
wasm-pack build --target web --release

# Optimize wasm further
wasm-opt pkg/ai_trainer_bg.wasm -O3 -o pkg/ai_trainer_opt.wasm
mv pkg/ai_trainer_opt.wasm pkg/ai_trainer_bg.wasm

echo "Build complete!"