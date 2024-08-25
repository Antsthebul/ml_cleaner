<script lang="ts">
    export let data

    let config = data.data
</script>

<div>
    <p>Config</p>
    <pre>{JSON.stringify(config, null, 2)}</pre>
</div>
