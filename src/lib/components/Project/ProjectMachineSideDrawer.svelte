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
	import { onMount } from "svelte";

    interface ProjectMachineWithState extends ProjectMachine{
        state:string
    }

    export let machines: ProjectMachine[] = []
    export let showSideDrawer = false
    
    let slug = $page.params.slug
    let deployment = $page.params.deployment
    let error: string | null = null
    let isProvisioning = false
    let isStarted=false
    let statusMachines:ProjectMachineWithState[] = []
    
    $:isMachineReady = (machineId:string):boolean=>{
        let m = statusMachines.find(mach=>mach.id === machineId)
        if (!m){
            return false
        }
        if (m.state !== "off" && !"stopping"){
            isStarted = true
        }
        return  m.state === "ready"
    }

    async function handleMachineAction(machineId:string, action:"START"|"STOP"){
        console.log(`Sending '${action}' request for machine `, machineId)
        let funcToCall = ""
        switch (action){

            case "START":
                funcToCall = "start_machine"
                isStarted = true
                break
            case "STOP":
                funcToCall = "stop_machine"
                isStarted = false
                break
            default:
                console.error(`handleMachineAction received invalid action. ${action}`)
                return
            }
        
            try{
                await invoke(funcToCall, {deploymentName:deployment, projectName:slug, machineId})
            }catch(err){
                console.error(`Failed to STOP machine '${machineId}'' due to `,err )
                error = JSON.stringify(err)
                isStarted = !isStarted

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
    
    onMount( ()=>{
            let unsub = setInterval(async ()=>{
                try{

                    let res:string = await invoke("get_machine_status",{deploymentName:deployment, projectName:slug} )
                    let response  = JSON.parse(res)
                    statusMachines = [...response.data]


                    console.log("Success POLL!", res)
                }catch(err){
                    console.error("Error When Polling for Machine state")
                }
                
            },  1000)
            return ()=>{clearInterval(unsub)}
        })
    console.log("side drawer mounted")
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
                {#if isStarted || isProvisioning}  
                <button class="button" on:click={()=>handleMachineAction(machine.id, "STOP")}>Off</button>
                {:else}
                <button class="button"  on:click={()=>handleMachineAction(machine.id, "START")}>On</button> 
                {/if}
            </span>
            <span class="display-block"><b>MachineID:</b> {machine.id}</span>
            <span class="display-block"><b>IP:</b> {machine.ip_addr ?? "-"}</span>
            <span class="display-block"><b>Type:</b> {machine.machine_type}</span>
            <div class="display-flex gap-10 justify-content-center mt-5">
                    <button class="button button-option">Show Runs</button>
                    <button class={`button button-success ${!isMachineReady(machine.id) && "button-success-disabled"}`} 
                        on:click={()=>handleTrain(machine.id)}
                        disabled={!isMachineReady(machine.id)}
                        >Train</button>
            </div>

        </div>
        {/each}
    </div>
</SideDrawer>