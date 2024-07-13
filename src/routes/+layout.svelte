<style>
    #page{
        display: flex;
    }
    #side_nav{
        width: 25%;
    }

    #side_nav a{
        display: block;
    }
    #content{
        width:75%;
    }
    
</style>
<script lang="ts">
    import { setContext } from "svelte";
    import {writable} from "svelte/store";

    /**@type {import('./$types').LayoutData}*/
    export let data;

    const projects = writable();
    $: projects.set(data.projects)

    setContext("projects", projects)
</script>

<div id="page">
    <div id="side_nav">

        <a href="/">Machines</a>
        <a href="/projects">Projects</a>
    </div>
    <div id="content">
        <slot></slot>
    </div>
</div>