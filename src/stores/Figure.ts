import {writable, get, type Writable} from 'svelte/store';
import {invoke} from '@tauri-apps/api';

const content: any[] = JSON.parse(await invoke('get_file', {
  path: './figure.json',
  def: '[]',
}));
console.log('get file');
const figure: Writable<any[]> = writable(content);
figure.subscribe(async (v: any[]) => {
  if (v != get(figure)) {
    console.log(v, get(figure));
    await invoke('write_file', {path: './figure.json', content: JSON.stringify(v)});
  }
});
export default figure;
