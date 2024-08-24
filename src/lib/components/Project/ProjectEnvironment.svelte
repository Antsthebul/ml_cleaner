<style>
    #main{
        display: flex;
    }

    #main *{
        flex:1
    }

    #titleSection{
        display: flex;
        justify-content: space-between;
    }
    #machine_section{
        display: flex;
        justify-content: space-between;
        height: 75px;
        margin-top:10px;

        & *{
            flex:1;
        }
    }
    .bold{
        font-weight: 700;
    }

    .errorMessage{
        color:red;
        margin-bottom: 0;
        font-style: italic;
        font-weight: 500;
        font-size: .8em;
    }
</style>

<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
    import { page } from "$app/stores";
    import {onMount} from "svelte"
    import {goto} from "$app/navigation"
    import {projects} from "$lib/store"
    import type {ResponseType, SimpleSuccessResponse, Project, Machine, ProjectMachine} from "$lib/global_types";
	import DependVarWindow from "$lib/components/Project/DependVarWindow.svelte";


    interface ClassData {
        file_exists: boolean,
        classes: string[],
        lastModified:string
    }
    interface ProjectDetailResponse{
        project:Project,
        class_data: ClassData
       
    }

    let slug = $page.params.slug
        
    $: allowEditClassesPath = false;
    $: localProject = {
        project:{
            name:"",
            classes_file:"",
            machine:null
        },
        class_data:{
            file_exists:true,
            classes:[],
            lastModified:""
        },
    } as ProjectDetailResponse;
    let fileNameInput = localProject?.project.classes_file
    // Used for saerch display purposes
    $: listOfClasses = localProject.class_data.classes

    $: fileLoadResponse = ""


    $: showMachineListDropDown = false;
    let machines = [] as Machine[]
    let selectedMachineIdx = 0;
    
    const updateSelectedMachine = () =>{
        if (selectedMachineIdx > 0 ){
            let server_machine = machines[selectedMachineIdx-1]
            let machine: ProjectMachine = {
                id:server_machine.id,
                name:server_machine.name,
                machine_type:server_machine.machineType,
                ip_addr:server_machine.ip_addr
            }
            
            localProject.project.machine = machine
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
    async function onSaveUpdatedConfig(){

        console.log("Updating config ", localProject, fileNameInput)
        if (fileNameInput){

            localProject.project.classes_file = fileNameInput.trim()
        }
        let result:string = await invoke("update_configuration_file_command", {file:JSON.stringify({default_machine:null, projects:{[localProject.project.name]: localProject.project}})})
        console.log("oh i see", result)
        fileLoadResponse = Object.entries(result)[0][1]
        setAllowEditClassesPath(false)
    }

    async function loadProjectByName(val:string){
        let data:string = await invoke("get_project_by_project_name", {name:val})
        let result:ResponseType<ProjectDetailResponse> = JSON.parse(data)
        if (result.data){
            localProject = result.data
        }else{
            console.error("Unable to local project configuration due to: ", result.error)
        }
    }



    async function handleAddMachine(){
        await listMachines()
        showMachineListDropDown = true
    }

    async function handleSaveMachine(){
        showMachineListDropDown = false
        updateSelectedMachine()
        await onSaveUpdatedConfig()
    }

    function handleFileNameInput(e:any){
        localProject.class_data.file_exists = true
        fileNameInput = e.target.value
    }
    async function trainModel(){
        let response = await invoke("train_model", {projectName:localProject.project.name, })
        console.log("train cool ", response)
    }
    onMount(async ()=>{
        await loadProjectByName(slug)
    })

</script>

<section>

    <div>
        <span>{fileLoadResponse}</span>
        {#if !localProject.class_data.file_exists}
        <p class="errorMessage">File does not exist</p>
        {/if}
        <span class="display-block mb-5"><b>Machine: </b> {#if !localProject.project.machine}No machine added at this time{/if}</span>
        {#if !showMachineListDropDown}
        <button on:click={handleAddMachine}>Add a Machine</button>
        {:else}
        <button on:click={handleSaveMachine}>Save</button>
        <select bind:value={selectedMachineIdx}>
            <option value={0}></option>
            {#each machines as machine, i}
            <option value={i+1}>{machine.name}</option>
            {/each}
        </select>
        {/if}
        <div id="machine_section">
            <div>
                <button class="display-block mt-5" disabled={!localProject.project.machine}>Start</button>
                {#if localProject.project.machine}
                <button class="display-block">Stop</button>
                <button class="display-block" on:click={trainModel} disabled={!localProject.project.machine.ip_addr}>Train</button>
                {/if}
            </div>
            
            <div>
                <span class="display-block">ID</span>
                <span class="display-block">

                    {localProject.project.machine?.id ??""}
                </span>
            </div>
            <div>
                <span class="display-block">Name</span>
                {#if localProject.project.machine?.name }
                <a href={`/machines/${localProject.project.machine.name}`} class="display-block">
                    {localProject.project.machine.name}
                </a>
                {/if}    
            </div>
            <!-- <div>
                <span class="display-block">State</span>
                <span class="display-block">
                    {localProject.project.machine?.state ??""}
                </span>    
            </div> -->
        </div>
        <span class="display-block mb-5"><b>Classes Key/File: </b>{localProject.project.classes_file}</span>
        <span class="display-block mb-5"><b>Info file: </b></span>
        
        {#if allowEditClassesPath}
            <div>

                <input bind:value={fileNameInput} on:input={handleFileNameInput}/>
                <button on:click={onSaveUpdatedConfig}>Save</button>
                <button on:click={()=>setAllowEditClassesPath(false)}>Cancel</button>
            </div>
        {:else}

            <button on:click={()=>setAllowEditClassesPath(true)} class="display-block">Edit</button>
        {/if}    
    </div>
    <div id="main">

        <div>
            <p>Last Modified: {localProject.class_data.lastModified ? new Date(localProject.class_data.lastModified).toLocaleString():""}</p>
            <p><span class="bold">Total Trained Classes: </span> {listOfClasses.length}</p>
        </div>
        <DependVarWindow listOfClasses={listOfClasses}/>
    </div>
</section>