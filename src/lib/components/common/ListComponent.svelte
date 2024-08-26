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

    const Component = renderItem;
</script>
<main>

    <section id="list_container">
        {#each items as item}
        <div>
            <Component item={item}/>
             
        </div>
        {/each}
    </section>
</main>