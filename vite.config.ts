import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig(async () => ({
  plugins: [vue()],
  css: {
    preprocessorOptions: {
      scss: {
        additionalData: `
          @import "./src/style/breakpoints";
          @import "./src/style/spacing";
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
