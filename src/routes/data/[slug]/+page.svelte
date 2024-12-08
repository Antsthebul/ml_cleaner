<script lang="ts">
	import { page } from '$app/stores';
	import { ImageDataAPI, ProjectAPI } from '$lib';
	import { Loader } from '$lib';
	import ImageDataList from '$lib/components/ImageData/ImageDataList.svelte';
	import { invoke } from '@tauri-apps/api/core';
    export let data
    
    let slug = $page.params.slug
    let projectName = $page.url.searchParams.get("project")
    let isLoading = false

    $: imageData = data

    async function deleteObject(filePath:string){
        console.log(`Deleting ${filePath} from ${projectName}`)    
        try{

            await invoke("remove_image", {projectName,filePath})
        }catch(err){
            console.error(`Unable to delete object at ${filePath}. ${err}`)
        }
    }

    async function handlePage(page:string){
        isLoading = true
        let res = await ImageDataAPI.getUnverifiedImages(projectName as string, slug, page)
        imageData = {data:res}
        isLoading=false
    }

    async function handleKeep(filePath:string) {
        try{
            await invoke("keep_data_for_class", {projectName, filePath})
        }catch(err){
            console.error(`HandleKeepFailed. ${err}`)
        }
    }

</script>
<div>
    {#if isLoading}
    <Loader />
    {/if}
    <h1>{slug}</h1>
    <button class="button-less fake-link cursor" on:click={()=>history.back()}>Back</button>
    <ImageDataList data={imageData.data} handleDelete={deleteObject} handlePage={handlePage} handleKeep={handleKeep}/>
</div>
