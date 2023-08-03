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
  <div
    class="b-red/20 b b-solid w-full h-full flex p2 flex-items-end box-border">
    <OrdinaryMessage
      content={`你好，这是一条消息，我是SUiM。
这是测试消息元素，消息目前均会被自动增加盘古之白。
该消息元素使用grid实现，应该还有其他变体：连续消息（取消头像显示但是保证对齐）同时，这里应该有自动折行，如果没有就要改。样式还未确定，仍需修订。
`} />
  </div>
</div>
