<script lang="ts">
    import DownArrow from "~icons/material-symbols/arrow-drop-down-rounded"
    import UpArrow from "~icons/material-symbols/arrow-drop-up-rounded"

    export let up :boolean
    export let handleClick 
</script>
<span>
    <button class="cursor button-less" on:click={handleClick}>

        {#if up}
        <UpArrow />
        {:else}
        <DownArrow />
        {/if}
    </button>
</span>