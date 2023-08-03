import './styles.css'
import 'normalize.css'
import App from './App.svelte'
import { invoke } from '@tauri-apps/api'
import { OS } from './common'

const app = new App({
  target: document.getElementById('app')
})

export default app
document.oncontextmenu = () => false

//为win10环境添加背景
// invoke('is_windows10').then((b: boolean) => {
//   if (b) {
//     document
//       .querySelector('html').toggleAttribute('data-set-background')
//   }
// })
if (import.meta.env.TAURI_PLATFORM !== OS.MacOS) {
  document.querySelector('html').toggleAttribute('data-set-background')
}
