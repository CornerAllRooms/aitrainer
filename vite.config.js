// vite.config.js
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  base: '/static/', // All assets will use this base path
  build: {
    outDir: '../build',
    emptyOutDir: true,
    rollupOptions: {
      input: path.resolve(__dirname, 'src/index.jsx'),
      output: {
        entryFileNames: 'js/[name]-[hash].js',
        assetFileNames: 'assets/[name]-[hash][extname]'
      }
    }
  }
})