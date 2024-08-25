<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
    import ListComponent from '$lib/components/common/ListComponent.svelte';
    export let data
    
    let slug = $page.params.slug

</script>
<div>
    <h1>{slug}</h1>
    <button class="button-less fake-link cursor" on:click={()=>history.back()}>Back</button>
    <ListComponent bind:items={data.data}/>
</div>
