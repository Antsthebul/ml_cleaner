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
</div>