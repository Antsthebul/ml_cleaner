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
    .classBox{
        height: 200px;
        overflow: hidden;
        border: 1px solid black;
        padding:0 5px 5px;
        border-radius:5px;
        margin-top:5px;
        overflow-y: scroll;
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
    import {projects} from "../../../store"
    import type {ResponseType, SimpleSuccessResponse, Project, Machine, ProjectMachine} from "../../../global_types";


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
    
    $: searchText = ''
    
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
    $: searchableClasses = isTextInClassList(searchText);

    // Used for saerch display purposes
    $: listOfClasses = localProject.class_data.classes

    $: fileLoadResponse = ""

    $: isTextInClassList = (searchText:string):string[]=>{
        if (!searchText || listOfClasses.length == 0) {
            return listOfClasses
        }
        let res = listOfClasses.filter((className:string)=> className.toLowerCase().startsWith(searchText)) ??  []
        return res
    }
    $: showMachineListDropDown = false;
    let machines = [] as Machine[]
    let selectedMachineIdx = 0;
    
    const updateSelectedMachine = () =>{
        if (selectedMachineIdx > 0 ){
            let server_machine = machines[selectedMachineIdx-1]
            let machine: ProjectMachine = {
                id:server_machine.id,
                name:server_machine.name,
                machine_type:server_machine.machineType
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

    async function deleteProject(){
        let data:string = await invoke("delete_project_by_name", {name:slug})
        let result:SimpleSuccessResponse = JSON.parse(data)
        if (result.data){
            projects.update(projects=>projects.filter(proj=>proj.name !== slug))
            goto("/projects")
        }else{
            console.error("Unable to delete project due to ", result)
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
        console.log("type")
        localProject.class_data.file_exists = true
        fileNameInput = e.target.value
    }
    onMount(async ()=>{
        await loadProjectByName(slug)
    })

</script>

<section>
    <div id="titleSection">
        <h1>{slug}</h1>
        <button on:click={deleteProject} class="button">Delete</button>
    </div>
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
                <button class="display-block">Train</button>
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
        <div>
            <input bind:value={searchText} placeholder="Search for an existing class"/>
            <div class="classBox">
                
                {#each searchableClasses as className, ix}
                <p>{ix+1}). {className}</p>
                {/each}
            </div>
        </div>
    </div>
</section>