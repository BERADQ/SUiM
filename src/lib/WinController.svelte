<script lang="ts">
  import { exit } from '@tauri-apps/api/process'
  import { appWindow } from '@tauri-apps/api/window'
</script>

<div class="flex flex-content-center justify-end h-full">
  <div
    class="h-full box-border mt3px mb3px filter-drop-shadow flex-row w-full"
    data-tauri-drag-region>
  </div>
  <button
    class="cb"
    on:click={() => {
      appWindow.minimize()
    }}
    ><div class="i-mingcute:minimize-fill dark:text-light text-dark"></div>
  </button>
  <button
    class="cb"
    on:click={() => {
      appWindow.toggleMaximize()
    }}
    ><div class="i-mingcute:square-line dark:text-light text-dark"></div>
  </button>
  <button
    class="x cb"
    on:click={() => {
      appWindow.close()
      exit()
    }}
    ><div class="i-mingcute:close-fill dark:text-light text-dark"></div>
  </button>
</div>

<style lang="postcss">
  .cb {
    user-select: none;
    transition: all 300ms;
    aspect-ratio: 1.3/1;
    height: 100%;
    background-color: transparent;
    border: none;
    color: #eeeeee;
    display: flex;
    justify-content: center;
    align-items: center;

    &:hover {
      background-color: #9993;
      @apply dark:bg-[#eee3];
    }

    &.x:hover {
      background-color: #ee3333aa;
    }
  }
</style>
