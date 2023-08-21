<script lang="ts">
  import Avatar from './Avatar.svelte'
  import pangu from 'pangu'
  export let content: string
  $: content = pangu.spacing(content)
</script>

<div id="mainbox" class="grid">
  <div class="grid-area-[a] w12 h12 mr2">
    <Avatar iName="SUiM" base64={undefined} onclick={() => {}} />
  </div>
  <span class="grid-area-[n] h6 text-gray500">SUiM</span>
  <pre class="grid-area-[i] m0 text-gray800 dark:text-gray300">{content}</pre>
</div>

<style lang="postcss">
  #mainbox {
    grid-template:
      'a a n'
      'a a i'
      '. . i';

    & > pre {
      white-space: pre-wrap;
      word-wrap: normal;
    }
  }
</style>
