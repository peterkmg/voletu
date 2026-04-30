import process from 'node:process'
import tailwindcss from '@tailwindcss/vite'
import { tanstackRouter } from '@tanstack/router-plugin/vite'
import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

const host = process.env.TAURI_DEV_HOST


export default defineConfig(async () => ({
  plugins: [
    tanstackRouter({
      target: 'react',
      autoCodeSplitting: true,
      routeFileIgnorePattern: 'shared|\\.(test|spec)\\.(ts|tsx)$',
    }),
    react(),
    tailwindcss(),
  ],

  resolve: {
    tsconfigPaths: true,
  },


  clearScreen: false,


  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
  },
}))
