import { defineConfig } from 'vite'

export default defineConfig(async() => ({
  clearScreen: false,
  server: {
    port: 8000,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
  test: {
    globals: true,
    environment: "jsdom",
  },
}));