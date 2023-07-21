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

    @apply transition-all duration-300;
    @apply flex flex-justify-center;
    @apply overflow-hidden p-0 lh-100% grid-items-center aspect-ratio-square w-full b-none select-none;
    @apply cursor-pointer;
    @apply bg-gbase-100 dark:bg-gbase-500;
    @apply hover:bg-brand-200 hover:dark:bg-brand-700;

    & > img {
      @apply w-full h-full b-rounded-[var(--inner-radius-size)];
    }

    & > span {
      @apply text-brand-300;
      @apply transition-all w-full text-center;
      font-size: 60%;

      @apply hover:text-brand-500 hover:dark:text-brand-300;
    }
  }
</style>
