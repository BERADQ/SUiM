<script lang="ts">
  import SideBar from "./lib/SideBar.svelte";
  import Topbar from "./lib/Topbar.svelte";
  import Main from "./Main.svelte";
</script>

<main class="container">
  <Topbar />
  <div class="fv">
    <SideBar />
    <Main />
  </div>
</main>

<style>
  main {
    height: 100%;
  }
  .fv {
    height: calc(100% - var(--topbar-height));
    width: 100%;
    background-color: var(--bg-color1);
    display: flex;
  }
</style>
