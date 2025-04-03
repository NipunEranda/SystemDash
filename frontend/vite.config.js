import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'

// https://vite.dev/config/
export default defineConfig({
  server: {
    port: 3000,
    proxy: {
      '/ws': {
        target: 'ws://127.0.0.1:9999',
        changeOrigin: true,
        ws: true,
      },
      '/api/': {
        target: 'http://localhost:8080/api/',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
    },
  },
  plugins: [
    vue(),
    tailwindcss()
  ],
})
