<script lang="ts">
  import { invoke } from '@tauri-apps/api'
  import OrdinaryMessage from '../lib/OrdinaryMessage.svelte'
  import Avatar from '../lib/Avatar.svelte'
</script>

<div class="p-2">
  <button on:click={() => invoke('open_devtools')}> open devtools </button>
  <button on:click={() => window.location.reload()}> reload webview </button>
  <button
    on:click={() =>
      document.querySelector('html').toggleAttribute('data-set-background')}
    >toggle background</button>

</div>

