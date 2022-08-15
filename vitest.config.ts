/// <reference types="vitest" />
/// <reference types="vite/client" />

import { defineConfig } from 'vite'
import Vue from '@vitejs/plugin-vue'
import react from '@vitejs/plugin-react'
export default defineConfig({
  plugins: [
    Vue(),react(),
  ],
  test: {
testTimeout: 60_000,
    hookTimeout: 60_000,
    globals: true,
    environment: 'jsdom',
setupFiles: './src/test/setup.ts',
    css: true,
  },
})
