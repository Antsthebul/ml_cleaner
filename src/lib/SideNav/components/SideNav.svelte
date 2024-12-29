<style lang="scss">
     #min_nav{
        z-index: 98;
        position: relative;
        text-align: center;
        background-color: white;
        box-shadow: 4px 0px 5px grey;

     }
     #side_nav{
        z-index: 99;
        text-align: center;

        background-color: rgb(255, 212, 133);
    }
    
    #side_nav a{
        display: flex;
        margin-top: 5px;
        padding:0 10px;
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
    import { slide } from 'svelte/transition' 
	import { tweened } from 'svelte/motion';

    import {projects} from "$lib/store"
    import { FadeInText } from '$lib';
    import TerminalIcon from "~icons/ph/computer-tower-light"
    import ProjectIcon from "~icons/ix/project"
    import PanelLeftIcon from "~icons/lucide/panel-left-open"
    import PanelRightIcon from "~icons/lucide/panel-right-open"
    import GrommetIcon from "~icons/grommet-icons/home-rounded"
    import CaretRightBold from '~icons/ph/caret-right-bold';
	import { cubicInOut } from 'svelte/easing';

    type SelectionArrowOption = "MACHINES"|"PROJECTS"|null

    const OPEN = 20   
    const CLOSED = 5
    const MINI_NAV_OPEN = 50
    const MINI_NAV_CLOSED = -30
    
    let selectionArrowOption:SelectionArrowOption = null
    let miniNavIsEntered=true
    $: resetSelectionArrowOption = () =>{
        selectionArrowOption = null
    } 

    $: width = tweened(OPEN, {
        duration: 100,
        easing: cubicInOut
    })
    $: minNavWidth = tweened(MINI_NAV_CLOSED, {
        duration:100,
        easing: cubicInOut
    })
    
    $: isPanelOpen = (): boolean =>{
        return $width === OPEN
    }


    $: handleOpenPanel = () =>{
        if ($width == OPEN){
            width.set(CLOSED)
        }else{
            width.set(OPEN)
        }
    }
    $: {
        if (selectionArrowOption){
            minNavWidth.set(MINI_NAV_OPEN)
        }else{
            minNavWidth.set(MINI_NAV_CLOSED)
        }
    }
</script>

<div id="side_nav"
    style ={ `width:${$width}%; max-width:600px;`}
    >
    <div class="display-inline">
        <button 
            class="button button-less display-flex text-white justify-self-end"
            on:click={handleOpenPanel}
            >
            {#if isPanelOpen()}
                <PanelRightIcon />
            {:else}
                <PanelLeftIcon />
            {/if}
        </button>
    </div>
    <a href="/"><GrommetIcon /> <FadeInText visible={isPanelOpen()} text="&nbsp;Home" /></a>
    <a href="/machines"
        on:mouseenter={()=>selectionArrowOption = "MACHINES"}
        on:mouseout={resetSelectionArrowOption}
        on:blur={resetSelectionArrowOption}
        on:focus={()=>selectionArrowOption = "MACHINES"}
        > <TerminalIcon /> 
        <FadeInText visible={isPanelOpen() } text="&nbsp;Machines"/>
        {#if selectionArrowOption === "MACHINES"}
        <CaretRightBold />
    {/if}
    </a>

    <a href="/projects"
        on:mouseenter={()=>selectionArrowOption = "PROJECTS"}
        on:mouseout={resetSelectionArrowOption}
        on:blur={resetSelectionArrowOption}
        on:focus={()=>selectionArrowOption = "PROJECTS"}
        > 
        <ProjectIcon /> 
        <FadeInText visible={isPanelOpen()} text="&nbsp;Projects" />
        {#if selectionArrowOption === "PROJECTS"}
            <CaretRightBold />
        {/if}
    </a>

    <!-- <a href="/editConfig">Edit Config</a> -->
</div>
{#if selectionArrowOption === "PROJECTS"}
<!--svelte-ignore a11y-no-static-element-interactions-->
<div
    id="min_nav"
    on:mouseenter={()=>miniNavIsEntered = true}
    on:focus={()=>miniNavIsEntered = true}

    style={`transform: translateX(${$minNavWidth}px); width:${$minNavWidth === MINI_NAV_CLOSED ? "0":"20"}%;`}>
    {#each $projects as project}
        <a data-sveltekit-preload-data="tap" href={`/projects/${project.name}`} class="noLink">&nbsp;&nbsp;{project.name}</a>
    {/each}
</div>
{/if}