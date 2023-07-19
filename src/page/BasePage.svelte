<script lang="ts">
  export let title;
</script>
<div>
  <h1 class="title">
    {title}
  </h1>
  <slot></slot>
</div>
<style lang="postcss">
  .title {
    color: var(--text-color0);
    margin: calc(var(--main-border) * 0.5) var(--main-border);
    font-size: var(--title-size);
  }
</style>