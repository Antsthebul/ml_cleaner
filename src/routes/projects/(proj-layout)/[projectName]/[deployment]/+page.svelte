<!-- Deployment Detail Page  -->

<style>
    .bold{
        font-weight: 700;
    }
</style>

<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
    import type { Machine, ProjectMachine, Deployment, TrainingData, TrainingDataDataSeries} from "$lib/global_types";
	import { ProjectMachineSideDrawer } from "$lib";
    import { DependVarWindow } from "$lib"; 

	export let data

    type TableHeader = "epoch"|"train_acc"|"test_acc"|"train_loss"|"test_loss"
    interface TableSortOption{
        column: TableHeader,
        asc: boolean
    }

	const INIT_DEPLOYMENT = {
		name:'',
		files:null,
		machines:[],
	}

	let curDeployment:Deployment|null = data.data ?? {...INIT_DEPLOYMENT}
    let trainingResults: TrainingData[] = []
    let fileNameInput = ''
    
    let showMachineList = false;
    let machines = [] as Machine[]
    let selectedMachineIdx = 0;
    
    $: tableSortOption  = {} as TableSortOption
    $: allowEditClassesPath = false;
    // Used for saerch display purposes
    $: listOfClasses = []

    $: fileLoadResponse = ""

    $: showMachineText = ():string =>{
        let text = showMachineList ? "Hide": "Show"
        return text + " machines"
    }



    


    const updateSelectedMachine = () =>{
        if (selectedMachineIdx > 0 ){
            let server_machine = machines[selectedMachineIdx-1]
            let machine: ProjectMachine = {
                id:server_machine.id,
                name:server_machine.name,
                machine_type:server_machine.machineType,
                ip_address:server_machine.ip_address
            }
            
            // TODO: 'actual' machine
            // curDeployment.machine = machine
        };
    };


    async function listMachines(){
        try{

            let data:string = await invoke("list_machines")
            let result = JSON.parse(data)
            if (result.data){
                machines = result.data
            }else{
                console.error("Failed to list machines due to: ", result.error)
            }
        }catch(err){
            console.error("Failed to list machines due to: ", err)

        }
    }

    function setAllowEditClassesPath(val:boolean){
        allowEditClassesPath = val
    }

    async function handleSaveUpdateEnv(){

        console.log(`Updating env for  '${curDeployment.name}'`, curDeployment, fileNameInput)
        if (fileNameInput){

            curDeployment.classes_file = fileNameInput.trim()
        }
		// Need to update this
        // let result:string = await invoke("update_configuration_file_command", {file:JSON.stringify({default_machine:null, projects:{[localProject.project.name]: project}})})
        // console.log("oh i see", result)
        // fileLoadResponse = Object.entries(result)[0][1]
        setAllowEditClassesPath(false)
    }


    async function handleAddMachine(){
        await listMachines()
        showMachineListDropDown = true
    }

    async function handleSaveMachine(){
        showMachineListDropDown = false
        updateSelectedMachine()
        await handleSaveUpdateEnv()
    }

    function handleFileNameInput(e:any){
        // localProject.class_data.file_exists = true
        fileNameInput = e.target.value
    }
    async function trainModel(){
		// TODO: Should not be by proejct name, specificy by URN
        let response = await invoke("train_model", {projectName:curDeployment.name, })
        console.log("train cool ", response)
    }


</script>

<section>
    <div>
        <span>{fileLoadResponse}</span>
        <!-- {#if !data.data.classes_data.file_exists}
        <p class="errorMessage">File does not exist</p>
        {/if} -->
        <!-- <span class="display-block mb-5"><b>Machine: </b> {#if !curDeployment.machine}No machine added at this time{/if}</span> -->
        <div class="text-center">
            <button class="button-less fake-link cursor" on:click={()=> showMachineList = !showMachineList}>{showMachineText()}</button>
            <!-- {#if !showMachineListDropDown}
            <button on:click={handleAddMachine}>Add a Machine</button>
            {:else}
            <button on:click={handleSaveMachine}>Save</button>
            <select bind:value={selectedMachineIdx}>
                <option value={0}></option>
                {#each machines as machine, i}
                <option value={i+1}>{machine.name}</option>
                {/each}
            </select>
            {/if} -->
        </div>
        <div id="machines_container mb-10">
            <!--Shwo only name and state-->
            <div>
                <!-- <button class="display-block mt-5" disabled={!curDeployment.machine}>Start</button> -->
                <!-- {#if curDeployment.machines}
                <button class="display-block">Stop</button>
                <button class="display-block" on:click={trainModel} disabled={!curDeployment.machine.ip_addr}>Train</button>
                {/if} -->
            </div>
            <!-- <div>
                <span class="display-block">State</span>
                <span class="display-block">
                    {localProject.project.machine?.state ??""}
                </span>    
            </div> -->
        </div>
        <!-- <span class="display-block mt-10 mb-5"><b>Classes Key/File: </b>{curDeployment.classes_file}</span> -->
        <span class="display-block mb-5"><b>File(s): </b></span>  
    </div>
    <div>

        <div class="w-100">
            <small>Last Modified: Today</small>
            <p><span class="bold">Total Trained Classes: </span> {listOfClasses?.length}</p>
        </div>
    </div>
    
    <ProjectMachineSideDrawer 
        bind:showSideDrawer={showMachineList} 
        bind:trainingResults={trainingResults}
        machines={curDeployment.machines}
        />
    
    <DependVarWindow listOfClasses={[]}/>
    
</section>