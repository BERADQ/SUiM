<script lang="ts">
  import {exit} from '@tauri-apps/api/process';
  import {appWindow} from '@tauri-apps/api/window';
</script>

<div class="ww">
  <div data-tauri-drag-region class="main"></div>
  <button
      class="i cb"
      on:click={() => {
      appWindow.minimize();
    }}>–
  </button
  >
  <button
      class="b cb"
      on:click={() => {
      appWindow.toggleMaximize();
    }}>▢
  </button
  >
  <button
      class="x cb"
      on:click={() => {
      exit();
    }}>⨉
  </button
  >
</div>

<style lang="postcss">
  .ww {
    background-color: var(--bg-color0);
    box-sizing: border-box;
    width: 100%;
    display: flex;
  }

  .main {
    height: calc(var(--topbar-height) - 6px);
    display: flex;
    width: 100%;
    margin: 3px;
  }

  .cb {
    user-select: none;
    transition: all 300ms;
    width: var(--topbar-height);
    height: var(--topbar-height);
    background-color: transparent;
    border: none;
    color: #eeeeee;

    &:hover {
      background-color: #eeeeee33;
    }

    &.x:hover {
      background-color: #ee3333aa;
    }
  }


</style>
