<style lang="scss">
     #side_nav{
        width: 15%;
        text-align: center;
        margin-right:10px;
        background-color: rgb(255, 212, 133);
    }
    
    #side_nav a{
        display: block;
        margin-top: 5px;
        font-weight: 800;
        font-size: 1.2em;
        color:white;
        text-decoration: none;
        margin-bottom: 15px;

        &:hover{
            background-color: rgb(211, 151, 12);
        }
    }   
</style>
<script lang="ts">
    import {projects} from "../../../store"


</script>

<div id="side_nav">

    <a href="/">Home</a>
    <a href="/machines">Machines</a>
    <a href="/projects">Projects</a>
    {#each $projects as project}
        <a data-sveltekit-preload-data="tap" href={`/projects/${project.name}`} class="noLink">&nbsp;&nbsp;{project.name}</a>
    {/each}
    <a href="/editConfig">Edit Config</a>
</div>