import {writable, get, type Writable} from 'svelte/store';
import {invoke} from '@tauri-apps/api';

const figure: Writable<any[]> = writable([]);

invoke('get_file', {
  path: './figure.json',
  def: '[]',
}).then(
  (v: string) => {
    figure.set(JSON.parse(v));
  },
);
figure.subscribe(async (v: any[]) => {
  if (v != get(figure)) {
    console.log(v, get(figure));
    await invoke('write_file', {path: './figure.json', content: JSON.stringify(v)});
  }
});
export default figure;
