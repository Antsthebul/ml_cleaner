<style lang="scss">
     #side_nav{
        width: 25%;
    }

    #side_nav a{
        display: block;
        margin-top: 5px;

        &:hover{
            background-color: rgb(214, 215, 215);
        }
    }   
</style>
<script lang="ts">
    import type {  Project } from '../global_types';
    import {projects} from "../store"


</script>
<div id="side_nav">

    <a href="/">Home</a>
    <a href="/machines">Machines</a>
    <a href="/projects">Projects</a>
    {#each $projects as project}
        <a href={`/projects/${project.name}`} class="noLink">&nbsp;{project.name}</a>
    {/each}
</div>