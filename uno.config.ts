import { defineConfig, presetAttributify, presetUno, presetWind, presetIcons } from 'unocss'
import { PresetMiniOptions } from 'unocss/preset-mini'
import extractor_svelte from '@unocss/extractor-svelte'
import Colors from './uno.colors'

const config: PresetMiniOptions = {
  dark: 'media'
}
export default defineConfig({
  presets: [presetAttributify(), presetUno(config), presetWind(config), presetIcons({ prefix: "i-" })],
  theme: {
    colors: Colors
  },
  extractors: [extractor_svelte()]
})
