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
    import {onMount} from 'svelte'

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

    $: runningState = false;
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
    }
    function getStatus(){
        invoke("get_status")
    }

    async function isRunning(){
        let result:{[key:string]: boolean} = await invoke("is_running")
        runningState = result?.data ?? false

    }
    async function getConfig(){
        let raw_response:string = await invoke("get_config")
        let result:{data:ConfigurationResponse} = JSON.parse(raw_response)
        configuration = result.data.configuration ?? {}
    }
    async function updateDefaultMachine(machineId:string){
        await invoke("update_default_machine", {machineId})
        await getConfig()

    }
    
    // setInterval(isRunning,5000)
    
    onMount(async ()=>{
       await getConfig()
       await listMachines()
    })
</script>

<div>
    <p>Default Machine: {configuration.default_machine}</p>
    {#if configuration.default_machine}
    <div id="indicator" class={`${runningState ? "green":"red"}`}>
    </div>
    {/if}
<div>

    <button on:click={searchBucket}>Search bucket</button>
    <button on:click={listMachines}>Refresh Machines List</button>
    <button on:click={getStatus}>Get Status</button>
</div>

</div>
<div>
    {#each machines as machine}
    <div>
        <p>ID: {machine.id}</p>
        <p>Machine: {machine.name}</p>
        <p>State: {machine.state}</p>
    </div>
    <button disabled={true}>Start</button>
    <button disabled={!isRunning}>Stop</button>
    <button 
        on:click={()=>updateDefaultMachine(machine.id === configuration.default_machine ? "resetDefaultMachine":machine.id)}
        >{machine.id === configuration.default_machine ? "Remove" :"Set" } as default</button>
    {/each}
</div>