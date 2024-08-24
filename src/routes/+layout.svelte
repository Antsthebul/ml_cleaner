<style lang="scss">
    #page{
        display: flex;
    }

    
    #content{
        width:75%;
    }

</style>
<script lang="ts">
    import '../app.css';
    import { Nav } from '$lib';
    import {onMount} from 'svelte'
    import {loadProjects} from '../store'
    import ConfigAPI from '$lib/api/ConfigAPI';

    onMount(async ()=>{
        try{

            let res = await ConfigAPI.getConfig()
            console.log("Got config ", res)
            await loadProjects()
        }catch(err){
            console.error(`[Home] Failed to load projects due to the following '${err}'`)
        }
    })

</script>

<div id="page">
    <Nav/>
    <div id="content">
        <slot></slot>
    </div>
</div>