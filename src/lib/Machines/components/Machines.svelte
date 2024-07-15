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
    import type { Machine, Configuration } from "../../../global_types";
    import {onMount} from 'svelte';
    import MachineAPI from "$lib/api/MachineAPI";
    import {isRunning} from "../utils"

    $: runningState = {} as {[key:string]:boolean};
    $: machines = [] as Machine[];
    $: configuration={default_machine:null} as Configuration;

    async function listMachines() {
        try{
            machines = await MachineAPI.listMachines()
        }catch(err){
            console.error("[Machines] Failed to list machines due to: ", err)
        }
        
    }
    async function handleGetMachine(val:string){
        console.log(`GET machine information for machine '${val}'`)
        try{

            let result = await MachineAPI.getMachineByMachineId(val)
            let updatedMachine = machines.find(machine=>machine.id === val)
            if (updatedMachine){
                let prevMachines = machines.filter(machine=>machine.id !==val)
                machines = [...prevMachines, result]
            }
        }catch(err){
            console.error(`[Machines] Unable to get machine data for '${val}' `)
        }
    }

    /**Send command to backend to start machine . Starts a poll
     * To check the status of the machine until it is ready
    */
    async function handleStartMachine(machineId:string){
        try{

            await MachineAPI.startMachine(machineId)
        }catch(err){
            console.log("[Machine] Unable to start machine due to: ", err)
        }
    }

    onMount(async ()=>{
        await listMachines()
    })
</script>

<div>
    <p>Default Machine: {configuration.default_machine}</p>
    {#if configuration.default_machine}
    <div id="indicator" class={`${runningState[configuration.default_machine] ? "green":"red"}`}>
    </div>
    {/if}
<div>
    <button on:click={listMachines}>Refresh Machines List</button>
</div>

</div>
<h1>Machines</h1>
<div>
    {#each machines as machine, i}
    <div>
        <h3>Machine # {i+1}</h3>
        <button on:click={()=>handleGetMachine(machine.id)}>Get State</button>
        <p>ID: {machine.id}</p>
        <p>Machine: {machine.name}</p>
        <p>State: {machine.state}</p>
    </div>
    <button disabled={isRunning(machine.id, machines)} on:click={()=>handleStartMachine(machine.id)}>Start</button>
    <button disabled={!isRunning(machine.id, machines)}>Stop</button>
    <button 
        on:click={()=>MachineAPI.updateDefaultMachine(machine.id, configuration)}
        >{machine.id === configuration.default_machine ? "Remove" :"Set" } as default</button>
    {/each}
</div>