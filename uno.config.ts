import { defineConfig, presetAttributify, presetUno, presetWind } from 'unocss'
import extractor_svelte from '@unocss/extractor-svelte'

export default defineConfig({
  presets: [presetAttributify(), presetUno(), presetWind({
    
  })],
  extractors: [extractor_svelte()]
})
