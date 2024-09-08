<script lang="ts">
	import DependVarWindow from "$lib/components/Project/DependVarWindow.svelte";
	import { get } from "svelte/store";
	import { projectDetailStore } from "../../../../store";
	import { invoke } from "@tauri-apps/api/tauri";
	import { page } from "$app/stores";

    let slug = $page.params.slug
    let projectDetail = get(projectDetailStore)
</script>
<div class="mb-10">
	<span>Sync Data</span> <button class="button button-info" on:click={()=>invoke("sync_data",{projectName:slug})}>Sync</button>
</div>
<DependVarWindow listOfClasses={projectDetail.classes}/>