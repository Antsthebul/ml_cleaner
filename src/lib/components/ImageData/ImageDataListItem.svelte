<script lang="ts">
    export let item
    export let handleDelete

</script>

<img src={`data:image/jpeg;base64,${item.b64}`} width="150px" height="150px"/>
<div class="display-flex justify-center gap-10 mt-10">
    <div>
        
        <button class="button button-info">Keep</button>
    </div>
    <div>
        
        <button class="button button-danger" on:click={()=>handleDelete(item.file_path)}>Remove</button>
    </div>
</div>