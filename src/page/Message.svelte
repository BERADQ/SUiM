<script lang="ts">
  import OrdinaryMessage from '../lib/OrdinaryMessage.svelte'
</script>

<div
  class="w-full h-full flex p10px flex-items-end flex-col box-border">
  <div class="message">
    <OrdinaryMessage
      content={`你好，这是一条消息，我是SUiM。
这是测试消息元素，消息目前均会被自动增加盘古之白。
该消息元素使用grid实现，应该还有其他变体：连续消息（取消头像显示但是保证对齐）同时，这里应该有自动折行，如果没有就要改。样式还未确定，仍需修订。
`} />
  </div>
  <div class="message">
    <OrdinaryMessage
      content={`你好，这是一条消息，我是SUiM。
这是测试消息元素，消息目前均会被自动增加盘古之白。
该消息元素使用grid实现，应该还有其他变体：连续消息（取消头像显示但是保证对齐）同时，这里应该有自动折行，如果没有就要改。样式还未确定，仍需修订。
`} />
  </div>
</div>

<style lang="postcss">
  .message {
    margin-bottom: var(--radius-size);
  }
</style>
