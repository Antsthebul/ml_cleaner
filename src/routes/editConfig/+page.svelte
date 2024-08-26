<script lang="ts">
    export let data

    let config = data.data
</script>

<div>
    <div class="display-flex">
        <p>Config</p>
        <button class="button button-info">
            Edit Config
        </button>
    </div>
    <pre>{JSON.stringify(config, null, 2)}</pre>
</div>
