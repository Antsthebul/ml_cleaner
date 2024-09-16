<script lang="ts">

    import { page } from "$app/stores";
	import type { SimpleSuccessResponse } from "$lib/global_types";
	import { invoke } from "@tauri-apps/api/tauri";
    import {projects} from "$lib/store"
    import {goto} from "$app/navigation"

    import CaretRightBold from '~icons/ph/caret-right-bold';
    
    export let data

    const INIT_PROJECT = {envs:[]}

    let slug = $page.params.slug

    let curProject = data.project ?? INIT_PROJECT
    let selectedDeployment = ''

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

    async function handleNavigateDeployment(name:string){
        let baseLink = `/projects/${slug}`;
        if (name !=="home"){
            selectedDeployment = name
        }else{
            selectedDeployment = ''
        }
        
        let link = name === "home"? baseLink :`${baseLink}/${name}`
        await goto(link)
    }
</script>

<div id="titleSection">
    <div class="display-flex justify-content-between">

        <h1>{slug}</h1>
        <div>
            <button on:click={deleteProject} class="button">Delete</button>
        </div>

    </div>
    <button class="fake-link button-less cursor"
        on:click={async ()=>handleNavigateDeployment("home")}>Project Home</button>
    
    <!--Fake breadcrumbs-->
    {#if selectedDeployment}
        <span class="display-i-flex fit-content f-5 fake-link">
            <CaretRightBold />
        </span>
        <span class="fake-link">  {selectedDeployment}</span>
    {/if}
    <h3>Environments</h3>
    <div class="display-flex gap-10">

        {#each curProject.deployments as dep}
            <button 
            class={`button-link ${selectedDeployment === dep.name ? "button-link-disabled":''} cursor`}
                on:click={async ()=>handleNavigateDeployment(dep.name)}
            disabled={selectedDeployment === dep.name}
            >{dep.name}</button>
        {/each}
    </div>


    <slot></slot>
</div>