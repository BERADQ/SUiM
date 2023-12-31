import { defineConfig } from 'vite'
import { svelte, vitePreprocess } from '@sveltejs/vite-plugin-svelte'
import sveltePreprocess from 'svelte-preprocess'
import preset_env from 'postcss-preset-env'
import UnoCSS from '@unocss/svelte-scoped/vite'
import PostUnoCss from '@unocss/postcss'
import UnoConf from './uno.config'

import { OS } from './src/common'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  css: {
    postcss: {
      plugins: [
        preset_env({
          stage: 2,
          browsers: 'since 2015'
        }),
        PostUnoCss()
      ]
    }
  },
  plugins: [
    UnoCSS(),
    svelte({
      preprocess: [
        vitePreprocess(),
        sveltePreprocess({
          typescript: true
        })
      ]
    })
  ],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM == OS.Windows ? 'chrome105' : 'safari13',
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG
  }
}))
