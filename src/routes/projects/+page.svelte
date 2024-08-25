<!--List route-->
<style>
#project_section{
    margin-top: 150px;
    display: flex;
    flex-wrap: wrap;
    gap:5px;


    & .projectLink{
        width: 24%;


    }
}


#pageHeader{
    display:flex;
    justify-content: space-between;
}

#buttonWrapper{
    align-content: center;
}

.button{
    padding:5px;
    background-color: lightgrey;
    border: 1px solid black;
    border-radius:5px
}

.projectLink{
    display: block;
}

</style>
<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
    import {projects, loadProjects} from "../../store"
    import { onMount } from "svelte";

    let path:string|null = null;

    function checkIfPathExists(path:string){
        invoke("check_if_path_exists",{ path})
    }

    onMount(async ()=>{
        await loadProjects()
    })
</script>

<div>
    <div id="pageHeader">

        <h1>Repository</h1>
        <div id="buttonWrapper">
            <a class="button noLink" href="/create-project">Create Project</a>
        </div>
    </div>
    
    <span>Base Folder</span>
    <input bind:value={path}>
    <button>Create</button>
</div>
<section id="project_section">
    {#each $projects as project}
    <a href={`/projects/${project.name}`} class="projectLink">
        <p>{project.name}</p>
    </a>
    {/each}
</section>