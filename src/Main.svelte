<script lang="ts">
  import {get} from 'svelte/store';
  import figure from './stores/Figure';
  import BasePage from './page/BasePage.svelte';
  import NewCard from './page/NewCard.svelte';

  const onclick = () => {
    console.log(get(figure));
  };
</script>

<div class="main">
  <BasePage title="Test">
    <NewCard/>
  </BasePage>
</div>

<style>
  .main {
    background-color: transparent;
    width: calc(100% - var(--side-width));
    box-sizing: border-box;
    margin: var(--main-border);
  }
</style>
