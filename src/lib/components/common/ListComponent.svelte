<style>
    #list_container{
        margin-top: 20px;
        display:flex;
        flex-wrap:wrap;
        gap:10px
    }
</style>

<script lang="ts">
    export let items:any =[]
    export let renderItem:any   
    export let handleDelete:(filePath:string)=>void 

    const Component = renderItem;
</script>
<main>

    <section id="list_container">
        {#each items as item}
        <div>
            <Component item={item} handleDelete={handleDelete}/>
             
        </div>
        {/each}
    </section>
</main>