import { writable, get, type Writable } from 'svelte/store'
import { invoke } from '@tauri-apps/api'
import { resolveResource } from '@tauri-apps/api/path'

const figure: Writable<Figure[]> = writable([])

resolveResource('figure.json').then((path) => {
  invoke('get_file', {
    path,
    def: '[]'
  }).then((v: string) => {
    figure.set(JSON.parse(v))
  })
  figure.subscribe((v: Figure[]) => {
    if (v == get(figure)) {
      return
    }
    console.log(v, get(figure))
    return invoke('write_file', { path, content: JSON.stringify(v) })
  })
})
export default figure
