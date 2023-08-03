<script lang="ts">
  import { OS } from './common'
  import SideBar from './lib/SideBar.svelte'
  import BasePage from './page/BasePage.svelte'
  import NewCard from './page/NewCard.svelte'
  import Settings from './page/Settings.svelte'
  import WinController from './lib/WinController.svelte'
  import figure from './stores/Figure'
  import { get } from 'svelte/store'

  let title
  let a_count
  const change_page = (count: number) => {
    a_count = count
    if (count === -1 || count === -2) {
      title = count === -1 ? 'Setting' : 'Add'
    } else {
      title = get(figure)[count].name
      console.log(title)
    }
  }

  const platform = import.meta.env.TAURI_PLATFORM
</script>

<main class={`container ${platform}`}>
  <div class="fv">
    <SideBar onchange={change_page} />
    <div
      class={`main-page ${a_count === -1 ? '' : 'ov'}`}
      style="position: relative">
      <div class="h-30px w-full absolute top-0 left-0">
        {#if platform == OS.Windows}
          <WinController />
        {:else if platform == OS.MacOS}
          <div class="h-full w-full" data-tauri-drag-region></div>
        {/if}
      </div>
      <BasePage {title}>
        {#if a_count === -1}
          <Settings />
        {:else}
          <NewCard />
        {/if}
      </BasePage>
    </div>
  </div>
</main>

<style lang="postcss">
  main {
    @apply h-full;
  }
  .fv {
    height: 100%;
    width: 100%;
    /* background-color: transparent; */
    display: flex;
  }

  .main-page {
    /* background-color: var(--bg-color1); */
    @apply bg-gbase-50 dark:bg-gbase-800;
    width: calc(100% - var(--side-width));
    box-sizing: border-box;
    padding: var(--main-border);
    transition: all var(--transition);
  }
</style>
