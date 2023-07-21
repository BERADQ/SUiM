import './styles.css'
import 'normalize.css'
import App from './App.svelte'
import { invoke } from '@tauri-apps/api'

const app = new App({
  target: document.getElementById('app')
})

export default app
document.oncontextmenu = () => false

//为win10环境添加背景，所以需要单独添加class
invoke('is_windows10').then((b: boolean) => {
  if (!b) {
    document.documentElement.toggleAttribute('data-set-background', true)
  }
})
