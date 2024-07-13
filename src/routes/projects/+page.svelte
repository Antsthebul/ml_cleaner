<style>
#project_section{
    margin-top: 150px;
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



</style>
<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";


    let path:string|null = null;

    function checkIfPathExists(path:string){
        invoke("check_if_path_exists",{ path})
    }
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
    <a href="/projects/1">
        <p>Test Project</p>
    </a>
</section>