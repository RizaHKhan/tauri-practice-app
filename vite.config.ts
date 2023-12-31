import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export const aliases = {
  '@': resolve(__dirname, './src'),
}

export default defineConfig(async () => ({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  resolve: {
    aliases,
  },
}))
