<style>
    .classBox{
        height: 200px;
        overflow: hidden;
        border: 1px solid black;
        padding:0 5px 5px;
        border-radius:5px;
        margin-top:5px;
        overflow-y: scroll;
    }
    
    .depVar{
        &:hover{
            background-color: grey;
        }
    }

</style>
<script lang="ts">
	import { goto } from "$app/navigation";
    import { page } from "$app/stores";
	import {Loader }from "$lib";
    
    export let listOfClasses:string[] = []
    
    let loading = false
    
    const DEPENDENT_VARIABLE_OPTIONS = {
        CLASSES:"CLASSES",
        LABELS:"LABELS"
    } as const

    type DependentVariableOptions = typeof DEPENDENT_VARIABLE_OPTIONS[keyof typeof DEPENDENT_VARIABLE_OPTIONS]
    let showDependentVariablesAs:DependentVariableOptions = DEPENDENT_VARIABLE_OPTIONS.CLASSES
    let searchText = ''
    let projectName = $page.params.projectName
    $:listOfClasses
    let searchableClasses = [...listOfClasses]

    $: isTextInClassList(searchText)

    const isTextInClassList = (searchText:string)=>{

        if (!searchText) {
            searchableClasses = listOfClasses
        }
        let res = listOfClasses.filter((className:string)=> className.toLowerCase().startsWith(searchText)) ??  []

        searchableClasses = [...res]
    }    

</script>
{#if loading}
<Loader />
{/if}

<div>
    <button disabled={showDependentVariablesAs === DEPENDENT_VARIABLE_OPTIONS.CLASSES}>As Classes</button>
    <button on:click={()=>showDependentVariablesAs = DEPENDENT_VARIABLE_OPTIONS.LABELS} disabled={showDependentVariablesAs === DEPENDENT_VARIABLE_OPTIONS.LABELS}>As Labels</button>
    <input bind:value={searchText} placeholder="Search for an existing class"/>
    <div class="classBox">
        
        {#each searchableClasses as className, ix}
        <button class="cursor depVar button-less display-block"
            on:click={()=>{loading = true; goto(`/data/${className}?project=${projectName}`)}}
            >{ix+1}). {className}</button>
        {/each}
    </div>
</div>
