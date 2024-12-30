<script lang="ts">
    import { goto } from "$app/navigation";
    let name = ''
</script>
<button class="button"
    on:click={()=>history.back()}
    >
    Cancel
</button>

<button class="button button-info">
    Ok
</button>
<div>
    <input bind:value={name}>
</div>