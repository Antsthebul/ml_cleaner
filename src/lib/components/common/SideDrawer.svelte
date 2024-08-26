<style>
    #side-bar{
        height: 100%;
        position: fixed;
        top:0;
        z-index: 999;
        background-color: white;
        padding:5px 8px;
    }
</style>
<script lang="ts">
    import {slide} from "svelte/transition"

    export let showSideDrawer:boolean=false
    export let width:number|string = "25%"
    export let side: "left"|"right" = "right"

    let cssStyles = `width:${width};${side}:0;` 

    // Apply shadow
    if(side === "left"){
        cssStyles += "box-shadow: 1px 1px 8px lightgrey"
    }else{
        cssStyles += "box-shadow: -1px 1px 8px lightgrey"

    }
</script>

{#if showSideDrawer}
<div in:slide={{delay:50, duration:300}} out:slide={{delay:0, duration:100}}>

    <section id="side-bar" style={`${cssStyles}`}>
        <slot></slot>
    </section>
</div>
{/if}