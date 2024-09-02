<script lang="ts">
	import type { ImageData } from "$lib/global_types";
	import ListComponent from "../common/ListComponent.svelte";
	import ImageDataListItem from "./ImageDataListItem.svelte";

    export let data:{images:ImageData[], next_page?:string, previous_page?:string}
	export let handleDelete:(filePath:string)=>void
	export let handlePage:any

	async function handleDeleteObject(filePath:string){
		let images = data.images.filter(item=>item.file_path !== filePath)
		data = {...data, images}
		await handleDelete(filePath)
	}
	
</script>

<ListComponent bind:data={data}  
	handleDelete={handleDeleteObject} 
	renderItem={ImageDataListItem}
	handlePage={handlePage}
	/>