<!--List route-->
<style>
    #page{
        padding:0 5px;
    }
        
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

    import { projects } from "$lib/store"


</script>

<div id="page">
    <div id="pageHeader">
        <h1>Projects</h1>
        <div id="buttonWrapper">
            <a class="button noLink" href="/create-project">Create Project</a>
        </div>
    </div>

</div>
<section id="project_section">
    {#each $projects as project}
    <a href={`/projects/${project.name}`} class="projectLink">
        <p>{project.name}</p>
    </a>
    {/each}
</section>