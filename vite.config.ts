import { defineConfig } from 'vite'
import path from 'path'
import vue from '@vitejs/plugin-vue'

export default defineConfig(async () => ({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      atoms: path.resolve(__dirname, './src/components/atoms'),
      types: path.resolve(__dirname, './src/types'),
      views: path.resolve(__dirname, './src/components/views'),
    },
  },
  css: {
    preprocessorOptions: {
      scss: {
        additionalData: `
          @import "./src/style/variables";
          @import "./src/style/breakpoints";
          @import "./src/style/layout";
          `,
      },
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
}))
