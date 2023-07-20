<script lang="ts">
  export let onclick: () => any
</script>

<button class="main" on:click={onclick}>
  <svg
    class="icon text-neutral-500"
    viewBox="0 0 1024 1024"
    version="1.1"
    xmlns="http://www.w3.org/2000/svg"
    width="40%"
    height="40%">
    <path
      d="M960 448H576V64a64 64 0 0 0-128 0v384H64a64 64 0 0 0 0 128h384v384a64 64 0 0 0 128 0V576h384a64 64 0 0 0 0-128z"
      fill="currentColor" />
  </svg>
</button>

<style>
  .main {
    font-size: 40px;
    border: none;
    /*background-color: var(--bg-ac-color0);*/
    background-color: transparent;
    width: 100%;
    aspect-ratio: 1/1;
    display: flex;
    justify-content: center;
    align-items: center;
    line-height: 100%;
    border-radius: var(--inner-radius-size);
    padding: 0;
  }
</style>
