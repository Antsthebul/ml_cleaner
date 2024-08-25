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

    console.log("eh => ", items)
</script>
<main>

    <section id="list_container">
        {#each items as item}
        <div>

            <img src={`data:image/jpeg;base64,${item.b64}`} width="150px" height="150px"/>
            <div class="display-flex justify-center gap-10 mt-10">
                <div>
                    
                    <button class="button button-info">Keep</button>
                </div>
                <div>
                    
                    <button class="button button-danger">Remove</button>
                </div>
            </div>
        </div>
        {/each}
    </section>
</main>