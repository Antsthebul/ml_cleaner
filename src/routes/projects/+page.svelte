<style>
#project_section{
    margin-top: 150px;
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
    <h1>Repository</h1>

    <span>Base Folder</span>
    <input bind:value={path}>
    <button>Create</button>
</div>
<section id="project_section">
    <a href="/projects/1">
        <p>Test Project</p>
    </a>
</section>