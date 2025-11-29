import { readFileSync } from 'node:fs';
import { fileURLToPath, URL } from 'node:url';

import tailwindcss from '@tailwindcss/vite';
import vue from '@vitejs/plugin-vue';
import { defineConfig, loadEnv } from 'vite';
import vueDevTools from 'vite-plugin-vue-devtools';

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  process.env = loadEnv(mode, '.', ['APP_', 'TLS_', 'VITE_', 'TAURI_ENV_']);
  const port = parseInt(process.env.APP_PORT || '8000');
  const host = process.env.APP_HOST || 'localhost';
  const cert = process.env.TLS_CERT || '';
  const key = process.env.TLS_KEY || '';
  const https = {
    key: readFileSync(key),
    cert: readFileSync(cert),
  };

  return {
    plugins: [vue(), vueDevTools(), tailwindcss()],
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url)),
      },
    },
    server: {
      strictPort: true,
      port,
      host,
      https,
      watch: {
        ignored: ['**/src-tauri/**'],
      },
      hmr: {
        protocol: 'wss',
        host,
        port,
      },
    },
    build: {
      target: process.env.TAURI_ENV_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
      minify: !process.env.TAURI_ENV_DEBUG ? 'esbuild' : false,
      sourcemap: !!process.env.TAURI_ENV_DEBUG,
    },
  };
});
