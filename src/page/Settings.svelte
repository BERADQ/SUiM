<script lang="ts">
  import { invoke } from '@tauri-apps/api'
  import MessageBox from '../lib/MessageBox.svelte'
  import Avatar from '../lib/Avatar.svelte'
</script>

<div class="p-2">
  <button on:click={() => invoke('open_devtools')}> open devtools </button>
  <button on:click={() => window.location.reload()}> reload webview </button>
  <button
    on:click={() =>
      document.querySelector('html').toggleAttribute('data-set-background')}
    >toggle background</button>
  <div class="b-red b b-solid w-full h-full flex p2 flex-items-end">
    <div class="h12 w12 mr2">
      <Avatar iName="SUiM" base64="" onclick={() => {}} />
    </div>
    <MessageBox />
  </div>
</div>
