import { defineConfig, presetAttributify, presetUno, presetWind } from 'unocss'
import extractor_svelte from '@unocss/extractor-svelte'

export default defineConfig({
  presets: [
    presetAttributify(),
    presetUno(),
    presetWind()
  ],
  theme:{
    colors:{
      aaa:"#f00"
    }
  },
  extractors: [extractor_svelte()]
})
