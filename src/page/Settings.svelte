<script lang="ts">
  import { invoke } from '@tauri-apps/api'
</script>

<div class="p-2">
  <button
    on:click={() =>
      document.documentElement.toggleAttribute('data-set-background')}>
    toggle transparent
  </button>
  <button on:click={() => invoke('open_devtools')}> open devtools </button>
  <button on:click={() => window.location.reload()}> reload webview </button>
</div>
