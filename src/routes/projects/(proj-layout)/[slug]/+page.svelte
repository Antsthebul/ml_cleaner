<script lang="ts">
	import DependVarWindow from "$lib/components/Project/DependVarWindow.svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { page } from "$app/stores";

	export let data
    let slug = $page.params.slug

	async function handleGenerateTestTrainData(){
        await invoke("generate_test_train_data", {projectName:slug})
        return
    }
	console.log("Testing in Project/slug - page.svelte")
</script>
<div class="mb-10">
	<div class="mb-10 mt-10 display-flex">
		<div class="w-50">
			<span class="display-block">Train file</span>
			<span class="display-block">&nbsp;&nbsp;<b>Path:</b> {data.project.train_file.path}</span>
			<span class="display-block">&nbsp;&nbsp;<b>Exists:</b> {data.project.train_file.exists}</span>
		</div>
		<div class="w-50">
			<span class="display-block mt-10">Test File</span>
			<span class="display-block">&nbsp;&nbsp;<b>Path:</b> {data.project.test_file.path}</span>
			<span class="display-block">&nbsp;&nbsp;<b>Exists:</b> {data.project.test_file.exists}</span>
		</div>
	</div>

	<span>Sync Data</span> <button class="button button-info" on:click={()=>invoke("sync_data",{projectName:slug})}>Sync</button>
	<button on:click={()=>handleGenerateTestTrainData()} class="button button-info">Generate Test/Train Data</button>

</div>
<DependVarWindow listOfClasses={data.classes}/>