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
    .loading{
        position:fixed;
        top:0;
        width: 100%;
        background-color:rgba(255, 255, 255, 0.566);
    }
    .inner-loader{
        width: 100%;
        z-index: 9999;
        height:100vh;
        font-size: 4em;
        margin: 0 auto;
        color:rgba(6, 115, 205, 0.884);
        display:flex;
        justify-content: center;
        align-items: center;
    }
</style>
<script lang="ts">
	import { goto } from "$app/navigation";
    import LineMdLoadingLoop from '~icons/line-md/loading-loop';
    
    export let listOfClasses:string[] = []
    let loading = false

    const DEPENDENT_VARIABLE_OPTIONS = {
        CLASSES:"CLASSES",
        LABELS:"LABELS"
    } as const

    type DependentVariableOptions = typeof DEPENDENT_VARIABLE_OPTIONS[keyof typeof DEPENDENT_VARIABLE_OPTIONS]
    let showDependentVariablesAs:DependentVariableOptions = DEPENDENT_VARIABLE_OPTIONS.CLASSES
    let searchText = ''
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
<div class="loading">
    <div class="inner-loader">

        <LineMdLoadingLoop />
    </div>
</div>
{/if}

<div>
    <button disabled={showDependentVariablesAs === DEPENDENT_VARIABLE_OPTIONS.CLASSES}>As Classes</button>
    <button on:click={()=>showDependentVariablesAs = DEPENDENT_VARIABLE_OPTIONS.LABELS} disabled={showDependentVariablesAs === DEPENDENT_VARIABLE_OPTIONS.LABELS}>As Labels</button>
    <input bind:value={searchText} placeholder="Search for an existing class"/>
    <div class="classBox">
        
        {#each searchableClasses as className, ix}
        <button class="cursor depVar button-less display-block"
            on:click={()=>{loading = true; goto(`/data/${className}`)}}
            >{ix+1}). {className}</button>
        {/each}
    </div>
</div>
