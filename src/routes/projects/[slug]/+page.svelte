<style>
    #main{
        display: flex;
    }

    #main *{
        flex:1
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
	import type { ChangeEventHandler } from 'svelte/elements';
	import { invoke } from "@tauri-apps/api/tauri";
    import { page } from "$app/stores";
    import {onMount} from "svelte"

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
        console.log("oh ", result)
        fileLoadResponse = Object.entries(result)[0][1]
        setAllowEditClassesPath(false)
    }
    async function loadProjectByName(val:string){
        await invoke("get_config_by_project_name", {name:val})
    }
    onMount(async ()=>{
        await loadProjectByName(slug)
    })

</script>

<section>
    <h1>{slug}</h1>
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