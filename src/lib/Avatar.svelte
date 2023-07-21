<script lang="ts">
  export let onclick: () => any
  export let base64: string
  export let iName: string
</script>

<button class="main" on:click={onclick}>
  {#if !!base64}
    <img src={`data:image/jpeg;base64,${base64}`} alt={iName} />
  {:else}
    <span>{iName.at(0)}</span>
  {/if}
</button>

<style lang="postcss">
  .main {
    font-size: var(--ava-title);
    border-radius: var(--inner-radius-size);
    transition: all var(--transition);

    @apply bg-brand-50/50 dark:bg-brand-950/50 overflow-hidden p-0 lh-100% grid-items-center flex flex-justify-center aspect-ratio-square w-100% b-none select-none;
    &:not(.macos) {
      @apply bg-brand-200/40 dark:bg-brand-950/50;
    }

    &:hover {
      @apply bg-brand-200 dark:bg-brand-800 cursor-pointer;
      &:not(.macos) {
        @apply bg-brand-200 dark:bg-brand-800;
      }
    }

    & > img {
      width: 100%;
      height: 100%;
      image-rendering: crisp-edges;
      border-radius: var(--inner-radius-size);
    }

    & > span {
      @apply text-brand-400;
      @apply transition-all;
      width: 100%;
      text-align: center;
      font-size: 60%;
    }
    &:hover > span {
      @apply transition-all;
      @apply text-brand-500;
    }
  }
</style>
