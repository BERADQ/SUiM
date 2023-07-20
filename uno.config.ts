import { defineConfig, presetAttributify, presetUno, presetWind } from 'unocss'
import extractor_svelte from '@unocss/extractor-svelte'

export default defineConfig({
  presets: [presetAttributify(), presetUno(), presetWind({
    theme:{
      colors:{
        'aaa':'#000'
      }
    }
  })],
  extractors: [extractor_svelte()]
})
