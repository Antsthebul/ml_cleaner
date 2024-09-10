<script lang="ts">
    export let item
    export let handleDelete
    export let handleKeep

</script>

<img src={`data:image/jpeg;base64,${item.b64}`} width="150px" height="150px"/>
<div class="display-flex justify-center gap-10 mt-10">
    <div>
        
        <button class={`button button-info ${item.verified && "button-disabled"}`} on:click={()=>handleKeep(item.file_path)} disabled={item.verified}>Keep</button>
    </div>
    <div>
        
        <button class="button button-danger" on:click={()=>handleDelete(item.file_path)}>Remove</button>
    </div>
</div>