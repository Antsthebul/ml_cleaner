<style>
    @import '../../../app.css';

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
        top:0;
        z-index: 99999;

    }
    .indicatorLight{
        border-radius: 50%;
        height:10px;
        width:10px;
        display: inline-block;
    }
    .green{
        background-color: var(--success-green);
    }
    .red{
        background-color: var(--red);
    }
    .warning{
        background-color: var(--warning);
    }

</style>
<script lang="ts">
	import { SideDrawer } from "$lib";
	import type { ProjectMachine, TrainingData } from "$lib/global_types";
	import { invoke } from "@tauri-apps/api/tauri";
	import { page } from '$app/stores';
	import { onMount } from "svelte";


    interface ProjectMachineWithState extends ProjectMachine{
        state:MachineState
    }
    type MachineState = "ready"|"starting"|"stopping"|"off"|"training"
    type RequestedMachineState = "START"|"TRAIN"|"STOP_TRAIN"|"OFF"

    export let machines: ProjectMachine[] = []
    export let showSideDrawer = false
    export let trainingResults: TrainingData[]
    
    let slug = $page.params.slug
    let deployment = $page.params.deployment
    let error: string | null = null
    let statusMachines:ProjectMachineWithState[] = []
    let machineState:MachineState = "off"
    let requestedMachineState:{[machineId: string]:RequestedMachineState} = {}
    let trainingResultPollerUnsub:any = null

    $: isDownloadModelButtonDisabled = (machineId:string):boolean=>{
        let m = statusMachines.find(m=>m.id === machineId)
        let reqMachAction = requestedMachineState[machineId]

        if (!m || m && m.state == "off"){
            return true
        }
        if ((isMachineReady(machineId) && reqMachAction === "TRAIN" )||["training", "starting"].includes(m.state)){
            return true
        }
        return false
    }


    $: handleMachineIndicator = (machineId:string):string=>{
        let reqMachAction = requestedMachineState[machineId]
        let m = statusMachines.find(m=>m.id === machineId)
        if (!m){
            return "red"
        }
        if(["off", "starting"].includes(m.state) && reqMachAction === "START"){
            return "warning"
        }
        if (isMachineReady(machineId) || m.state === "training"){
            return "green"
        }
       return "red"
    }
    $: updateRequestedMachineState = (machineId:string, action:RequestedMachineState) =>{
        requestedMachineState = {...requestedMachineState, [machineId]:action}
    }
    
    $:isMachineReady = (machineId:string):boolean=>{
        let m = statusMachines.find(mach=>mach.id === machineId)
        if (!m){
            return false
        }
        return  m.state === "ready"
    }
    $:showMachineOnButton = (machineId:string):boolean=>{
        // We dont want to be able to 'immediately' turn 
        // a machine on, if we just requested it off, so we
        // check the requested state 
        let m = statusMachines.find(mach=>mach.id === machineId)
        let reqMachAction = requestedMachineState[machineId]


        if (!m){ // Initial page load
            return true
        }else{
            if (m.state === "off" && (!reqMachAction|| reqMachAction === "OFF")){
                return true
            }
        }
        return false
    }

    $:showTrainingButton = (machineId:string):boolean => {
        // Button may be disabled but we still want to
        // show it as a 'default' option
        let m = statusMachines.find(m=>m.id === machineId)
        let reqMachineAction = requestedMachineState[machineId]
        if (!m){ // Initial page load (Button visible but disabled)
            return true
        }
        if (m.state === "training" && reqMachineAction === "STOP_TRAIN"){
            return true
        }
        if (m.state !== "training" && reqMachineAction !== "TRAIN" ){
            return true
        }
        return false
    }

    $:pollForTrainingResults = async (machineId:string)=>{
        let m = statusMachines.find(m=>m.id === machineId)

        if (m!.state === "training"){

            try{
                let response:string = await invoke("get_training_results", {deploymentName:deployment, projectName:slug, machineId})
                console.log("TRINGIN RESULTS POLLL", response)
                let result = JSON.parse(response)
                console.log("not help ", result)
                if (result){

                    trainingResults = [...trainingResults, {machineId:machineId, trainData:result.data}]
                }
            }catch(e){
                try{

                    let error_data = JSON.parse(e as string)
                    if (error_data.error !== "No data"){
    
                        console.error("Failed to poll training results", e)
                    }
                    return
                }catch(e){
                    
                }
                console.error("Poll for training result failed ot unknown")
            }
        }
    }

    /**Set the requestedMachineState */
    async function handleMachineAction(machineId:string, action:"START"|"STOP"){
        console.log(`Sending '${action}' request for machine `, machineId)
        let funcToCall;
        switch (action){

            case "START":
                funcToCall = "start_machine"
                updateRequestedMachineState(machineId, "START")
                setTimeout(()=>{
                    alert("did that machine start?")
                }, 1200000)

                break
            case "STOP":
                funcToCall = "stop_machine"
                updateRequestedMachineState(machineId,"OFF")
                clearInterval(trainingResultPollerUnsub)

                break
            default:
                console.error(`handleMachineAction received invalid action. ${action}`)
                return
            }
        
            try{
                await invoke(funcToCall, {deploymentName:deployment, projectName:slug, machineId})
            }catch(err){
                console.error(`Failed to ${action} machine '${machineId}'' due to `,err )
                error = JSON.stringify(err)
            }
    }

    async function handleTrainAction(machineId:string, action:"START"|"STOP"){
        console.log(`Sending 'train' request to '${action}' model `,machineId)
        let command;
        let prevTrainState = machineState
        switch (action){
            case "START":
                command = "train_model"
                updateRequestedMachineState(machineId, "TRAIN")
                trainingResultPollerUnsub = setInterval(()=>{
                    pollForTrainingResults(machineId)
                }, 2000)                
                break
            case "STOP":
                command = "stop_train_model"
                updateRequestedMachineState(machineId, "STOP_TRAIN")
                clearInterval(trainingResultPollerUnsub)
                break
        }
        try{
            await invoke(command, {deploymentName:deployment, projectName:slug, machineId})
        }catch(err){
            machineState = prevTrainState
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

    async function downloadModel(machineId:string){
        try{

            let response:string = await invoke("download_model", {deploymentName:deployment, projectName:slug, machineId})
        }catch(e){
            console.error("failed to download model")
        }
    }

    onMount( ()=>{
        console.log("Project side bar mounted")
        let unsub = setInterval(async ()=>{
            try{

                let res:string = await invoke("get_machine_status",{deploymentName:deployment, projectName:slug} )
                let response:{data:ProjectMachineWithState[]}  = JSON.parse(res)
                
                statusMachines = [...response.data]

                let updateMachines = []
                for (let m of machines){
                    let s = statusMachines.find(prevMach => prevMach.id === m.id)
                    if (s){
                        m.ip_address = s.ip_address
                    }
                    updateMachines.push(m)
                }
                machines = [...updateMachines]

                console.log("Success POLL!", res)
            }catch(err){
                console.error("Error When Polling for Machine state")
            }
            
        },  1000)
        return ()=>{
            clearInterval(unsub)
            if (trainingResultPollerUnsub)clearInterval(trainingResultPollerUnsub)
        }
    })

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
                {#if showMachineOnButton(machine.id)}  
                <button class="button"  on:click={()=>handleMachineAction(machine.id, "START")}>On</button> 
                {:else}
                <button class="button" on:click={()=>handleMachineAction(machine.id, "STOP")}
                    disabled={requestedMachineState[machine.id] === "OFF"}>Off</button>
                {/if}
                <span class={`indicatorLight ${handleMachineIndicator(machine.id)}`}></span>
            </span>
            <span class="display-block"><b>MachineID:</b> {machine.id}</span>
            <span class="display-block"><b>IP:</b> {machine.ip_address ?? "-"}</span>
            <span class="display-block"><b>Type:</b> {machine.machine_type}</span>
            <div class="display-flex gap-10 justify-content-center mt-5">
                <button class="button button-option">Show Runs</button>
                {#if showTrainingButton(machine.id)}
                <button class={`button button-success ${!isMachineReady(machine.id) && "button-success-disabled"}`} 
                    on:click={()=>handleTrainAction(machine.id, 'START')}
                    disabled={!isMachineReady(machine.id)}
                    >Train</button>
                {:else}
                <button class={`button button-danger ${requestedMachineState[machine.id] === "STOP_TRAIN" && "button-danger-disabled"}`}
                    on:click={()=>handleTrainAction(machine.id, 'STOP')}
                    disabled={requestedMachineState[machine.id] === "STOP_TRAIN"}
                    >Stop Train</button>
                {/if}
            </div>
            <div class="justify-content-center">

                <button 
                    class={`button button-info mx-auto mt-5 ${isDownloadModelButtonDisabled(machine.id) && "button-info-disabled"}`}
                    disabled={isDownloadModelButtonDisabled(machine.id)}
                    on:click={()=>downloadModel(machine.id)}>
                    Download Model
                </button>
            </div>

        </div>
        {/each}
    </div>
</SideDrawer>