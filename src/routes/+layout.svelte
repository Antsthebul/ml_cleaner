<style lang="scss">
    #page{
        display: flex;
        height: 100vh;
    }
    

</style>
<script lang="ts">
    import '../app.css';
    import { SideNav } from '$lib';
    import {onMount} from 'svelte'
    import {loadProjects} from '../store'
    import ConfigAPI from '$lib/api/ConfigAPI';

    onMount( async ()=>{
        try{
            await ConfigAPI.getConfig()
            await loadProjects()
        }catch(err){
            console.error(`[Home] Failed to load projects due to the following '${err}'`)
        }
    })

</script>

<div id="page">
    <SideNav/>
    <div id="app" class="w-100">
        <slot></slot>
    </div>
</div>