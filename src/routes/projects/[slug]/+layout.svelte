<script lang="ts">
    import { page } from "$app/stores";
	import type { SimpleSuccessResponse } from "$lib/global_types";
	import { invoke } from "@tauri-apps/api/tauri";
    import {projects} from "$lib/store"
    import {goto} from "$app/navigation"
    
    export let data

    const INIT_PROJECT = {envs:[]}

    let slug = $page.params.slug

    let curProject = data.data ?? INIT_PROJECT

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
    
</script>

<div id="titleSection">
    <h1>{slug}</h1>
    <button on:click={deleteProject} class="button">Delete</button>
    <a href={`/projects/${slug}`}>Project Home</a>
    <h3>Environments</h3>
    <div class="display-flex gap-10">

        {#each curProject.envs as env}
        <a href={`/projects/${slug}/${env.name}`}>{env.name}</a>
        {/each}
    </div>
    <slot></slot>
</div>