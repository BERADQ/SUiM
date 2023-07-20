import './styles.pcss'
import 'normalize.css'
import App from './App.svelte'
import { invoke } from '@tauri-apps/api'

const app = new App({
  target: document.getElementById('app')
})

export default app
document.oncontextmenu = () => false

invoke('is_windows10').then((b: boolean) => {
  if (b) {
    document.body.classList.add('windows10')
  }
})
