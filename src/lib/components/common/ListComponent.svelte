<style>
    #list_container{
        margin-top: 20px;
        display:flex;
        flex-wrap:wrap;
        gap:10px
    }
</style>

<script lang="ts">
    import LeftArrBold from '~icons/ep/arrow-left-bold';
    import RightArrBold from '~icons/ep/arrow-right-bold';

    export let data:any ={images:[]}
    export let renderItem:any   
    export let handleDelete:(filePath:string)=>void 
    export let handleKeep:(filePath:string)=>void
    export let handlePage:any

    const Component = renderItem;
    $: items = data.images
</script>
<main class="display-flex g-10">
    <div class="w-10">
        {#if data.previous_page}
        <button class="button-less cursor" on:click={()=>handlePage(data.previous_page)}>

            <LeftArrBold />
        </button>
        {/if}
    </div>
    <section id="list_container">
        {#each items as item}
        <div>
            <Component item={item} handleDelete={handleDelete} handleKeep={handleKeep}/>
             
        </div>
        {/each}
    </section>
    <div class="w-10 ">

        {#if data.next_page}
        <button class="button-less cursor" on:click={()=>handlePage(data.next_page)}>
            <RightArrBold />
        </button>
        {/if}
    </div>
</main>