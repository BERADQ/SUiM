<script lang="ts">
  export let title:string;
</script>

<div class="main-box">
  <h1 class="title">
    {title}
  </h1>
  <slot />
</div>

<style lang="postcss">
  .title {
    @apply text-brand-700 dark:text-brand-300;
    margin: 0 var(--main-border);
    padding: calc(var(--main-border) * 0.5) 0;
    font-size: var(--title-size);
    border-bottom: 2px solid var(--bg-ac-color0);
  }
</style>
