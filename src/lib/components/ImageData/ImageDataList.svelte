<script lang="ts">
	import type { ImageData, ImageDataCollection } from "$lib/global_types";
	import ListComponent from "../common/ListComponent.svelte";
	import ImageDataListItem from "./ImageDataListItem.svelte";

    export let data: ImageDataCollection
	export let handleDelete:(filePath:string)=>void
	export let handleKeep:(filePath:string) => void
	export let handlePage:any

	async function handleDeleteObject(filePath:string){
		let images = data.images.filter(item=>item.file_path !== filePath)
		data = {...data, images}
		await handleDelete(filePath)
	}
	
	async function handleKeepObject(filePath:string){
		let images = data.images.map(item=>{
			if (item.file_path === filePath){
				item.verified = true
			}
			return item
		})
		data.images = images

		await handleKeep(filePath)
	}
</script>

<ListComponent bind:data={data}  
	handleDelete={handleDeleteObject}
	handleKeep={handleKeepObject}
	renderItem={ImageDataListItem}
	handlePage={handlePage}
	/>