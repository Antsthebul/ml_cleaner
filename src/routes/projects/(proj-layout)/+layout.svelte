<style>
    #title_section{
        text-align: center;
        justify-content: center;
    }

    #environment_section{
        min-width: fit-content;
    }
</style>
<script lang="ts">

    import { page } from "$app/stores";
	import type { SimpleSuccessResponse } from "$lib/global_types";
	import { invoke } from "@tauri-apps/api/core";
    import { projects } from "$lib/store"
    import { goto } from "$app/navigation"

    import CaretRightBold from '~icons/ph/caret-right-bold';
    
    export let data

    const INIT_PROJECT = {envs:[]}

    let projectName = $page.params.projectName
    
    let curProject = data.project ?? INIT_PROJECT
    
    $: currentDeployment = $page.params.deployment

    async function deleteProject(){
        let data:string = await invoke("delete_project_by_name", {name:projectName})
        let result:SimpleSuccessResponse = JSON.parse(data)
        if (result.data){
            projects.update(projects=>projects.filter(proj=>proj.name !== projectName))
            goto("/projects")
        }else{
            console.error("Unable to delete project due to ", result)
        }
    }

    async function handleNavigateDeployment(name:string){
        let baseLink = `/projects/${projectName}`
        let link = name === "home"? baseLink :`${baseLink}/${name}`
        await goto(link)
    }
</script>

<div id="title_section">
    <div class="abs-right">
        <button on:click={deleteProject} class="button">Delete</button>
    </div>
    <div style="height:30px;">

        <h1>{currentDeployment ?? projectName}</h1>

    </div>
    <div class="text-left">

        <button class="fake-link button-less cursor"
        on:click={async ()=>handleNavigateDeployment("home")}>{projectName}</button>
        
        <!--Fake breadcrumbs-->
        {#if  currentDeployment}
        <span class="display-i-flex fit-content f-5 fake-link">
            <CaretRightBold />
        </span>
        <span class="fake-link">  {currentDeployment}</span>
        {/if}
    </div>
    <slot></slot>
</div>