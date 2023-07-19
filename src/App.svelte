<script lang="ts">
  import SideBar from './lib/SideBar.svelte'
  import Topbar from './lib/Topbar.svelte'
  import BasePage from './page/BasePage.svelte'
  import NewCard from './page/NewCard.svelte'

  import figure from './stores/Figure'
  import { get } from 'svelte/store'

  let title
  const change_page = (count: number) => {
    if (count === -1 || count === -2) {
      title = count === -1 ? 'Setting' : 'Add'
    } else {
      title = get(figure)[count].name
      console.log(title)
    }
  }

  const platform = import.meta.env.TAURI_PLATFORM
</script>

<main class="container">
  <!--  <Topbar/>-->
  <div class="fv">
    <SideBar onchange={change_page} />
    <div class="main-page" style="position: relative">
      {#if platform === 'macos'}
        <div
          data-tauri-drag-region
          style="height: 1.7em; width: 100%; position: absolute; top: 0; left: 0 margin-left: calc(var(--main-border) * -0.5); right: 0">
        </div>
      {/if}

      <BasePage {title}>
        <NewCard />
      </BasePage>
    </div>
  </div>
</main>

<style>
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
    background-color: var(--bg-color1);
    width: calc(100% - var(--side-width));
    box-sizing: border-box;
    padding: var(--main-border);
  }
</style>
