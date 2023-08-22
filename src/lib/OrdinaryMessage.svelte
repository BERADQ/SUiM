<script lang="ts">
  import Avatar from './Avatar.svelte'
  import pangu from 'pangu'
  export let content: string
  $: content = pangu.spacing(content)
</script>

<div id="mainbox" class="grid">
  <div class="grid-area-[a] w44px h44px mr2">
    <Avatar iName="SUiM" base64={undefined} onclick={() => {}} />
  </div>
  <span class="grid-area-[n] h6 text-gray500 font-sans">SUiM</span>
  <pre class="grid-area-[i] m0 text-gray800 dark:text-gray300 line-height-[1.6] font-sans">{content}</pre>
</div>

<style lang="postcss">
  #mainbox {
    grid-template:
      'a . n'
      'a i i'
      'a i i';

    & > pre {
      white-space: pre-wrap;
      word-wrap: normal;
    }
  }
</style>
