<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import {goto} from "$app/navigation";
    import type {ResponseType } from "../../global_types";
    import {loadProjects} from "../../store";

    interface NewProject {
        name:string|null,
        classes_file: string|null
    }

    const initNewProject: NewProject = {
        name:null,
        classes_file:null
    }
    $: newProject = initNewProject
    $: saveButtonIsDisabled= ()=>{
        return !Object.values(newProject).every(el=>el)
    }

    const handleUpdateNewProject = ()=>{
        newProject.name = newProject.name?.trim() ??  null
        newProject.classes_file = newProject.classes_file?.trim() ??  null
    }
    async function saveNewProject(){
        let result:string = await invoke("create_new_project", {project:JSON.stringify(newProject)})
        let response:ResponseType<NewProject> = JSON.parse(result)
        if (response.data){ 
            await loadProjects()
            goto(`/projects/${newProject.name}`)}
        response.error && console.log("horriable fuckboui dev")

    }
</script>

<section>
    <div>

        <span>Project Name</span>
        <input bind:value={newProject.name} on:change={handleUpdateNewProject}/>
    </div>
    <div>
        <span>Project classes file</span>
        <input bind:value={newProject.classes_file} on:input={handleUpdateNewProject}>
    </div>

    <button on:click={saveNewProject} disabled={saveButtonIsDisabled()}>Create</button>

</section>