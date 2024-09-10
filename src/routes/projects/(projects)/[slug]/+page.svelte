<script lang="ts">
	import DependVarWindow from "$lib/components/Project/DependVarWindow.svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	import { page } from "$app/stores";

	export let data
    let slug = $page.params.slug
</script>
<div class="mb-10">
	<span>Sync Data</span> <button class="button button-info" on:click={()=>invoke("sync_data",{projectName:slug})}>Sync</button>
</div>
<DependVarWindow listOfClasses={data.classes}/>