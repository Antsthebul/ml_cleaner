<style>
    #indicator{
        height: 10px;
        width: 10px;
        border-radius: 50%;
        display: inline-block;
    }

    .red{
        background-color: red;
    }
    .green{
        background-color: rgb(9, 215, 9);
    }


</style>
<script lang="ts">

    import {invoke} from "@tauri-apps/api/tauri"
    import {writable} from "svelte/store";
    import {onMount} from 'svelte'
    import {loadProjects} from "../store"


    interface Machine{
        id:string,
        state:string,
        name:string
    }
    interface ConfigurationResponse{
        configuration:Configuration
    }
    interface Configuration{
        default_machine:string | null
    }
    

    $: runningState = {} as {[key:string]:boolean};
    $: machines = [] as Machine[];
    $: configuration={default_machine:null} as Configuration;

    function searchBucket(){
        invoke("search_bucket") 
    }
    function listMachines(){
        invoke("list_machines")
        .then((res:any)=>{
            machines = JSON.parse(res).data
        })
        .catch((err:any)=>console.error("Failed to list machines due to: ", err))
    }
    function getStatus(){
        invoke("get_status")
    }

    function isRunning(machineId:string){
        return Boolean(runningState[machineId])

    }
    async function getConfig(){
        let raw_response:string = await invoke("get_config")
        let result:{data:ConfigurationResponse} = JSON.parse(raw_response)
        configuration = result.data.configuration ?? {}
    }
    async function updateDefaultMachine(machineId:string){
        await invoke("update_configuration_file_command", {machineId})
        await getConfig()

    }
    async function getMachineState(machineId:string){
        let result:{[key:string]: boolean} = await invoke("is_running", {machineId})
        let machineStatus = result?.data ?? false
        runningState = {...runningState, machineId:machineStatus}

    }

    async function startMachine(machineId:string){
        await invoke("start_machine", {machineId})
    }
    
    // setInterval(isRunning,5000)
    
    onMount(async ()=>{
       await getConfig()
       await listMachines()
       await loadProjects()
    })
</script>

<div>
    <p>Default Machine: {configuration.default_machine}</p>
    {#if configuration.default_machine}
    <div id="indicator" class={`${runningState[configuration.default_machine] ? "green":"red"}`}>
    </div>
    {/if}
<div>

    <button on:click={searchBucket}>Search bucket</button>
    <button on:click={listMachines}>Refresh Machines List</button>
    <button on:click={getStatus}>Get Status</button>
</div>

</div>
<h1>Machines</h1>
<div>
    {#each machines as machine, i}
    <div>
        <h3>Machine # {i+1}</h3>
        <button on:click={()=>getMachineState(machine.id)}>Get State</button>
        <p>ID: {machine.id}</p>
        <p>Machine: {machine.name}</p>
        <p>State: {machine.state}</p>
    </div>
    <button disabled={isRunning(machine.id)} on:click={()=>startMachine(machine.id)}>Start</button>
    <button disabled={!isRunning(machine.id)}>Stop</button>
    <button 
        on:click={()=>updateDefaultMachine(machine.id === configuration.default_machine ? "resetDefaultMachine":machine.id)}
        >{machine.id === configuration.default_machine ? "Remove" :"Set" } as default</button>
    {/each}
</div>