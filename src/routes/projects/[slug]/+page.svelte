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
    export let data


</script>

<section>
    <h1>Test Project</h1>
    
    <div id="main">

        <div><span class="bold">Class: </span> {data.data.length}</div>
        <div>
            <input />
            <div class="classBox">
                
                {#each data.data as className, ix}
                <p>{ix+1}).{className.replace("_", " ")}</p>
                {/each}
            </div>
        </div>
    </div>
</section>