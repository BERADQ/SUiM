<script lang="ts">
  export let title
</script>

<div>
  <h1 class="title">
    {title}
  </h1>
  <slot />
</div>

<style lang="postcss">
  .title {
    color: var(--text-color0);
    margin: 0 var(--main-border);
    padding: calc(var(--main-border) * 0.5) 0;
    font-size: var(--title-size);
    border-bottom: 2px solid var(--bg-ac-color0);
  }
</style>
