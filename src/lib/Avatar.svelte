<script lang="ts">
  export let onclick: () => any;
  export let base64: string;
  export let iName: string;
</script>

<button class="main" on:click={onclick}>
  {#if !!base64}
    <img src={`data:image/jpeg;base64,${base64}`} alt={iName}/>
  {:else}
    <span>{iName.at(0)}</span>
  {/if}
</button>

<style lang="postcss">
  .main {
    user-select: none;
    font-size: var(--ava-title);
    border: none;
    width: 100%;
    aspect-ratio: 1/1;
    display: flex;
    justify-content: center;
    align-items: center;
    line-height: 100%;
    border-radius: var(--inner-radius-size);
    padding: 0;
    overflow: hidden;
    background-color: var(--bg-ac-color0);
    transition: all var(--transition);

    &:hover {
      box-shadow: var(--shadow-color) 0 0 var(--shadow-size) inset;
      background-color: var(--hover--inner-color);
      cursor: pointer;
    }

    & > img {
      width: 100%;
      height: 100%;
      image-rendering: crisp-edges;
      border-radius: var(--inner-radius-size);
    }

    & > span {
      color: var(--text-color0);
      width: 100%;
      text-align: center;
      font-size: 60%;
    }
  }
</style>
