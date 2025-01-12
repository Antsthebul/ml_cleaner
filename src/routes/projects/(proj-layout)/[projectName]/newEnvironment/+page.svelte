<script lang="ts">
	import DeploymentAPI from "$lib/api/DeploymentAPI";
	import type { NewDeployment } from "$lib/global_types";


    import { newDeploymentName } from "$lib/store";
    


    let newDeployment:NewDeployment = {name:"", description:"", tags:[], project_id:-1}

    $:{
        newDeploymentName.set(newDeployment.name)
    }
    async function saveNewDeployment(){
        if (newDeployment.project_id > 0){

            await DeploymentAPI.createDeployment(newDeployment) 
        }
    }

 
</script>

<h1>New Project</h1>

<label for="">Name</label>
<input type="text" bind:value={newDeployment.name}/>
<label for="">Description</label>
<textarea id="" cols="30" rows="10" bind:value={newDeployment.description}></textarea>
<button class="button button-info" on:click={saveNewDeployment}>Ok</button>
