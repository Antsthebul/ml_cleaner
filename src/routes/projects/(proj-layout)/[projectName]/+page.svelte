<style>
	#add_node:hover{
		animation: slide-up 1s infinte;
	}

	@keyframes slide-up{
		from {
			transform: translateY(0px);
		}

		to {
			transform: translateY(-10px);
		}
	}
</style>
<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { page } from "$app/stores";
	import AddNodeIcon from "~icons/hugeicons/node-add"
	import { goto } from "$app/navigation";

	export let data

    let curProject = data.project ?? {}

    let projectName = $page.params.projectName

	async function handleGenerateTestTrainData(){
        await invoke("generate_test_train_data", {projectName:projectName})
        return
    }
	console.log("Testing in Project/projectName - page.svelte")
</script>
<div class="mb-10">


	<!-- <span>Sync Data</span> <button class="button button-info" on:click={()=>invoke("sync_data",{projectName:projectName})}>Sync</button>
	<button on:click={()=>handleGenerateTestTrainData()} class="button button-info">Generate Test/Train Data</button> -->
    <div class="display-flex justify-content-center gap-10">

		<h3>Environments</h3>
		<button 
			id="add_node" 
			class="button align-self-center bg-transparent border-light"
			on:click={()=>goto(`/projects/${curProject}/newEnvironment`)}>

			<AddNodeIcon />
		</button>

	</div>
    <div id="environment_section">

        <!-- {#each curProject.deployments as dep}
            <button 
            class={`button-link ${currentDeployment === dep.name ? "button-link-disabled":''} cursor`}
                on:click={async ()=>handleNavigateDeployment(dep.name)}
            disabled={currentDeployment === dep.name}
            >{dep.name}</button>
        {/each} -->
    </div>
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