<script lang="ts">
	import { fade } from "svelte/transition";

    export let visible:boolean
    export let text

</script>
{#if visible}
<span in:fade={{duration:300}}>
    {text}
</span>
{/if}