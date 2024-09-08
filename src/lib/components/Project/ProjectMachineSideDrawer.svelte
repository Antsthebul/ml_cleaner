<style>
    #inner-content{
        text-align: center;
    }
    .machineCard{
        border:1px solid lightgrey;
        box-shadow: 1px 1px 5px lightgrey;
        padding:5px 10px;
        border-radius: 15px;
        width:fit-content;
        text-align: left;
    }
</style>
<script lang="ts">
	import { SideDrawer } from "$lib";
	import type { ProjectMachine } from "$lib/global_types";

    export let machines: ProjectMachine[] = []
    export let showSideDrawer = false

</script>

<SideDrawer bind:showSideDrawer={showSideDrawer}>
    <div id="inner-content">

        <h2>Machines</h2>
        {#each machines as machine}
        <div class="machineCard">
            <span class="display-block">
                <a href={`/machines/${machine.name}`}>
                    {machine.name}
                </a>
                - 
            </span>
            <span class="display-block"><b>MachineID:</b> {machine.id}</span>
            <span class="display-block"><b>IP:</b> {machine.ip_addr ?? "-"}</span>
            <span class="display-block"><b>Type:</b> {machine.machine_type}</span>
            <div class="display-flex gap-10 justify-content-center mt-5">
                    <button class="button button-option">Show Runs</button>
                    <button class="button button-danger">Stop</button>
                    <button class="button button-success" on:click={()=>{}}>Train</button>
            </div>

        </div>
        {/each}
    </div>
</SideDrawer>