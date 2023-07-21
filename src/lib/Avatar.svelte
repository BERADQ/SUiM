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
    @apply bg-brand-50/50
    dark:bg-brand-950/50
    transition-all
    duration-300
    overflow-hidden
    p-0 lh-100%
    grid-items-center
    flex flex-justify-center
    aspect-ratio-square w-full
    b-none select-none;
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
      @apply w-full h-full b-rounded-[var(--inner-radius-size)];
    }

    & > span {
      @apply text-brand-400 transition-all w-full text-center [font-size:60%];
    }
    &:hover > span {
      @apply transition-all text-brand-200;
    }
  }
</style>
