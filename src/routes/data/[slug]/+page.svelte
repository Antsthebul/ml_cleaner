<script lang="ts">
	import { page } from '$app/stores';
	import ImageDataList from '$lib/components/ImageData/ImageDataList.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
    export let data
    
    let slug = $page.params.slug

    async function deleteObject(filePath:string){
        try{

            await invoke("delete_data", {filePath})
        }catch(err){
            console.error(`Unable to delete object at ${filePath}. ${err}`)
        }
    }
</script>
<div>
    <h1>{slug}</h1>
    <button class="button-less fake-link cursor" on:click={()=>history.back()}>Back</button>
    <ImageDataList items={data.data} handleDelete={deleteObject}/>
</div>
