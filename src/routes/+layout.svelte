<style lang="scss">
    #page{
        display: flex;
    }

    
    #content{
        width:75%;
    }

</style>
<script lang="ts">
    import '../app.css'
    import { getContext } from "svelte";
    import { invoke } from '@tauri-apps/api/tauri';
    import type {  Project } from '../global_types';
    import Nav from '../components/Nav.svelte';

    $: projectList = getContext("projects") ?? [] as Project[]

    console.log("oh ", getContext("projects"))
</script>

<div id="page">
    <Nav/>
    <div id="content">
        <slot></slot>
    </div>
</div>