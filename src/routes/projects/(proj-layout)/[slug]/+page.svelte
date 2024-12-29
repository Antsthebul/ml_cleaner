<script lang="ts">
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


	<!-- <span>Sync Data</span> <button class="button button-info" on:click={()=>invoke("sync_data",{projectName:slug})}>Sync</button>
	<button on:click={()=>handleGenerateTestTrainData()} class="button button-info">Generate Test/Train Data</button> -->

	<div>
		Some general data
	</div>
	<div>
		More data maybe like total cost or poject
	</div>
	<div>
		Total trainig time of project
	</div>
</div>