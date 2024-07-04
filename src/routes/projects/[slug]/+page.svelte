<style>
    #main{
        display: flex;
    }

    #main *{
        flex:1
    }


    .bold{
        font-weight: 700;
    }
    .classBox{
        height: 200px;
        overflow: hidden;
        border: 1px solid black;
        padding:5px;
    }
</style>

<script lang="ts">
	import type { ChangeEventHandler } from 'svelte/elements';

    export let data

    $: listOfClasses = data.data.classes ?? []

    $: searchText = ''

    $: searchableClasses = isTextInClassList(searchText);

    function isTextInClassList(searchText:string):string[]{
        if (!searchText) return listOfClasses
        if (searchText && listOfClasses){
            return listOfClasses.filter((className:string)=> className.toLowerCase().startsWith(searchText))
        }
        return []
    }

</script>

<section>
    <h1>Test Project</h1>
    
    <div id="main">

        <div>
            <p>Last Modified: {new Date(data.data.lastModified).toLocaleString()}</p>
            <p><span class="bold">Total Trained Classes: </span> {listOfClasses.length}</p>
        </div>
        <div>
            <input bind:value={searchText} />
            <div class="classBox">
                
                {#each searchableClasses as className, ix}
                <p>{ix+1}).{className}</p>
                {/each}
            </div>
        </div>
    </div>
</section>