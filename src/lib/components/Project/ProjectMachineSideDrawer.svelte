<style>
    #inner-content{
        text-align: center;
    }
    .machineCard{
        border:1px solid lightgrey;
        box-shadow: 1px 1px 5px lightgrey;
        padding:5px 10px;
        border-radius: 15px;
        width:fit-content;
        text-align: left;
    }
    .alert{
        position: absolute;
        color:red;
        top:0;
        z-index: 99999;
    }
</style>
<script lang="ts">
	import { SideDrawer } from "$lib";
	import type { ProjectMachine } from "$lib/global_types";
	import { invoke } from "@tauri-apps/api/tauri";
	import { page } from '$app/stores';

    export let machines: ProjectMachine[] = []
    export let showSideDrawer = false
    
    let slug = $page.params.slug
    let deployment = $page.params.deployment
    let error: string | null = null
    let isProvisioning = false
    let isStarted=false

    async function handleMachineStop(machineId:string){
        console.log("Sending 'STOP' request for machine ", machineId)

        try{
            await invoke("stop_machine", {deploymentName:deployment, projectName:slug, machineId})
            isStarted = false
        }catch(err){
            console.error(`Failed to STOP machine '${machineId}'' due to `,err )
            error = JSON.stringify(err)
        }
    }

    async function handleTrain(machineId:string){
        console.log("Sending 'train' request for model ",machineId)
        isProvisioning = true
        isStarted = true
        try{

            let val:string = await invoke("train_model", {deploymentName:deployment, projectName:slug, machineId})
            
        }catch(err){
            try{
                let data = JSON.parse(err as string)
                if (data.error){
                    error = data.error
                    console.log("parsed, ", data.error)
                }
            }catch(err){
                console.log("Failed to parse JSON, ",err)
            }            
            console.log("Train request failed. ", err)
        }
    }
</script>
{#if error}
<div class="alert">{error}</div>
{/if}

<SideDrawer bind:showSideDrawer={showSideDrawer}>
    <div id="inner-content">

        <h2>Machines</h2>
        {#each machines as machine}
        <div class="machineCard">
            <span class="display-block">
                <a href={`/machines/${machine.name}`}>
                    {machine.name}
                </a>
                - 
            </span>
            <span class="display-block"><b>MachineID:</b> {machine.id}</span>
            <span class="display-block"><b>IP:</b> {machine.ip_addr ?? "-"}</span>
            <span class="display-block"><b>Type:</b> {machine.machine_type}</span>
            <div class="display-flex gap-10 justify-content-center mt-5">
                    <button class="button button-option">Show Runs</button>
                    <button class={`button button-danger ${!isStarted && "button-danger-disabled"}`}
                        disabled={!isStarted} 
                        on:click={()=>handleMachineStop(machine.id)}>Stop</button>
                    <button class={`button button-success ${isProvisioning && "button-sucess-disabled"}`} 
                        on:click={()=>handleTrain(machine.id)}
                        disabled={isProvisioning}
                        >Train</button>
            </div>

        </div>
        {/each}
    </div>
</SideDrawer>