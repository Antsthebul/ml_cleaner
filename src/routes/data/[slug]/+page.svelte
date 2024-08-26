<script lang="ts">
	import { page } from '$app/stores';
	import ImageDataList from '$lib/components/ImageData/ImageDataList.svelte';
    export let data
    
    let slug = $page.params.slug

</script>
<div>
    <h1>{slug}</h1>
    <button class="button-less fake-link cursor" on:click={()=>history.back()}>Back</button>
    <ImageDataList items={data.data}/>
</div>
