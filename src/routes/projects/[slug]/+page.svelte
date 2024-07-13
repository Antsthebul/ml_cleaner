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
    .bold{
        font-weight: 700;
    }
    .classBox{
        height: 200px;
        overflow: hidden;
        border: 1px solid black;
        padding:5px;
    }
</style>

<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
    import { page } from "$app/stores";
    import {onMount} from "svelte"
    import {goto} from "$app/navigation"
    import {loadProjects, projects} from "../../../store"
    import type {SimpleSuccessResponse} from "../../../global_types";

    export let data
    let slug = $page.params.slug
    
    interface Project {
        classKey:string|null
    }
    
    const initlocalProject:Project = {
        classKey:null
    }


    $: listOfClasses = data.data.classes ?? []

    $: searchText = ''

    $: searchableClasses = isTextInClassList(searchText);

    $: allowEditClassesPath = false;
    $: localProject = initlocalProject;

    $: fileLoadResponse = ""

    function isTextInClassList(searchText:string):string[]{
        if (!searchText) return listOfClasses
        if (searchText && listOfClasses){
            return listOfClasses.filter((className:string)=> className.toLowerCase().startsWith(searchText))
        }
        return []
    }

    function setAllowEditClassesPath(val:boolean){
        allowEditClassesPath = val
    }
    async function onSaveUpdatedConfig(){

        if (localProject.classKey){

            localProject.classKey = localProject.classKey.trim()
        }
        
        let result:string = await invoke("update_configuration_file_command", {file:JSON.stringify(localProject)})

        fileLoadResponse = Object.entries(result)[0][1]
        setAllowEditClassesPath(false)
    }
    async function loadProjectByName(val:string){
        await invoke("get_config_by_project_name", {name:val})
    }
    async function deleteProject(){
        let data:string = await invoke("delete_project_by_name", {name:slug})
        let result:SimpleSuccessResponse = JSON.parse(data)
        console.log("ugh ", result.data, typeof result, Object.keys(result))
        if (result.data){
            projects.update(projects=>projects.filter(proj=>proj.name !== slug))
            goto("/projects")
        }else{
            console.error("Unable to delete project due to ", result)
        }
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
        <span>Classes Key:</span>
        
        {#if allowEditClassesPath}
            <input bind:value={localProject.classKey}/>
            <button on:click={onSaveUpdatedConfig}>Save</button>
            <button on:click={()=>setAllowEditClassesPath(false)}>Cancel</button>
        {:else}
        <p>{localProject.classKey ? localProject.classKey :""}</p>
        <button on:click={()=>setAllowEditClassesPath(true)}>Edit</button>
        {/if}    
    </div>
    <div id="main">

        <div>
            <p>Last Modified: {new Date(data.data.lastModified).toLocaleString()}</p>
            <p><span class="bold">Total Trained Classes: </span> {listOfClasses.length}</p>
        </div>
        <div>
            <input bind:value={searchText} />
            <div class="classBox">
                
                {#each searchableClasses as className, ix}
                <p>{ix+1}).{className}</p>
                {/each}
            </div>
        </div>
    </div>
</section>