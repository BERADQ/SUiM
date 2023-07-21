<script lang="ts">
  import SideBar from './lib/SideBar.svelte'
  import Topbar from './lib/Topbar.svelte'
  import BasePage from './page/BasePage.svelte'
  import NewCard from './page/NewCard.svelte'
  import Settings from './page/Settings.svelte'

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
  <!--  <Topbar/>-->
  <div class="fv">
    <SideBar onchange={change_page} />
    <div
      class={`main-page ${a_count === -1 ? '' : 'ov'}`}
      style="position: relative">
      {#if platform === 'macos'}
        <div
          data-tauri-drag-region
          style="height: 1.7em; width: 100%; position: absolute; top: 0; left: 0 margin-left: calc(var(--main-border) * -0.5); right: 0">
        </div>
      {/if}

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
    height: 100%;
  }

  .fv {
    /*height: calc(100% - var(--topbar-height));*/
    height: 100%;
    width: 100%;
    /* background-color: transparent; */
    display: flex;
  }

  .main-page {
    /* background-color: var(--bg-color1); */
    width: calc(100% - var(--side-width));
    box-sizing: border-box;
    padding: var(--main-border);
    transition: all var(--transition);
  }

  main.macos {
    .main-page {
      @apply bg-gbase-50 dark:bg-gbase-800;
    }
  }

  main:not(.macos) {
    .main-page.ov {
      border-top-left-radius: var(--radius-size);
    }
  }
</style>
